use elite_journal::incremental::{parse_dir, Event};

fn main() {
    let entries = parse_dir("dumps/").unwrap();

    // Print all FSDJumps to Sol.
    for entry in entries.iter() {
        if let Event::FsdJump(event) = &entry.event {
            if event.system.name == "Sol" {
                println!("{}", entry.timestamp);
                println!("{:#?}", event);
            }
        }
    }

    println!("total log count: {}.", entries.len());
}
