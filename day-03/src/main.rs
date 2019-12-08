use std::collections::HashMap;

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

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_by_amount(&self, movement: &Movement) -> Vec<Self> {
        match movement {
            Movement::Right(amount) => self.right(amount),
            Movement::Left(amount) => self.left(*amount),
            Movement::Up(amount) => self.up(*amount),
            Movement::Down(amount) => self.down(*amount),
        }
    }

    fn right(&self, amount: &i32) -> Vec<Self> {
        (1..=*amount)
            .map(|n| Self {
                x: self.x + n,
                y: self.y,
            })
            .collect()
    }

    fn left(&self, amount: i32) -> Vec<Self> {
        (1..=amount)
            .map(|n| Self {
                x: self.x - n,
                y: self.y,
            })
            .collect()
    }

    fn up(&self, amount: i32) -> Vec<Self> {
        (1..=amount)
            .map(|n| Self {
                x: self.x,
                y: self.y + n,
            })
            .collect()
    }

    fn down(&self, amount: i32) -> Vec<Self> {
        (1..=amount)
            .map(|n| Self {
                x: self.x,
                y: self.y - n,
            })
            .collect()
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

    fn points(&self) -> Vec<(Point, i32)> {
        let mut points = vec![];
        let mut previous_point: Point = Point { x: 0, y: 0 };
        points.push((previous_point, 0));
        let mut steps = 0;

        for movement in &self.movements {
            for point in previous_point.move_by_amount(movement) {
                steps += 1;
                points.push((point, steps));
                previous_point = point;
            }
        }

        points
    }
}

fn manhattan_distance(point: &Point) -> i32 {
    point.x.abs() + point.y.abs()
}

fn main() {
    let paths: Vec<Path> = include_str!("../input.txt")
        .lines()
        .map(|line| Path::from_line(&line))
        .collect();

    let path_a = &paths[0];
    let path_b = &paths[1];

    let mut points_carer: HashMap<Point, (Option<i32>, Option<i32>)> = HashMap::new();

    for (point, steps) in path_a.points() {
        let entry = points_carer.entry(point).or_insert((None, None));
        match entry.0 {
            None => entry.0 = Some(steps),
            _ => {}
        }
    }

    for (point, steps) in path_b.points() {
        let entry = points_carer.entry(point).or_insert((None, None));
        match entry.1 {
            None => entry.1 = Some(steps),
            _ => {}
        }
    }

    let mut manhattan_distances: Vec<i32> = points_carer
        .iter()
        .filter(|(_point, steps)| steps.0.is_some() && steps.1.is_some())
        .map(|(point, _steps)| manhattan_distance(point))
        .filter(|distance| distance != &0)
        .collect();

    manhattan_distances.sort();

    println!("Star 1: {:?}", manhattan_distances.get(0));

    let mut step_distances: Vec<i32> = points_carer
        .iter()
        .filter(|(_point, steps)| steps.0.is_some() && steps.1.is_some())
        .map(|(_point, steps)| steps.0.unwrap() + steps.1.unwrap())
        .filter(|distance| distance != &0)
        .collect();

    step_distances.sort();

    println!("Star 2: {:?}", step_distances.get(0));
}
