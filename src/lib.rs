use std::{
    fmt::{Debug, Display},
    io,
    num::ParseIntError,
    ops::RangeInclusive,
    str::FromStr,
};

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

/// Asks user for a number <T> in specified range.
pub fn prompt_number_range<T>(msg: &str, range: RangeInclusive<T>) -> T
where
    T: FromStr<Err = ParseIntError> + PartialOrd + Display + Debug,
{
    loop {
        println!("{}", msg);
        match read_number::<T>() {
            Ok(n) => {
                if range.contains(&n) {
                    return n;
                }
                println!(
                    "ENTER A NUMBER WITHIN {:?}, AND {:?}",
                    range.start(),
                    range.end()
                );
            }
            Err(_) => println!("ENTER A VALID NUMBER"),
        }
    }
}

/// Options for multiple element prompts:
///
/// Choose between a specific unit amount allowed or an amount within a range
pub enum PromptMultiOption {
    UnitAmount(usize),
    UnitAmountRange(RangeInclusive<usize>),
}

fn check_multi_option(o: &PromptMultiOption, l: usize) -> bool {
    use PromptMultiOption::*;

    match o {
        UnitAmount(a) => {
            if l == *a {
                return true;
            } else {
                println!("THERE MUST BE {a} UNITS")
            }
        }
        UnitAmountRange(r) => {
            if r.contains(&l) {
                return true;
            } else {
                println!(
                    "AMOUNT OF UNITS MUST BE WITHIN {:?} AND {:?}",
                    r.start(),
                    r.end()
                );
            }
        }
    }

    false
}

/// Asks users for a multiple string answer, units seperated by the "separator".
///
/// You can also optionally set a range for the amount of units expected.
pub fn prompt_multi_string(
    msg: &str,
    separator: &str,
    option: Option<PromptMultiOption>,
) -> Vec<String> {
    loop {
        println!("{}", msg);

        let input = read_line();
        let input: Vec<String> = input.split(separator).map(str::to_string).collect();

        if let Some(o) = &option {
            if check_multi_option(&o, input.len()) {
                return input;
            }
        } else {
            return input;
        }
    }
}

/// Asks user for a multiple number(T) answer, units spearated by the "separator".
///
/// You can also optionally set a range for the amount of units expected,
///
/// and a range in which the individual numbers should be.
pub fn prompt_multi_number<T>(
    msg: &str,
    separator: &str,
    option: Option<PromptMultiOption>,
    range: Option<RangeInclusive<T>>,
) -> Vec<T>
where
    T: FromStr + PartialOrd + Debug,
{
    loop {
        println!("{}", msg);

        let input = read_line();
        let input: Vec<&str> = input.split(separator).collect();

        let mut ok = if let Some(o) = &option {
            check_multi_option(&o, input.len())
        } else {
            true
        };

        let mut nums = Vec::new();

        if ok {
            for i in &input {
                match i.parse::<T>() {
                    Ok(n) => {
                        if let Some(r) = &range {
                            if r.contains(&n) {
                                nums.push(n);
                            } else {
                                println!("NUMBER MUST BE WITHIN {:?} AND {:?}", r.start(), r.end());
                                ok = false;
                                break;
                            }
                        } else {
                            nums.push(n);
                        }
                    }
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
