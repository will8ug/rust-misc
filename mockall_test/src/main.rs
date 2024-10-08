use mockall_double::double;

mod thing {
    use mockall::automock;

    pub struct Thing {}

    #[automock]
    impl Thing {
        pub fn new() -> Self {
            Thing {}
        }

        pub fn foo(&self) -> u32 {
            println!("In the real implementation.");
            return 0;
        }
    }
}

#[double]
use thing::Thing;

fn do_stuff(thing: &Thing) -> u32 {
    thing.foo()
}

fn main() {
    let obj = Thing::new();
    do_stuff(&obj);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_foo() {
        let mut mock = Thing::default();
        mock.expect_foo().returning(|| 2);
        let result = do_stuff(&mock);
        assert_eq!(result, 2);
    }
}
