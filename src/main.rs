
fn main() {
    let corvette:(i32, &str) = (755, "Horse Power");
    let car = print_labeled_measurement(corvette.0, corvette.1);
    println!("{car}");
}

fn print_labeled_measurement(value:i32, _unit:&str) -> &str {
    if value == 755 {
        "it's a zr1"
    }
    else {
        "it's a shitty car"
    }
}