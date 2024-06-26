use std::{
    fmt::{Debug, Error},
    io::{self, stdin, Lines, StdinLock},
    num::ParseIntError,
    str::FromStr,
};

macro_rules! parse_input {
    ($x:expr, $t:ident) => {
        $x.trim().parse::<$t>().unwrap()
    };
}

#[derive(Default)]
pub struct Input;

trait User {
    fn get_lines(&self) -> Result<Vec<String>, String>;

    fn read<InputType>(&self) -> Result<InputType, String>
    where
        InputType: Debug + Sized + Default + FromStr,
        <InputType as std::str::FromStr>::Err: std::fmt::Debug;
}

impl User for Input {
    fn get_lines(&self) -> Result<Vec<String>, String> {
        io::stdin()
            .lines()
            .map(|elem| {
                if let Ok(elem) = elem {
                    Ok(elem)
                } else {
                    Err("Error getting all lines".to_string())
                }
            })
            .collect::<Result<Vec<String>, String>>()
    }

    fn read<InputType>(&self) -> Result<InputType, String>
    where
        InputType: Debug + Sized + std::default::Default + std::str::FromStr,
        <InputType as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let mut handle = io::stdin().lines();
        if let Some(item) = handle.next() {
            if let Ok(item) = item {
                Ok(parse_input!(item, InputType))
            } else {
                Err("Failed to parse the value from string".to_string())
            }
        } else {
            Ok(InputType::default())
        }
    }
}

#[cfg(test)]
mod test_super {
    use super::*;

    #[test]
    fn test_reading_input() {
        let input = Input::default();
        dbg!(input.read::<String>());
        dbg!(input.read::<u32>());
        dbg!(input.read::<bool>());
    }
}
