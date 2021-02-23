use elite_journal::{parse_dir, travel, Event};

fn main() {
    let entries = parse_dir("dumps/").unwrap();
    dbg!(entries.len());
    for entry in entries {
        // dbg!(entry.timestamp);
        match entry.event {
            // Event::Fileheader(header) => { dbg!(header); },
            // Event::Cargo(manifest) => { dbg!(manifest); },
            // Event::Commander(cmdr) => { dbg!(cmdr); },
            // Event::LoadGame(lg) => { dbg!(lg); },
            Event::FsdJump(fsdj) => { dbg!(fsdj); }
            Event::Location(location) => { dbg!(location); }
            _ => {},
        }
    }
}
