use std::{sync::{Arc, Mutex}, thread};

trait Node {
    fn show_msg(&self, msg: &str) -> bool;
}

struct LocalNode {
    id: i32,
}

impl Node for LocalNode {
    fn show_msg(&self, msg: &str) -> bool {
        println!("The message [{}] is sent in local [{}].", msg, self.id);
        true
    }
}

type BoxNode = Box<dyn Node + Sync + Send>;

struct ManyNodes {
    nodes: Vec<Arc<Mutex<BoxNode>>>,
}

fn main() {
    let mut node_vec = Vec::with_capacity(3);
    for i in 0..3 {
        let n = Box::new(LocalNode{id: i});
        node_vec.push(Arc::new(Mutex::new(n as BoxNode)));
    }

    let many_nodes = ManyNodes {
        nodes: node_vec,
    };
    let mut handlers = Vec::with_capacity(3);
    for n in &many_nodes.nodes {
        let _n = Arc::clone(n);
        handlers.push(thread::spawn(move || {
            _n.lock().unwrap().show_msg("Hi");
        }));
    }

    for h in handlers {
        h.join().unwrap();
    }
}
