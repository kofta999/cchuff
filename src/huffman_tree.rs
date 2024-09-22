#[derive(Debug)]
pub struct HuffNode {
    value: Option<char>,
    freq: usize,
    left: Option<Box<HuffNode>>,
    right: Option<Box<HuffNode>>,
}
use std::collections::HashMap;

pub fn build(input: &str) -> Vec<HuffNode> {
    generate_tree(generate_queue(calculate_frequency(input)))
}

fn generate_queue(map: HashMap<char, usize>) -> Vec<HuffNode> {
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

fn generate_tree(mut nodes: Vec<HuffNode>) -> Vec<HuffNode> {
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

        balance(nodes.len() - 1, &mut nodes);
    }

    nodes
}

fn balance(idx: usize, nodes: &mut Vec<HuffNode>) {
    if idx == 0 {
        return;
    };

    let p = (idx - 1) / 2;
    let pv = &nodes[p];
    let v = &nodes[idx];

    if pv.freq > v.freq {
        nodes.swap(idx, p);
        balance(p, nodes)
    }
}

fn calculate_frequency(input: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for char in input.chars() {
        map.entry(char)
            .and_modify(|v| {
                *v += 1;
            })
            .or_insert(1);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_freq() {
        let string = "hello world";
        let mut map = HashMap::new();

        map.insert('h', 1);
        map.insert('e', 1);
        map.insert('l', 3);
        map.insert('o', 2);
        map.insert('w', 1);
        map.insert('d', 1);
        map.insert('r', 1);
        map.insert(' ', 1);

        assert_eq!(calculate_frequency(string), map)
    }

    // #[test]
    // fn test_build_tree() {
    //     let vec = vec![
    //         HuffNode {
    //             freq: 2,
    //             value: Some('Z'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 7,
    //             value: Some('K'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 24,
    //             value: Some('M'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 32,
    //             value: Some('C'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 37,
    //             value: Some('U'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 42,
    //             value: Some('D'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 42,
    //             value: Some('L'),
    //             left: None,
    //             right: None,
    //         },
    //         HuffNode {
    //             freq: 120,
    //             value: Some('E'),
    //             left: None,
    //             right: None,
    //         },
    //     ];
    //     let res = build_tree(vec);

    //     dbg!(res);
    // }
}
