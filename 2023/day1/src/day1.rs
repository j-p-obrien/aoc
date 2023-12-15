#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Config {
    Empty,
    One(char),
    Two([char; 2]),
}

impl Default for Config {
    fn default() -> Self {
        Self::Empty
    }
}

impl Config {
    pub fn from_str(data: &str) -> Self {
        data.chars().fold(Self::default(), |mut config, c| {
            Self::push_char(&mut config, c)
        })
    }

    fn push_char(&mut self, c: char) -> Self {
        if c.is_numeric() {
            match self {
                Config::Empty => Self::One(c),
                Config::One(c1) => Config::Two([*c1, c]),
                Config::Two([_, c1]) => {
                    *c1 = c;
                    *self
                }
            }
        } else {
            *self
        }
    }

    pub fn to_num(&self) -> u32 {
        match self {
            Config::One(c) => [c, c].into_iter().collect::<String>().parse().unwrap(),
            Config::Two(chars) => chars.into_iter().collect::<String>().parse().unwrap(),
            Config::Empty => 0,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{fs, vec};

    use super::Config;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("day1input.txt").unwrap();
        let configs: Vec<u32> = data
            .lines()
            .take(3)
            .map(|line| Config::from_str(line).to_num())
            .collect();
        assert_eq!(configs, vec![33, 11, 98])
    }
}
