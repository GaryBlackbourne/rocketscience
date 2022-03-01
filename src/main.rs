mod models;

fn main() {
    let rocket = models::Ship::new("Test");

    println!("{}", rocket.name);
}
