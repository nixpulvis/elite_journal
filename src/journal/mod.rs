//! The game's own journal directory, read while it is being written.
//!
//! [`Journal`] is a directory of `.log` files and how far into each of them
//! has been read. [`Journal::poll`] hands back everything written since the
//! last one, which on the first call is the whole directory — a commander's
//! entire history — and on every call after it is the tail. The rules that
//! makes necessary, and there are more of them than `tail -f` suggests, are
//! [`follow`]'s.
//!
//! [`Journal::watch`] is that on a beat, on a thread of its own, handing each
//! reading to a closure. What the beat waits on depends on how the crate was
//! built:
//!
//! - **With the `watch` feature**, on the filesystem: inotify, kqueue or
//!   FSEvents, or `ReadDirectoryChangesW`. A jump shows up as soon as the
//!   game has written the line, and the beat is only the longest the watch
//!   will wait without being told anything.
//! - **Without it**, on the clock. The directory is read every beat whether
//!   anything moved or not, which costs one `stat` per log file and finds a
//!   jump up to one beat late.
//!
//! Both are the same [`Journal`], the same [`Read`] and the same [`Watch`];
//! the feature buys latency, not capability. Nothing here judges what an
//! event means.

pub mod follow;
mod watch;

pub use self::follow::{Follower, Read};
pub use self::watch::{Watch, EVERY};

use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::Duration;

/// A journal directory, read once or followed.
///
/// Cloneable: a clone shares the one reading, so a watch thread and the
/// client that started it are never at different offsets in the same file.
/// Nothing is read until [`Self::poll`] is called or a [`Watch`] is started,
/// so constructing one cannot fail and a directory that is not there is not
/// an error until somebody looks.
#[derive(Clone, Debug)]
pub struct Journal {
    dir: PathBuf,
    follower: Arc<RwLock<Follower>>,
}

impl Journal {
    /// A journal over the directory at `dir`, having read none of it.
    pub fn new(dir: impl Into<PathBuf>) -> Journal {
        let dir = dir.into();
        Journal { follower: Arc::new(RwLock::new(Follower::new(&dir))), dir }
    }

    /// The directory being read.
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    /// Everything written to the directory since the last poll.
    ///
    /// The first poll reads the lot, which is the import.
    pub fn poll(&self) -> io::Result<Read> {
        self.follower().poll()
    }

    /// Take the directory as already read, without reading any of it.
    ///
    /// For a caller importing the whole directory some other way and
    /// following it afterwards. Call it **before** that import, not after:
    /// see [`Follower::caught_up`], which this is.
    ///
    /// Answers how many bytes were passed over.
    pub fn caught_up(&self) -> io::Result<u64> {
        self.follower().caught_up()
    }

    /// Read the directory on a beat, on a thread of its own, handing each
    /// reading to `on`.
    ///
    /// `every` is how long the beat waits; under the `watch` feature it is
    /// the *longest* it waits, and a directory that moves is read at once.
    /// [`EVERY`] is the beat to use without a reason for another.
    ///
    /// Errors reading the directory are warned and the beat goes on: a
    /// journal directory that has gone away may come back, and a watch that
    /// died with it would leave the client with nothing and no way to know.
    /// So `on` sees readings only.
    ///
    /// Stopped by dropping the [`Watch`], which joins the thread.
    pub fn watch<F>(&self, every: Duration, on: F) -> Watch
    where
        F: FnMut(Read) + Send + 'static,
    {
        self::watch::spawn(self.clone(), every, on)
    }

    /// The reading. Poisoning says a caller panicked mid-poll, which says
    /// nothing about whether this one may look.
    fn follower(&self) -> std::sync::RwLockWriteGuard<'_, Follower> {
        self.follower.write().unwrap_or_else(|it| it.into_inner())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    /// A journal directory holding one log of `lines`.
    fn journal(name: &str, lines: &[String]) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "elite_journal_watch_{}_{}",
            name,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("a scratch directory");
        std::fs::write(
            dir.join("Journal.2026-08-08T120000.01.log"),
            lines.join("\n") + "\n",
        )
        .expect("a journal log");
        dir
    }

    fn jump(system: &str, address: i64) -> String {
        format!(
            r#"{{"timestamp":"2026-08-08T12:00:00Z","event":"FSDJump","StarSystem":"{}","SystemAddress":{},"StarPos":[0.0,0.0,0.0]}}"#,
            system, address,
        )
    }

    /// A watch hands over what the game writes and stops when it is dropped
    ///
    /// The beat is whichever one the crate was built with, so this holds for
    /// a timer and for a filesystem watch alike: what a client is promised is
    /// that a line appended to a log arrives without being asked for, and
    /// that letting go of the watch ends the thread.
    #[test]
    fn a_watch_reads_what_arrives_and_stops_when_dropped() {
        let dir = journal("arrives", &[jump("Sol", 10477373803)]);
        let (tx, rx) = mpsc::channel();
        let journal = Journal::new(&dir);
        let watch = journal.watch(Duration::from_millis(50), move |read| {
            for entry in read.entries {
                let _ = tx.send(entry);
            }
        });

        // The first poll is the import, so the log that was already there
        // arrives without anything being written. A minute is a ceiling for
        // a loaded machine, not an expectation: what is tested is that the
        // reading arrives, and a watcher that never fires still fails.
        rx.recv_timeout(Duration::from_secs(60)).expect("the log was read");

        let path = dir.join("Journal.2026-08-08T120000.02.log");
        std::fs::write(&path, jump("Alpha Centauri", 22) + "\n")
            .expect("a second session");
        rx.recv_timeout(Duration::from_secs(60))
            .expect("the new session was read");

        // Whatever the beat had already queued, so what is asserted below
        // is the watch being over rather than the channel being behind.
        while rx.try_recv().is_ok() {}

        drop(watch);
        // The sender lived in the watch's closure, so a channel that is
        // hung up is a thread that has been joined.
        assert!(
            rx.recv_timeout(Duration::from_millis(500)).is_err(),
            "the watch went on reading after it was dropped",
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// A poll through a clone moves the offsets the original reads by
    ///
    /// The watch thread holds a clone and the client holds the original.
    /// Two readings of the same file would have every entry applied twice,
    /// which downstream survives, and every entry *counted* twice, which is
    /// what a client watching the counts is looking at.
    #[test]
    fn a_clone_shares_the_reading() {
        let dir = journal("shared", &[jump("Sol", 10477373803)]);
        let journal = Journal::new(&dir);
        let clone = journal.clone();

        assert_eq!(journal.poll().expect("a poll").entries.len(), 1);
        assert_eq!(
            clone.poll().expect("a poll through the clone").entries.len(),
            0,
            "the clone read the log a second time",
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    /// Under `watch` the beat is a ceiling, not the wait
    ///
    /// The whole of what the feature buys. A beat of ten minutes with a
    /// line arriving inside one of them is a reading the filesystem asked
    /// for; without the feature the same journal would sit unread for the
    /// rest of the beat. The margin is wide on purpose: a minute against
    /// ten proves the point on a machine too busy to answer in a second.
    #[cfg(feature = "watch")]
    #[test]
    fn a_watched_beat_does_not_wait_it_out() {
        let dir = journal("woken", &[jump("Sol", 10477373803)]);
        let (tx, rx) = mpsc::channel();
        let journal = Journal::new(&dir);
        let _watch = journal.watch(Duration::from_secs(600), move |read| {
            for entry in read.entries {
                let _ = tx.send(entry);
            }
        });

        // The import, which happens before the beat is ever waited on.
        rx.recv_timeout(Duration::from_secs(60)).expect("the log was read");

        std::fs::write(
            dir.join("Journal.2026-08-08T120000.02.log"),
            jump("Alpha Centauri", 22) + "\n",
        )
        .expect("a second session");
        rx.recv_timeout(Duration::from_secs(60))
            .expect("the beat waited out its ceiling");

        let _ = std::fs::remove_dir_all(&dir);
    }
}
