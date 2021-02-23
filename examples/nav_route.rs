use elite_journal::nav_route;

fn main() {
    let route = nav_route::parse_file("dumps/other/NavRoute.json").unwrap();
    dbg!(route);
}
