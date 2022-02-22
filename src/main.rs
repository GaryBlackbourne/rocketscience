mod ship;

fn main() {
    let rocket = ship::Ship {
        name: String::from("test"),
    };

    println!("{}", rocket.name);
}
