use std::collections::hash_set::HashSet;

#[derive(Debug)]
enum Movement {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

impl Movement {
    fn from_str(s: &str) -> Self {
        let dir = s.chars().nth(0);
        let amount: i32 = s
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("could not extract number from direction");

        match dir {
            Some('R') => Movement::Right(amount),
            Some('D') => Movement::Down(amount),
            Some('L') => Movement::Left(amount),
            Some('U') => Movement::Up(amount),
            _ => panic!("Unknown direction"),
        }
    }
}

type Point = (i32, i32);

#[derive(Debug)]
struct Path {
    movements: Vec<Movement>,
}

impl Path {
    fn from_line(line: &str) -> Self {
        Self {
            movements: line.split(",").map(|s| Movement::from_str(&s)).collect(),
        }
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        let mut previous_movement: Point = (0, 0);
        points.insert(previous_movement);

        for movement in &self.movements {
            let next_movement = match movement {
                Movement::Right(amount) => (previous_movement.0 + amount, previous_movement.1),
                Movement::Left(amount) => (previous_movement.0 - amount, previous_movement.1),
                Movement::Up(amount) => (previous_movement.0, previous_movement.1 + amount),
                Movement::Down(amount) => (previous_movement.0, previous_movement.1 - amount),
            };

            for point in self.points_between(previous_movement, next_movement) {
                points.insert(point);
            }

            points.insert(next_movement);
            previous_movement = next_movement;
        }

        points
    }

    fn points_between(&self, a: Point, b: Point) -> Vec<Point> {
        if b.0 > a.0 {
            (a.0..b.0).map(|x| (x, b.1)).collect()
        } else if b.0 < a.0 {
            (b.0..a.0).map(|x| (x, b.1)).collect()
        } else if b.1 > a.1 {
            (a.1..b.1).map(|y| (b.0, y)).collect()
        } else if b.1 < a.1 {
            (b.1..a.1).map(|y| (b.0, y)).collect()
        } else {
            panic!("Huh?");
        }
    }

    fn intersections_with(&self, other: &Path) -> Vec<Point> {
        let my_points = self.points();
        let other_points = other.points();

        let intersections = my_points.intersection(&other_points);

        intersections.map(|point| *point).collect()
    }
}

fn distance(point: &Point) -> i32 {
    point.0.abs() + point.1.abs()
}

fn main() {
    let paths: Vec<Path> = include_str!("../input.txt")
        .lines()
        .map(|line| Path::from_line(&line))
        .collect();

    let path_a = &paths[0];
    let path_b = &paths[1];

    let mut distances: Vec<i32> = path_a
        .intersections_with(path_b)
        .iter()
        .map(|point| distance(point))
        .collect();
    distances.sort();

    println!("distance: {:?}", distances.iter().nth(1).unwrap());
}
