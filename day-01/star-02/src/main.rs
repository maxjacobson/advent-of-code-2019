fn mass_to_fuel_requirement(mass: f32) -> i32 {
    let fuel = (mass / 3.0).floor() as i32 - 2;

    if fuel <= 0 {
        return 0;
    }

    fuel + mass_to_fuel_requirement(fuel as f32)
}

fn main() {
    let result = include_str!("../input.txt").lines().fold(0, |acc, line| {
        acc + mass_to_fuel_requirement(line.parse().unwrap())
    });

    println!("The result is: {}", result);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_mass_to_fuel_requirement() {
        assert_eq!(mass_to_fuel_requirement(14.0), 2);
        assert_eq!(mass_to_fuel_requirement(1969.0), 966);
        assert_eq!(mass_to_fuel_requirement(100756.0), 50346);
    }
}
