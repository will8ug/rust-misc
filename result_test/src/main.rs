use std::{error::Error, fmt::{Display, Formatter}};

#[derive(Debug)]
enum SadError {
    LevelOne(String),
    LevelTwo(String),
}

impl Display for SadError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            SadError::LevelOne(msg) => write!(f, "{}", msg),
            SadError::LevelTwo(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for SadError {}

fn main() {
    for i in 0..2 {
        let ret = do_something(i);
        match ret {
            Ok(num) => println!("Got a number: {}", num),
            Err(e) => println!("Got an error: {}", e),
        }
    }

    for i in -1..2 {
        match do_something_custom_err(i) {
            Ok(num) => println!("Working with: {}", num),
            Err(e) => println!("Got an error: {}", e),
        }
    }
}

fn do_something(i: i32) -> Result<i32, String> {
    println!("in do_something()");

    if i > 0 {
        return Ok(i + 1);
    } else {
        return Err(String::from("0 or negative number"));
    }
}

fn do_something_custom_err(i: i32) -> Result<i32, SadError> {
    println!("in do_something_custom_err()");

    if i > 0 {
        return Ok(i * 2);
    } else if i == 0 {
        return Err(SadError::LevelOne(String::from("0")));
    } else {
        return Err(SadError::LevelTwo(String::from("negative")));
    }
}