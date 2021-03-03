use elite_journal::route::parse_file;

fn main() {
    let route = parse_file("dumps/other/NavRoute.json").unwrap();
    dbg!(route);
}
