use std::collections::BinaryHeap;

#[derive(Clone, Debug)]
struct Node {
    weight: usize,
    value: Value,
}

#[derive(Clone, Debug)]
enum Value {
    Char(char),
    Node(Box<(Node, Node)>),
}

fn huffman_tree(text: impl AsRef<str>) -> Node {
    let mut freqs: Vec<(char, usize)> = Vec::new();
    for char in text.as_ref().chars() {
        let item = freqs.iter_mut().find(|e| e.0 == char);
        if let Some(item) = item {
            item.1 += 1;
        } else {
            freqs.push((char, 1));
        }
    }
    let mut nodes = BinaryHeap::from_iter(freqs.iter().map(|(char, freq)| Node {
        weight: *freq,
        value: Value::Char(*char),
    }));
    while nodes.len() > 1 {
        let one = nodes.pop().unwrap();
        let two = nodes.pop().unwrap();
        let node = Node {
            weight: one.weight + two.weight,
            value: Value::Node(Box::new((one, two))),
        };
        nodes.push(node);
    }
    nodes.pop().unwrap()
}

fn huffman_codex(tree: Node) -> Vec<(char, Vec<u8>)> {
    let mut codex = Vec::new();
    let mut traverse = Vec::new();
    traverse.push((tree, vec![]));
    while traverse.len() > 0 {
        let node = traverse.pop().unwrap();
        match node.0.value {
            Value::Char(char) => {
                println!("got {char}");
                codex.push((char, node.1))
            }
            Value::Node(ref nodes) => {
                let mut new_one = node.1.clone();
                new_one.push(0);
                let mut new_two = node.1.clone();
                new_two.push(1);
                traverse.push((nodes.0.clone(), new_one));
                traverse.push((nodes.1.clone(), new_two));
            }
        }
    }
    codex
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.weight.cmp(&self.weight)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

#[test]
fn test_huffman() {
    let text = "this is an example of a huffman tree";
    let tree = huffman_tree(&text);
    assert_eq!(tree.weight, 36);
    // dbg!(tree);
    let codex = huffman_codex(tree);
    dbg!(codex);
}
