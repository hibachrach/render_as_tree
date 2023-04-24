pub trait Node {
    type I<'a>: DoubleEndedIterator<Item = &'a Self>
    where
        Self: 'a;

    fn name(&self) -> &str;
    fn children(&self) -> Self::I<'_>;
}

pub fn render<T: Node>(node: &T) -> Vec<String> {
    let mut lines = vec![node.name().to_owned()];
    let mut children = node.children();
    let maybe_last_child = children.next_back();
    let non_last_children: Vec<&T> = children.collect();
    if let Some(last_child) = maybe_last_child {
        let child_node_lines = non_last_children.iter().flat_map(|child| {
            render(*child)
                .iter()
                .enumerate()
                .map(|(idx, child_line)| {
                    if idx == 0 {
                        format!("├── {}", child_line)
                    } else {
                        format!("│   {}", child_line)
                    }
                })
                .collect::<Vec<String>>()
        });
        let last_child_node_lines = render(last_child);
        let formatted_last_child_node_lines_iter =
            last_child_node_lines
                .iter()
                .enumerate()
                .map(|(idx, child_line)| {
                    if idx == 0 {
                        format!("└── {}", child_line)
                    } else {
                        format!("    {}", child_line)
                    }
                });
        let children_lines = child_node_lines.chain(formatted_last_child_node_lines_iter);
        lines.extend(children_lines);
    }
    lines
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;

    #[derive(Debug, PartialEq)]
    struct BasicNode {
        pub name: String,
        pub children: VecDeque<BasicNode>,
    }

    impl BasicNode {
        pub fn new(name: String) -> BasicNode {
            BasicNode {
                name,
                children: VecDeque::new(),
            }
        }
    }

    impl Node for BasicNode {
        type I<'a> = std::collections::vec_deque::Iter<'a, Self>;

        fn name(&self) -> &str {
            &self.name
        }
        fn children(&self) -> Self::I<'_> {
            self.children.iter()
        }
    }

    #[test]
    fn trivial_case() {
        assert_eq!(
            render(&BasicNode::new(String::from("beans"))),
            vec![String::from("beans")]
        )
    }

    #[test]
    fn simple_case() {
        let root = BasicNode {
            name: String::from("root - selena"),
            children: VecDeque::from(vec![
                BasicNode {
                    name: String::from("child 1 - sam"),
                    children: VecDeque::from(vec![
                        BasicNode::new(String::from("grandchild 1A - burt")),
                        BasicNode::new(String::from("grandchild 1B - crabbod")),
                        BasicNode::new(String::from("grandchild 1C - mario")),
                    ]),
                },
                BasicNode {
                    name: String::from("child 2 - dumptruck"),
                    children: VecDeque::from(vec![
                        BasicNode::new(String::from("grandchild 2A - tilly")),
                        BasicNode::new(String::from("grandchild 2B - curling iron")),
                    ]),
                },
            ]),
        };
        assert_eq!(
            render(&root),
            vec![
                String::from("root - selena"),
                String::from("├── child 1 - sam"),
                String::from("│   ├── grandchild 1A - burt"),
                String::from("│   ├── grandchild 1B - crabbod"),
                String::from("│   └── grandchild 1C - mario"),
                String::from("└── child 2 - dumptruck"),
                String::from("    ├── grandchild 2A - tilly"),
                String::from("    └── grandchild 2B - curling iron"),
            ]
        );
    }
}
