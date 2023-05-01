#![warn(missing_docs)]

//! A library that allows you to visualize tree data structures in Rust with
//! output like `tree(1)`, like so:
//!
//! ```text
//! Parent
//! ├── Child 1
//! ├── Child 2
//! │   ├── Grandchild 1
//! │   └── Grandchild 2
//! └── Child 3
//! ```
//!
//! This crate was extracted from [ruut](https://github.com/hibachrach/ruut), a CLI intended for doing the same
//! thing. See that repo if you're interested in executing a tree visualizer from
//! the commandline or for something that can process common serialized data types
//! (e.g. JSON).

/// Represents a node in the tree.
///
/// Important as [render] takes a [`Node`] as its only parameter.
///
/// # Example
///
/// ```
/// use render_as_tree::Node;
/// struct BasicNode {
///     pub name: String,
///     pub children: Vec<BasicNode>,
/// }
///
/// impl BasicNode {
///     pub fn new(name: String) -> BasicNode {
///         BasicNode {
///             name,
///             children: Vec::new(),
///         }
///     }
/// }
///
/// impl Node for BasicNode {
///     type I<'a> = std::slice::Iter<'a, Self>;
///
///     fn name(&self) -> &str {
///         &self.name
///     }
///     fn children(&self) -> Self::I<'_> {
///         self.children.iter()
///     }
/// }
/// ```
pub trait Node {
    /// An iterator over the children of this node
    type I<'a>: DoubleEndedIterator<Item = &'a Self>
    where
        Self: 'a;

    /// What is displayed for this node when rendered
    fn name(&self) -> &str;
    /// The immediate children of this node in the tree
    fn children(&self) -> Self::I<'_>;
}

/// Renders the given [`Node`] in a human-readable format
///
/// Here's an example:
/// ```no_run
/// vec![
///     String::from("root - selena"),
///     String::from("├── child 1 - sam"),
///     String::from("│   ├── grandchild 1A - burt"),
///     String::from("│   ├── grandchild 1B - crabbod"),
///     String::from("│   └── grandchild 1C - mario"),
///     String::from("└── child 2 - dumptruck"),
///     String::from("    ├── grandchild 2A - tilly"),
///     String::from("    └── grandchild 2B - curling iron"),
/// ];
/// ```
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
    use super::*;

    #[derive(Debug, PartialEq)]
    struct BasicNode {
        pub name: String,
        pub children: Vec<BasicNode>,
    }

    impl BasicNode {
        pub fn new(name: String) -> BasicNode {
            BasicNode {
                name,
                children: Vec::new(),
            }
        }
    }

    impl Node for BasicNode {
        type I<'a> = std::slice::Iter<'a, Self>;

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
            children: vec![
                BasicNode {
                    name: String::from("child 1 - sam"),
                    children: vec![
                        BasicNode::new(String::from("grandchild 1A - burt")),
                        BasicNode::new(String::from("grandchild 1B - crabbod")),
                        BasicNode::new(String::from("grandchild 1C - mario")),
                    ],
                },
                BasicNode {
                    name: String::from("child 2 - dumptruck"),
                    children: vec![
                        BasicNode::new(String::from("grandchild 2A - tilly")),
                        BasicNode::new(String::from("grandchild 2B - curling iron")),
                    ],
                },
            ],
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
