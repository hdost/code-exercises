pub mod graph {
    use graph_items::edge::Edge;
    use graph_items::node::Node;
    use std::collections::HashMap;
    pub struct Graph {
        pub nodes: Vec<Node>,
        pub edges: Vec<Edge>,
        pub attrs: HashMap<String, String>,
    }

    impl Graph {
        pub fn new() -> Self {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                attrs: HashMap::new(),
            }
        }
        pub fn with_nodes(mut self, new_nodes: &[Node]) -> Self {
            self.nodes = new_nodes.to_vec();
            self
        }
        pub fn with_edges(mut self, new_edges: &[Edge]) -> Self {
            self.edges = new_edges.to_vec();
            self
        }
        pub fn with_attrs(mut self, new_attr: &[(&str, &str)]) -> Self {
            for attr in new_attr {
                self.attrs.insert(attr.0.to_string(), attr.1.to_string());
            }

            self
        }
        pub fn get_node(&self, node_name: &str) -> Option<& Node> {
            for ref node in self.nodes.iter() {
                if node.name == node_name {
                    return Some(node)
                }
            }
            None
        }
    }

    pub mod graph_items {
        pub mod edge {

            use std::cmp;
            use std::collections::HashMap;
            use std::fmt;

            #[derive(fmt::Debug, Clone)]
            pub struct Edge {
                source: String,
                target: String,
                attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(source: &str, target: &str) -> Self {
                    Edge {
                        source: source.to_string(),
                        target: target.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, new_attr: &[(&str, &str)]) -> Self {
                    for attr in new_attr {
                        self.attrs.insert(attr.0.to_string(), attr.1.to_string());
                    }

                    self
                }
            }
            impl cmp::PartialEq for Edge {
                fn eq(&self, other: &Self) -> bool {
                    self.source == other.source
                        && self.target == other.target
                        && self.attrs == other.attrs
                }
            }
        }
        pub mod node {
            use std::cmp;
            use std::collections::HashMap;
            use std::fmt;
            #[derive(fmt::Debug, Clone)]
            pub struct Node {
                pub name: String,
                attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(mut self, new_attrs: &[(&str, &str)]) -> Self {
                    for pair in new_attrs {
                        self.attrs.insert(pair.0.to_string(), pair.1.to_string());
                    }
                    self
                }
                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    if let Some(value) = self.attrs.get(name) {
                        Some(value)
                    } else {
                        None
                    }
                }
            }
            impl cmp::PartialEq for Node {
                fn eq(&self, other: &Self) -> bool {
                    self.name == other.name && self.attrs == other.attrs
                }
            }
        }
    }
}
