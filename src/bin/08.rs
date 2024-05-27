advent_of_code::solution!(8);

struct NavNode {
    children: Vec<Box<NavNode>>,
    metadata: Vec<u32>
}

impl NavNode{
    fn metadata_total(&self) -> u32 {
        let total = self.children
        .iter()
        .fold(
            0,
            |total, child| {
                total + child.metadata_total()
            }
        );

        self.metadata
        .iter()
        .fold(
            total,
            |total, metadata| {
                total + metadata
            }
        )
    }

    fn metadata_value(&self) -> u32 {
        if self.children.len() == 0 {
            self.metadata_total()
        } else {
            self.metadata
            .iter()
            .fold(
                0,
                |total, metadata_number| {
                    let metadata_usize = *metadata_number as usize;
                    if metadata_usize > 0 && metadata_usize <= self.children.len() {
                        total + self.children[metadata_usize - 1].metadata_value()
                    } else {
                        total
                    }
                }
            )
        }
    }
}

fn get_next_node(numbers_list: &[u32]) -> (NavNode, &[u32]) {
    let child_node_numbers = numbers_list[0];
    let metadata_entries = numbers_list[1];

    let mut children = Vec::new();

    let mut remaining_numbers_list = &numbers_list[2..];

    for _ in 0..child_node_numbers {
        let child: NavNode;
        (child, remaining_numbers_list) = get_next_node(remaining_numbers_list);
        children.push(Box::new(child));
    }

    let metadata_entries_size = metadata_entries as usize;

    let metadata = remaining_numbers_list[..metadata_entries_size].to_vec();

    (NavNode{children: children, metadata: metadata}, &remaining_numbers_list[metadata_entries_size..])
}

pub fn part_one(input: &str) -> Option<u32> {
    let numbers_list: Vec<u32> = input
    .split(" ")
    .map(
        |number_string| {
            number_string.trim().parse::<u32>().unwrap()
        }
    ).collect();

    let (node, _) = get_next_node(&numbers_list);

    Some(node.metadata_total())
}

pub fn part_two(input: &str) -> Option<u32> {
    let numbers_list: Vec<u32> = input
    .split(" ")
    .map(
        |number_string| {
            number_string.trim().parse::<u32>().unwrap()
        }
    ).collect();

    let (node, _) = get_next_node(&numbers_list);

    Some(node.metadata_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(66));
    }
}
