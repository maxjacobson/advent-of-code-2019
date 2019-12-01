// Fuel required to launch a given module is based on its mass. Specifically, to find the fuel
// required for a module, take its mass, divide by three, round down, and subtract 2.
fn mass_to_fuel_requirement(mass: f32) -> i32 {
    (mass / 3.0).floor() as i32 - 2
}

fn main() {
    let result = include_str!("../input.txt").lines().fold(0, |acc, line| {
        acc + mass_to_fuel_requirement(line.parse().unwrap())
    });

    println!("The result is: {}", result);
}
