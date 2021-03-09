use elite_journal::entry::{parse_status_file, Entry, Route};

fn main() {
    let route: Entry<Route> = parse_status_file("dumps/other/NavRoute.json").unwrap();
    dbg!(route);
}
