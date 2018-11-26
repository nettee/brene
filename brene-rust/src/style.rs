use std::collections::HashMap;

use css::Value;
use dom::Node;
use dom::ElementData;
use css::Selector;
use css::Selector::Simple;
use css::SimpleSelector;
use css::Specificity;
use css::Rule;
use css::Stylesheet;
use dom::NodeType::Element;
use dom::NodeType::Text;
use dom::NodeType::Comment;

// Map from CSS property names to values
type PropertyMap = HashMap<String, Value>;

// A node with associated styled data
#[derive(Debug)]
pub struct StyledNode<'a> {
    node: &'a Node, // pointer to a DOM node
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

fn matches(element: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Simple(ref simple_selector) => matches_simple_selector(element, simple_selector),
    }
}

fn matches_simple_selector(element: &ElementData, selector: &SimpleSelector) -> bool {
    // Check type selector
    if selector.tag_name.iter().any(|name| element.tag_name != *name) {
        return false;
    }

    // Check ID selector
    if selector.id.iter().any(|id| element.id() != Some(id)) {
        return false;
    }

    // Check class selectors
    let element_classes = element.classes();
    if selector.class.iter().any(|class| !element_classes.contains(&**class)) {
        return false;
    }

    // We didn't find any non-matching selector components
    true
}

type MatchedRule<'a> = (Specificity, &'a Rule);

// If `rule` matches `element`, return a `MatchedRule`. Otherwise return `None`
fn match_rule<'a>(element: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    // Find the first (highest-specificity) matching selector
    rule.selectors.iter()
        .find(|selector| matches(element, *selector))
        .map(|selector| (selector.specificity(), rule))
}

// Find all CSS rules that match the given element
fn matching_rules<'a>(element: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet.rules.iter()
        .filter_map(|rule| match_rule(element, rule))
        .collect()
}

// Apply styles to a single element, returning the specified values
fn specified_values(element: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(element, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)|
        a.cmp(&b)
    );

    for (_, rule) in rules {
        for declaration in &rule.declarations {
            values.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    values
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        specified_values: match root.node_type {
            Element(ref element) => specified_values(element, stylesheet),
            Comment(_) => HashMap::new(),
            Text(_) => HashMap::new(),
        },
        children: root.children.iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}