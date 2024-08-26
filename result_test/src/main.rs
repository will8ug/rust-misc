fn main() {
    for i in 0..2 {
        let ret = do_something(i);
        match ret {
            Ok(num) => println!("Got a number: {}", num),
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