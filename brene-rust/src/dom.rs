use std::collections::{HashSet, HashMap};

pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attributes: AttrMap,
}

#[derive(Debug)]
pub enum NodeType {
    Element(ElementData),
    Text(String),
    Comment(String),
}

#[derive(Debug)]
pub struct Node {
    pub node_type: NodeType,
    pub children: Vec<Node>,
}

impl Node {
    pub fn element(tag_name: String, attributes: AttrMap, children: Vec<Node>) -> Node {
        Node {
            children,
            node_type: NodeType::Element(ElementData { tag_name, attributes }),
        }
    }

    pub fn text(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn comment(data: String) -> Node {
        Node {
            children: Vec::new(),
            node_type: NodeType::Comment(data),
        }
    }
}

impl ElementData {
    pub fn id(&self) -> Option<&String> {
        self.attributes.get("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classes) => classes.split(' ').collect(),
            None => HashSet::new(),
        }
    }
}

pub fn pretty_print(node: &Node) {
    print_rec(node, 0);
}

fn print_rec(node: &Node, indent: i32) {
    match node.node_type {
        NodeType::Element(ref data) => {
            print_indent(indent);
            println!("<{}>", data.tag_name);
            for child in &node.children {
                print_rec(child, indent + 1);
            }
            print_indent(indent);
            println!("</{}>", data.tag_name)
        },
        NodeType::Text(ref string) => {
            print_indent(indent);
            println!("{}", string)
        },
        NodeType::Comment(ref _string) => {},
    }
}

fn print_indent(indent: i32) {
    for _i in 1..=indent {
        print!("    ");
    }
}