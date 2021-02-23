use elite_journal::{parse_dir, travel, Event};

fn main() {
    let entries = parse_dir("dumps/").unwrap();
    dbg!(entries.len());
    for entry in entries {
        // dbg!(entry.timestamp);
        match entry.event {
            // Event::Fileheader { part, .. } => { dbg!(part); },
            // Event::Cargo { vessel, inventory } => { dbg!(vessel, inventory); },
            // Event::Commander(cmdr) => { dbg!(cmdr); },
            // lg @ Event::LoadGame { .. } => { dbg!(lg); },
            Event::FsdJump(travel::FsdJump { star_system, factions, star_pos, .. }) => {
                // for faction in factions {
                //     println!("{}: {:?}", star_system, faction.active_states);
                // }

                println!("{}: {:?}", star_system, star_pos);
            }
            _ => {},
        }
    }
}
