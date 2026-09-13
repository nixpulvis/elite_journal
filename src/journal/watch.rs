//! Reading the directory over and over, on a thread of its own.

use super::{Journal, Read};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::{debug, info, warn};

/// How often a watch goes back to the directory, unless told otherwise.
///
/// A second, which is the beat the game writes at: an arrival is one line and
/// a full system scan is a few dozen, and the map's own refresh poll is of the
/// same order. Faster buys nothing a player can see; slower is a jump that
/// shows up late on the map they are flying by.
///
/// Under the `watch` feature this is the longest the beat waits rather than
/// the length of it; see [`Journal::watch`].
pub const EVERY: Duration = Duration::from_secs(1);

/// The longest a stopping watch goes on sleeping before it notices.
///
/// The beat is slept in slices of this, so dropping a [`Watch`] — which joins
/// — costs at most one of them rather than one whole beat. A window closing
/// is the case: it joins this thread, and a beat set to a minute would
/// otherwise hold the window open for most of one.
const WAKE: Duration = Duration::from_millis(100);

/// A running [`Journal::watch`], stopped by dropping it.
///
/// Dropping joins, so a client that lets go of the watch is not left with a
/// thread reading a directory nobody is drawing.
pub struct Watch {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Watch {
    /// Ask the watch to stop, without waiting for it.
    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        self.stop();
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// Poll `journal` on the beat and hand each reading to `on`, until the
/// returned [`Watch`] is stopped or dropped.
///
/// A thread and not a task: this crate has no runtime and the client that
/// holds it may have any. The reads are blocking, short and rare — one
/// `stat` per journal file per beat, and bytes only where the game wrote
/// some — so a thread asleep between them costs a stack.
pub(super) fn spawn<F>(journal: Journal, every: Duration, mut on: F) -> Watch
where
    F: FnMut(Read) + Send + 'static,
{
    let stop = Arc::new(AtomicBool::new(false));
    let stopping = stop.clone();
    let dir = journal.dir().to_owned();
    let handle = thread::Builder::new()
        .name("elite-journal".into())
        .spawn(move || {
            info!(dir = %dir.display(), "following the journal");
            // The watch is registered before the first read, so a line
            // written between the two is woken for rather than waiting out
            // the beat.
            let beat = Beat::over(&dir);
            while !stopping.load(Ordering::Relaxed) {
                match journal.poll() {
                    Ok(read) => on(read),
                    Err(err) => warn!(
                        dir = %dir.display(),
                        error = %err,
                        "the journal could not be read",
                    ),
                }
                beat.wait(every, &stopping);
            }
            debug!(dir = %dir.display(), "the journal watch stopped");
        })
        .expect("a thread to follow the journal on");
    Watch { stop, handle: Some(handle) }
}

/// What a watch waits on between one reading and the next.
enum Beat {
    /// Slept through, which is what a build without the `watch` feature
    /// gets and what a build with it falls back to over a directory the
    /// filesystem would not hand out notifications for.
    Slept,
    /// Woken by the filesystem, with the beat as the longest it waits.
    #[cfg(feature = "watch")]
    Woken {
        events: std::sync::mpsc::Receiver<()>,
        /// Held to be dropped with the beat: dropping the watcher is what
        /// unregisters the directory, and one dropped at the end of
        /// [`Beat::over`] would leave a channel nothing ever sends on.
        _watcher: notify::RecommendedWatcher,
    },
}

impl Beat {
    /// A beat over `dir`.
    ///
    /// Watching it where the feature is on and the directory is there to
    /// watch: a journal directory is named by a commander and may not exist
    /// yet, and one that appears later is found by the sleeping beat.
    #[cfg(not(feature = "watch"))]
    fn over(_dir: &Path) -> Beat {
        Beat::Slept
    }

    /// A beat over `dir`.
    ///
    /// Watching it where the feature is on and the directory is there to
    /// watch: a journal directory is named by a commander and may not exist
    /// yet, and one that appears later is found by the sleeping beat.
    #[cfg(feature = "watch")]
    fn over(dir: &Path) -> Beat {
        use notify::{RecursiveMode, Watcher};

        let (tx, rx) = std::sync::mpsc::channel();
        let watcher = notify::recommended_watcher(move |_| {
            // The event itself says nothing this needs. Which file moved and
            // how is exactly what a poll works out from the offsets it holds,
            // and a notification the channel dropped is a beat that ends on
            // its timeout instead.
            let _ = tx.send(());
        })
        .and_then(|mut watcher| {
            watcher.watch(dir, RecursiveMode::NonRecursive)?;
            Ok(watcher)
        });
        match watcher {
            Ok(watcher) => Beat::Woken { events: rx, _watcher: watcher },
            Err(err) => {
                warn!(
                    dir = %dir.display(),
                    error = %err,
                    "the journal will be read on a timer, not watched",
                );
                Beat::Slept
            }
        }
    }

    /// Wait for the directory to have something new in it, giving up after
    /// `every` and as soon as `stopping` is set.
    ///
    /// Slept in slices of [`WAKE`] either way, so a stopping watch is joined
    /// within one slice rather than one beat.
    fn wait(&self, every: Duration, stopping: &AtomicBool) {
        match self {
            Beat::Slept => sleep(every, stopping),
            #[cfg(feature = "watch")]
            Beat::Woken { events, .. } => {
                use std::sync::mpsc::RecvTimeoutError;

                let mut left = every;
                while left > Duration::ZERO
                    && !stopping.load(Ordering::Relaxed)
                {
                    let slice = left.min(WAKE);
                    match events.recv_timeout(slice) {
                        Ok(()) => {
                            // Let it settle before draining. The game writes
                            // a system scan as a few dozen lines in a few
                            // milliseconds, and one change to the directory
                            // should be one pass over it.
                            thread::sleep(WAKE);
                            while events.try_recv().is_ok() {}
                            return;
                        }
                        Err(RecvTimeoutError::Timeout) => left -= slice,
                        // The watcher is gone, which leaves the timer as the
                        // whole of the beat.
                        Err(RecvTimeoutError::Disconnected) => {
                            sleep(left, stopping);
                            return;
                        }
                    }
                }
            }
        }
    }
}

/// Sleep `every` in slices, stopping early where `stopping` is set.
///
/// In slices, so that dropping the watch is the length of one slice and not
/// of the beat. A map closing its window joins that thread, and a beat set
/// to a minute would hold the window open for most of one.
fn sleep(every: Duration, stopping: &AtomicBool) {
    let mut left = every;
    while left > Duration::ZERO && !stopping.load(Ordering::Relaxed) {
        let slice = left.min(WAKE);
        thread::sleep(slice);
        left -= slice;
    }
}
