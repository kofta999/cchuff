use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct HuffNode {
    pub value: Option<char>,
    pub freq: u32,
    pub left: Option<Box<HuffNode>>,
    pub right: Option<Box<HuffNode>>,
}

pub fn generate_queue(map: &BTreeMap<char, u32>) -> Vec<HuffNode> {
    let mut vec = map
        .iter()
        .map(|(k, v)| HuffNode {
            freq: *v,
            value: Some(*k),
            left: None,
            right: None,
        })
        .collect::<Vec<HuffNode>>();

    vec.sort_by_key(|node| node.freq);

    vec
}

pub fn generate_tree(mut nodes: Vec<HuffNode>) -> HuffNode {
    while nodes.len() > 1 {
        let curr = nodes.remove(0);
        let next = nodes.remove(0);

        let node = HuffNode {
            value: None,
            freq: curr.freq + next.freq,
            left: Some(Box::new(curr)),
            right: Some(Box::new(next)),
        };

        nodes.push(node);

        nodes.sort_by_key(|v| v.freq);
    }

    nodes.remove(0)
}
