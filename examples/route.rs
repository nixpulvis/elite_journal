use elite_journal::entry::{parse_status_file, Entry, NavRoute};

fn main() {
    let route: Entry<NavRoute> =
        parse_status_file("dumps/other/NavRoute.json").unwrap();
    dbg!(route);
}
