struct A {
    id: i32,
    num: i32,
}

impl A {
    fn do_first(&self, num: i32) -> Result<i32, &str> {
        println!("id: {}", self.id);
        let n = self.id + num;

        println!("sum result: {}", n);
        if n > 0 {
            return Ok(n);
        } else {
            return Err("Got a non-positive number");
        }
    }
    
    fn do_second(&mut self, num: i32) -> Result<(), &str> {
        println!("id: {}", self.id);
        self.num = num;

        if self.id < self.num {
            return Ok(());
        } else {
            return Err("id is larger or equal to num now");
        }
    }
}

fn simple_immutable_use(s: &str) {
    println!("{}", s);
}

fn test_refs() {
    let mut s = String::from("value");

    simple_immutable_use(&s);  // immutable use at first

    s.clear();  // mmutable use here

    println!("try immutable use again: {s}");
}

fn for_immutable_use_later(s: &str) -> &str {
    &s[..]
}

fn test_refs_may_err() {
    let mut s = String::from("value");

    // let ret = for_immutable_use_later(&s);  // immutable use at first
    for_immutable_use_later(&s);                 // immutable use at first

    s.clear();     // mmutable use here          // may cause compiler error *1*

    // println!("try immutable use again: {ret}");  // may cause compiler error *2*
}

fn main() {
    let a = A {id: 0, num: 0};
    match a.do_first(10) {
        Ok(n) => println!("sum: {}", n),
        Err(msg) => println!("{}", msg),
    }

    let mut b = A {id: 0, num: 0};
    if b.do_second(10).is_err() {
        println!("Got an error");
    }

    test_refs();

    test_refs_may_err();
}
