trait Node {
    fn send_msg(&self, msg: &str) -> bool;
}

struct LocalNode {
    id: i32,
}

impl Node for LocalNode {
    fn send_msg(&self, msg: &str) -> bool {
        println!("The message [{}] is sent in local [{}].", msg, self.id);
        true
    }
}

struct ManyNodes {
    nodes: Vec<Box<dyn Node>>,
}

fn main() {
    let many_nodes = ManyNodes {
        nodes: vec![
            Box::new(LocalNode { id: 1 }),
            Box::new(LocalNode { id: 2 }),
            Box::new(LocalNode { id: 3 }),
        ],
    };
    for n in many_nodes.nodes {
        n.send_msg("Hey there!");
    }
}
