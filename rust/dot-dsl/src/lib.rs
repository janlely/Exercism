pub mod graph {
    use graph_items::node::Node;
    use graph_items::edge::Edge;
    use std::collections::HashMap;

    #[derive(PartialEq, Debug, Clone)]
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

        pub fn with_nodes(&self, nodes: &Vec<Node>) -> Self {
            Self {
                nodes: nodes.clone(),
                ..self.clone()
            }
        }
        pub fn with_edges(&self, edges: &Vec<Edge>) -> Self {
            Self {
                edges: edges.clone(),
                ..self.clone()
            }
        }
        pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
            Self {
                attrs: attrs.into_iter().map(|(a,b)| (String::from(*a), String::from(*b))).collect(),
                ..self.clone()
            }
        }
        pub fn get_node(&self, node: &str) -> Option<Node> {
            self.nodes.iter().map(|n| n.clone()).find(|n| n.name == node)
        }
    }
    pub mod graph_items {
        pub mod node {
            use std::collections::HashMap;
            #[derive(PartialEq, Debug, Clone)]
            pub struct Node {
                pub name: String,
                pub attrs: HashMap<String, String>,
            }
            impl Node {
                pub fn new(name: &str) -> Self {
                    Node {
                        name: name.to_string(),
                        attrs: HashMap::new(),
                    }
                }

                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        attrs: attrs.iter().map(|(a,b)| (String::from(*a), String::from(*b))).collect(),
                        ..self.clone()

                    }
                }

                pub fn get_attr(&self, name: &str) -> Option<&str> {
                    match self.attrs.get(name) {
                        Some(value) => Some(&value),
                        _ => None
                    }
                }
            }
        }
        pub mod edge {
            use super::node::Node;
            use std::collections::HashMap;
            #[derive(PartialEq, Debug, Clone)]
            pub struct Edge {
                pub a: Node,
                pub b: Node,
                pub attrs: HashMap<String, String>,
            }
            impl Edge {
                pub fn new(a: &str, b: &str) -> Self {
                    Edge {
                        a: Node::new(a),
                        b: Node::new(b),
                        attrs: HashMap::new(),
                    }
                }
                pub fn with_attrs(&self, attrs: &[(&str, &str)]) -> Self {
                    Self {
                        attrs: attrs.iter().map(|(a,b)| (String::from(*a), String::from(*b))).collect(),
                        ..self.clone()
                    }
                }
            }
        }
    }
}
