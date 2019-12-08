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

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_by_amount(&self, movement: &Movement) -> Self {
        match movement {
            Movement::Right(amount) => self.right(amount),
            Movement::Left(amount) => self.left(*amount),
            Movement::Up(amount) => self.up(*amount),
            Movement::Down(amount) => self.down(*amount),
        }
    }

    fn right(&self, amount: &i32) -> Self {
        Self {
            x: self.x + amount,
            y: self.y,
        }
    }

    fn left(&self, amount: i32) -> Self {
        Self {
            x: self.x - amount,
            y: self.y,
        }
    }

    fn up(&self, amount: i32) -> Self {
        Self {
            x: self.x,
            y: self.y + amount,
        }
    }

    fn down(&self, amount: i32) -> Self {
        Self {
            x: self.x,
            y: self.y - amount,
        }
    }
}

#[derive(Debug)]
struct Path {
    movements: Vec<Movement>,
}

impl Path {
    fn from_line(line: &str) -> Self {
        Self {
            movements: line.split(',').map(|s| Movement::from_str(&s)).collect(),
        }
    }

    fn points(&self) -> HashSet<Point> {
        let mut points = HashSet::new();
        let mut previous_movement: Point = Point { x: 0, y: 0 };
        points.insert(previous_movement);

        for movement in &self.movements {
            let next_movement: Point = previous_movement.move_by_amount(movement);

            for point in self.points_between(previous_movement, next_movement) {
                points.insert(point);
            }

            points.insert(next_movement);
            previous_movement = next_movement;
        }

        points
    }

    fn points_between(&self, a: Point, b: Point) -> Vec<Point> {
        if b.x > a.x {
            (a.x..b.x).map(|x| Point { x: x, y: b.y }).collect()
        } else if b.x < a.x {
            (b.x..a.x).map(|x| Point { x: x, y: b.y }).collect()
        } else if b.y > a.y {
            (a.y..b.y).map(|y| Point { x: b.x, y: y }).collect()
        } else if b.y < a.y {
            (b.y..a.y).map(|y| Point { x: b.x, y: y }).collect()
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
    point.x.abs() + point.y.abs()
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

    println!("distance: {:?}", distances.get(1).unwrap());
}
