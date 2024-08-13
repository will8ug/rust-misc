use std::sync::{Arc, Mutex};

#[allow(dead_code)]
struct SharedState {
    id: i32,
}

#[allow(dead_code)]
impl SharedState {
    fn do_something_in_lock(this: Arc<Mutex<Self>>, num: i32) {
        let mut state = this.lock().unwrap();
        state.id = num;
    }
}

fn main() {
    println!("Run the tests");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_new() {
        let state = Mutex::new(SharedState { id: 1 });
        {
            let mut state_inside = state.lock().unwrap();
            state_inside.id = 2;
        }
        assert_eq!(state.lock().unwrap().id, 2);
    }

    #[test]
    fn test_lock_struct() {
        let state = Arc::new(Mutex::new(SharedState { id: 1 }));
        SharedState::do_something_in_lock(Arc::clone(&state), 2);
        assert_eq!(state.lock().unwrap().id, 2);
    }
}
