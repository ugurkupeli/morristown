use std::{fmt::Display, io, num::ParseIntError, str::FromStr};

/// Options for displaying game instructions in the intro.
///
/// Determine if we want to ask to show instructions
/// and whether that prompt expects a 1 or 0.
///
/// You can also create a multiline (Vec<&str>) instruction set.
pub struct Instructions<'a> {
    ask_numeric: (bool, bool),
    msg: &'a str,
    instructions: &'a str,
    instructions_multiline: Vec<&'a str>,
    multiline: bool,
}

impl<'a> Instructions<'a> {
    pub fn new(ask_to_show: bool, numeric: bool, msg: &'a str, instructions: &'a str) -> Self {
        Instructions {
            ask_numeric: (ask_to_show, numeric),
            msg,
            instructions,
            instructions_multiline: Vec::new(),
            multiline: false,
        }
    }

    pub fn new_multiline(
        ask_to_show: bool,
        numeric: bool,
        msg: &'a str,
        instructions: Vec<&'a str>,
    ) -> Self {
        Instructions {
            ask_numeric: (ask_to_show, numeric),
            msg,
            instructions: "",
            instructions_multiline: instructions,
            multiline: true,
        }
    }

    pub fn print(&self) {
        if self.ask_numeric.0 {
            if prompt_bool(self.msg, self.ask_numeric.1) {
                if self.multiline {
                    for l in self.instructions_multiline.iter() {
                        println!("{}", l);
                    }
                } else {
                    println!("{}", self.instructions);
                }
            }
        }
    }
}

/// Prints the game intro template.
pub fn print_intro(name: &str) {
    println!("\n\n\t\t{name}\nCREATIVE COMPUTING MORRISTOWN, NEW JERSEY\n");
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");
    input.trim().to_uppercase()
}

fn read_number<T: FromStr<Err = ParseIntError>>() -> Result<T, ParseIntError> {
    let input = read_line();
    input.parse::<T>()
}

/// Asks user for a simple string.
pub fn prompt_string(msg: &str) -> String {
    println!("{}", msg);
    read_line()
}

/// Prompts user for a yes/no answer.
///
/// Set parameter "numeric" to true if we want a 1 or 0 answer.
pub fn prompt_bool(msg: &str, numeric: bool) -> bool {
    loop {
        println!("{}", msg);
        if numeric {
            match read_number::<u8>() {
                Ok(n) => match n {
                    1 => return true,
                    0 => return false,
                    _ => println!("ENTER 1 (YES) OR 0 (NO)"),
                },
                Err(_) => println!("ENTER A NUMBER (1 OR 0)"),
            }
        } else {
            match read_line().as_str() {
                "YES" | "Y" => return true,
                "NO" | "N" => return false,
                _ => println!("ENTER (Y)ES OR (N)O"),
            }
        }
    }
}

/// Ask user for a number (of type T).
pub fn prompt_number<T: FromStr<Err = ParseIntError>>(msg: &str) -> T {
    loop {
        println!("{}", msg);
        match read_number::<T>() {
            Ok(n) => return n,
            Err(_) => println!("ENTER A VALID NUMBER"),
        }
    }
}

/// Ask user for a number (of type T) between (and including) min and max.
pub fn prompt_number_range<T>(msg: &str, min: T, max: T) -> T
where
    T: FromStr<Err = ParseIntError> + PartialOrd + Display,
{
    loop {
        println!("{}", msg);
        match read_number::<T>() {
            Ok(n) => {
                if n >= min && n <= max {
                    return n;
                }
                println!("ENTER A NUMBER BETWEEN (INCLUDING) {} AND {}", min, max);
            }
            Err(_) => println!("ENTER A VALID NUMBER"),
        }
    }
}

/// Asks users for a multiple string answer, units seperated by the "separator".
///
/// You can also optionally set a min and max amount of units expected.
pub fn prompt_multi(msg: &str, separator: &str, min_max: Option<(usize, usize)>) -> Vec<String> {
    loop {
        println!("{}", msg);
        let input = read_line();
        let input: Vec<String> = input.split(separator).map(str::to_string).collect();
        if let Some(mm) = min_max {
            let (min, max) = mm;
            let l = input.len();
            if l <= max && l >= min {
                return input;
            }
            println!("ENTER MIN: {} AND MAX: {} UNITS", min, max);
        } else {
            return input;
        }
    }
}

/// Asks user for a multiple number(T) answer, units spearated by the "separator".
///
/// You can also optionally set a min and max amount of units expected.
pub fn prompt_multi_number<T>(msg: &str, separator: &str, min_max: Option<(usize, usize)>) -> Vec<T>
where
    T: FromStr,
{
    loop {
        println!("{}", msg);

        let input = read_line();
        let input: Vec<&str> = input.split(separator).collect();

        let mut ok = false;

        if let Some(mm) = min_max {
            let (min, max) = mm;
            let l = input.len();

            if l <= max && l >= min {
                ok = true;
            } else {
                println!("ENTER MIN: {} AND MAX: {} UNITS", min, max);
            }
        } else {
            ok = true;
        }

        let mut nums = Vec::new();

        if ok {
            for i in &input {
                match i.parse::<T>() {
                    Ok(n) => nums.push(n),
                    Err(_) => {
                        println!("ENTER ONLY NUMBERS");
                        ok = false;
                        break;
                    }
                }
            }
        }

        if ok {
            return nums;
        }
    }
}
