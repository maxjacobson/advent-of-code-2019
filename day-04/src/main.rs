struct Password {
    letters: Vec<char>,
}

impl Password {
    fn new(password: &str) -> Self {
        Self {
            letters: password.chars().collect(),
        }
    }

    fn valid(&self) -> bool {
        self.six_digits() && self.two_adjacent_digits_are_the_same() && self.digits_dont_decrease()
    }

    fn stricter_valid(&self) -> bool {
        self.six_digits()
            && self.digits_dont_decrease()
            && self.exactly_two_adjacent_digits_are_the_same()
    }

    fn six_digits(&self) -> bool {
        self.letters.len() == 6
    }

    fn two_adjacent_digits_are_the_same(&self) -> bool {
        self.letters
            .iter()
            .zip(self.letters.iter().skip(1))
            .any(|(a, b)| a == b)
    }

    fn exactly_two_adjacent_digits_are_the_same(&self) -> bool {
        let mut iter = self.letters.iter().peekable();
        let mut counter = 1;

        while let Some(digit) = iter.next() {
            if iter.peek() == Some(&digit) {
                counter += 1
            } else {
                if counter == 2 {
                    return true;
                }
                counter = 1 // reset
            }
        }

        false
    }

    fn digits_dont_decrease(&self) -> bool {
        !self
            .letters
            .iter()
            .zip(self.letters.iter().skip(1))
            .any(|(a, b)| b < a)
    }
}

fn valid_password(password: &str) -> bool {
    Password::new(password).valid()
}

fn stricter_valid_password(password: &str) -> bool {
    Password::new(password).stricter_valid()
}

fn main() {
    let range = 254032..=789860;

    let count = range
        .clone()
        .filter(|n| valid_password(&format!("{}", n)))
        .count();
    println!("star 1: {}", count);

    let count = range
        .filter(|n| stricter_valid_password(&format!("{}", n)))
        .count();
    println!("star 2: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_password() {
        assert_eq!(valid_password("111111"), true);
        assert_eq!(valid_password("223450"), false);
        assert_eq!(valid_password("123789"), false);
    }

    #[test]
    fn test_stricter_valid_password() {
        assert_eq!(stricter_valid_password("112233"), true);
        assert_eq!(stricter_valid_password("123444"), false);
        assert_eq!(stricter_valid_password("111122"), true);
    }
}
