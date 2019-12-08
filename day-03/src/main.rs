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

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        let mut previous_point: Point = Point { x: 0, y: 0 };
        points.push(previous_point);

        for movement in &self.movements {
            for point in previous_point.move_by_amount(movement) {
                points.push(point);
                previous_point = point;
            }
        }

        points
    }

    fn intersections_with(&self, other: &Path) -> Vec<Point> {
        let my_points = self.points();
        let other_points = other.points();

        intersections(&my_points, &other_points)
    }
}

fn intersections(a: &Vec<Point>, b: &Vec<Point>) -> Vec<Point> {
    a.iter()
        .filter(|point| b.contains(point))
        .copied()
        .collect()
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
        .filter(|distance| distance != &0)
        .collect();

    distances.sort();

    println!("distance: {:?}", distances.get(0));
}
