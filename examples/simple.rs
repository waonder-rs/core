extern crate wonder;

fn main() {
    let mut planet = wonder::Planet::new();

    planet.refine();
    planet.refine();
    planet.refine();
    planet.refine();
    planet.refine();
    planet.print_obj();
}
