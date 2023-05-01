# `render_as_tree`

## What is it

It's a library that allows you to visualize tree data structures in Rust with
output like `tree(1)`. For example,

```
Parent
├── Child 1
├── Child 2
│   ├── Grandchild 1
│   └── Grandchild 2
└── Child 3
```

This crate was extracted from [ruut][ruut], a CLI intended for doing the same
thing. See that repo if you're interested in executing a tree visualizer from
the commandline or for something that can process common serialized data types
(e.g. JSON).

[ruut]: https://github.com/hibachrach/ruut
