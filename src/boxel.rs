//! Where a system sits, as its own address spells it.
//!
//! A `SystemAddress` is not a serial number: it packs the system's place in
//! the galaxy, and the procedurally generated name spells the same place
//! back. `PRAEA EUQ YE-Q D5-0` is the sector `PRAEA EUQ`, then the boxel's
//! ordinal in base 26 (`YE-Q`) and base 10 (`5`), then the system's own
//! index inside that boxel (`0`).
//!
//! ```text
//! [3 mass][7-m boxel z][7 sector z][7-m boxel y][6 sector y][7-m boxel x][7 sector x][rest index]
//! ```
//!
//! `m` is the mass class, `A`..`H`, and a boxel is `10 * 2^m` light years
//! across — so a class `A` boxel is 10 ly and a class `H` one is the whole
//! 1,280 ly sector. The sector is a cube of the galaxy's own grid and the
//! boxel coordinates locate the system inside it.
//!
//! **The ordinal is packed on a stride of 128 an axis** — `x + 128y +
//! 16384z` — whatever the class. That is the one constant here that took
//! measuring to find rather than reading: dividing the sector by the boxel
//! side instead (`1280 / (10 * 2^m)`) agrees with only 6.25 % of real
//! names, which looks like the whole idea is wrong. Established against
//! 200,071,629 names from a Spansh galaxy dump, where the ordinal and the
//! index agree with what the game wrote for **every** system outside
//! Frontier's hand-authored regions.
//!
//! What is *not* here is the sector's name. The procedural ones come from
//! Frontier's own generator and the rest are hand-authored regions laid
//! over the grid as spheres — `COL 285 SECTOR`, `IC 2944 SECTOR` — whose
//! boxels are numbered from the region's own origin rather than the grid's.
//! Naming a sector is therefore a dictionary rather than arithmetic, and
//! a dictionary is data somebody has to have collected: `galos_index`
//! learns one from an imported galaxy and keeps it.

/// The three letters of a boxel code, in base 26, before the run counts.
const LETTERS: u32 = 26 * 26 * 26;

/// How many boxels a sector's code numbers along one axis.
///
/// The stride the ordinal is packed on, and it is 128 at every mass class:
/// the class changes how wide a boxel is, not how the ordinal is packed.
const STRIDE: u32 = 128;

/// Where a system sits, as its own address spells it.
///
/// See the [module docs](self) for the layout and for what the numbers
/// mean.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boxel {
    /// The mass class, 0 for `A` through 7 for `H`.
    pub mass: u8,
    /// The sector's coordinates on the galaxy's 1,280 ly grid.
    pub sector: [u8; 3],
    /// The boxel's ordinal inside the sector, `x + 128y + 16384z`.
    pub ordinal: u32,
    /// The system's own index inside the boxel.
    pub index: u32,
}

impl Boxel {
    /// What `address` says about where the system is.
    ///
    /// Total, because every bit pattern is some boxel: an address this
    /// cannot read does not exist. Whether anything has *named* that boxel
    /// is a question for whoever holds a sector dictionary.
    pub fn of(address: i64) -> Boxel {
        let held = address as u64;
        let mass = (held & 7) as u8;
        let bits = 7 - u32::from(mass);
        let mask = (1u64 << bits) - 1;
        let z = ((held >> 3) & mask) as u32;
        let sz = ((held >> (3 + bits)) & 0x7F) as u8;
        let y = ((held >> (10 + bits)) & mask) as u32;
        let sy = ((held >> (10 + 2 * bits)) & 0x3F) as u8;
        let x = ((held >> (16 + 2 * bits)) & mask) as u32;
        let sx = ((held >> (16 + 3 * bits)) & 0x7F) as u8;
        Boxel {
            mass,
            sector: [sx, sy, sz],
            ordinal: x + STRIDE * y + STRIDE * STRIDE * z,
            index: (held >> (23 + 3 * bits)) as u32,
        }
    }

    /// The address this boxel is, or [`None`] where it is not one.
    ///
    /// The fields are narrower than the types that hold them — a class `H`
    /// boxel has no coordinate bits at all — so a boxel assembled from a
    /// parsed name has to be checked before it is an address. That is what
    /// makes resolving a name to an address safe: a name spelling
    /// coordinates no address can hold answers nothing rather than
    /// somebody else's system.
    pub fn address(&self) -> Option<i64> {
        if self.mass > 7 {
            return None;
        }
        let bits = 7 - u32::from(self.mass);
        let mask = (1u32 << bits) - 1;
        let [x, y, z] = self.coordinates();
        if x > mask || y > mask || z > mask {
            return None;
        }
        if self.sector[0] > 0x7F
            || self.sector[1] > 0x3F
            || self.sector[2] > 0x7F
        {
            return None;
        }
        let shift = 23 + 3 * bits;
        if shift < 64 && self.index >= (1u32 << (64 - shift).min(31)) {
            return None;
        }
        let held = u64::from(self.mass)
            | (u64::from(z) << 3)
            | (u64::from(self.sector[2]) << (3 + bits))
            | (u64::from(y) << (10 + bits))
            | (u64::from(self.sector[1]) << (10 + 2 * bits))
            | (u64::from(x) << (16 + 2 * bits))
            | (u64::from(self.sector[0]) << (16 + 3 * bits))
            | (u64::from(self.index) << shift);
        Some(held as i64)
    }

    /// The boxel's coordinates inside its sector.
    pub fn coordinates(&self) -> [u32; 3] {
        [
            self.ordinal % STRIDE,
            (self.ordinal / STRIDE) % STRIDE,
            self.ordinal / (STRIDE * STRIDE),
        ]
    }

    /// How the boxel and the system's index are spelled, after the sector.
    ///
    /// `YE-Q D5-0`, and `AA-A H0` where the run is nought — which the game
    /// writes by leaving it out rather than as a zero.
    pub fn tail(&self) -> String {
        let letter = |n: u32| char::from(b'A' + n as u8);
        let code = self.ordinal % LETTERS;
        let run = self.ordinal / LETTERS;
        let class = letter(u32::from(self.mass));
        let (one, two, three) =
            (letter(code % 26), letter((code / 26) % 26), letter(code / 676));
        let index = self.index;
        if run == 0 {
            format!("{one}{two}-{three} {class}{index}")
        } else {
            format!("{one}{two}-{three} {class}{run}-{index}")
        }
    }

    /// The boxel a name's `tail` spells, in the `sector` the caller placed.
    ///
    /// `tail` is what [`Self::tail`] writes — the code and the class, with
    /// no sector words in front of it — and `sector` is the coordinates of
    /// whatever named those words, which only a dictionary can say.
    ///
    /// [`None`] where the tail is not one: a letter that is not a letter, a
    /// run or an index that is not a number, a mass class past `H`, or an
    /// ordinal that overflows. The result is still only a *claim* until
    /// [`Self::address`] accepts it.
    pub fn spelled(tail: &str, sector: [u8; 3]) -> Option<Boxel> {
        let (code, last) = tail.split_once(' ')?;
        let (pair, one) = code.split_once('-')?;
        let (pair, one) = (pair.as_bytes(), one.as_bytes());
        if pair.len() != 2 || one.len() != 1 {
            return None;
        }
        let letter =
            |b: u8| b.is_ascii_uppercase().then(|| u32::from(b - b'A'));
        let code =
            letter(pair[0])? + 26 * letter(pair[1])? + 676 * letter(one[0])?;

        let last = last.as_bytes();
        let (class, numbers) = last.split_first()?;
        let mass = letter(*class)?;
        if mass > 7 {
            return None;
        }
        let numbers = std::str::from_utf8(numbers).ok()?;
        // `D5-0` is run 5 and index 0; `H1` is run nought and index 1.
        let (run, index) = match numbers.split_once('-') {
            Some((run, index)) => {
                (run.parse::<u32>().ok()?, index.parse::<u32>().ok()?)
            }
            None => (0, numbers.parse::<u32>().ok()?),
        };
        Some(Boxel {
            mass: mass as u8,
            sector,
            ordinal: code.checked_add(LETTERS.checked_mul(run)?)?,
            index,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real systems, unpacked and spelled back
    ///
    /// The pairs are off a Spansh galaxy dump, one per shape: a run left
    /// unwritten, and a run written with its dash.
    #[test]
    fn an_address_spells_the_tail_the_game_wrote() {
        for (address, tail) in [
            (96_076_086i64, "AA-A G1"),
            (1_038_034_644, "NR-W E1-0"),
            (1_189_102_764, "TO-R E4-0"),
        ] {
            let boxel = Boxel::of(address);
            assert_eq!(boxel.tail(), tail, "{address}");
            assert_eq!(
                Boxel::spelled(tail, boxel.sector),
                Some(boxel),
                "{tail}",
            );
            assert_eq!(boxel.address(), Some(address), "{tail}");
        }
    }

    /// A boxel survives the round trip at every mass class
    ///
    /// The widest coordinates each class can hold, which is where a layout
    /// that shifted by the wrong width would fall over.
    #[test]
    fn a_boxel_survives_the_round_trip() {
        for mass in 0..8u8 {
            let most = (1u32 << (7 - u32::from(mass))) - 1;
            let boxel = Boxel {
                mass,
                sector: [39, 32, 18],
                ordinal: most + STRIDE * most + STRIDE * STRIDE * most,
                index: 3,
            };
            let address = boxel.address().expect("an address");
            assert_eq!(Boxel::of(address), boxel, "class {mass}");
            assert_eq!(boxel.coordinates(), [most, most, most]);
        }
    }

    /// A run of nought is written by leaving it out
    #[test]
    fn a_run_is_written_only_when_there_is_one() {
        let held = Boxel { mass: 3, sector: [1, 2, 3], ordinal: 5, index: 7 };
        assert_eq!(held.tail(), "FA-A D7");
        let run = Boxel { ordinal: LETTERS + 5, ..held };
        assert_eq!(run.tail(), "FA-A D1-7");
        for boxel in [held, run] {
            assert_eq!(
                Boxel::spelled(&boxel.tail(), boxel.sector),
                Some(boxel)
            );
        }
    }

    /// A tail that is not one spells no boxel
    #[test]
    fn a_tail_that_is_not_one_spells_nothing() {
        for tail in [
            "YE-Q",               // no class at all
            "YE-Q Z5-0",          // a mass class past H
            "YE-Q D5-",           // an index that is not a number
            "Y-Q D5-0",           // two letters, not three
            "ye-q d5-0",          // lower case, which the game never writes
            "YE-Q D4294967295-0", // an ordinal that overflows
        ] {
            assert_eq!(Boxel::spelled(tail, [0, 0, 0]), None, "{tail}");
        }
    }

    /// A boxel no address can hold is refused rather than packed wrongly
    ///
    /// A class `H` boxel has no coordinate bits, so anything but the
    /// sector's own corner is not an address; nor is a sector coordinate
    /// past the galaxy's grid.
    #[test]
    fn a_boxel_outside_the_layout_is_no_address() {
        let outside =
            Boxel { mass: 7, sector: [0, 0, 0], ordinal: 1, index: 0 };
        assert_eq!(outside.address(), None);
        let far = Boxel { mass: 0, sector: [0, 64, 0], ordinal: 0, index: 0 };
        assert_eq!(far.address(), None);
    }
}
