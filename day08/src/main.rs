fn main() {
    let input = include_str!("input.txt");
    let input = parse(input);
    let node = parse_node(&input, 0);
    assert_eq!(41760, node.sum_meta());
    assert_eq!(25737, node.value());
}

#[derive(Debug, Eq, PartialEq)]
struct Node {
    meta_start: Option<usize>,
    meta_end: Option<usize>,
    children: Vec<Node>,
    meta: Vec<usize>,
}

impl Node {
    fn sum_meta(&self) -> usize {
        let mut sum = 0;
        for child in &self.children {
            sum += child.sum_meta();
        }
        sum += self.meta.iter().sum::<usize>();
        sum
    }

    fn value(&self) -> usize {
        let mut value = 0;
        if self.children.is_empty() {
            value = self.meta.iter().sum::<usize>();
        } else {
            for m in &self.meta {
                if let Some(child) = self.children.get(*m - 1) {
                    value += child.value();
                }
            }
        }
        value
    }
}

fn parse(input: &str) -> Vec<usize> {
    input
        .trim()
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn parse_node(input: &Vec<usize>, start: usize) -> Node {
    let mut node = Node {
        meta_start: None,
        meta_end: None,
        children: Vec::new(),
        meta: Vec::new(),
    };

    let num_children = input[start];
    let num_meta = input[start + 1];
    let mut start = start + 2;
    for _ in 0..num_children {
        let next_node = parse_node(input, start);
        start = next_node.meta_end.unwrap();
        node.children.push(next_node);
    }

    node.meta_start = Some(start);
    node.meta_end = Some(start + num_meta);
    node.meta = input[start..start + num_meta].to_vec();
    node
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let input = parse("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
        let node = parse_node(&input, 0);
        assert_eq!(138, node.sum_meta());
        assert_eq!(66, node.value());
    }
}
