use std::collections::HashMap;

use css::Value;
use dom::Node;
use dom::ElementData;
use css::Selector;
use css::Selector::Simple;
use css::SimpleSelector;
use css::Specificity;
use css::Rule;

// Map from CSS property names to values
type PropertyMap = HashMap<String, Value>;

// A node with associated styled data
struct StyledNode<'a> {
    node: &'a Node, // pointer to a DOM node
    specified_values: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

fn matches(element: &ElementData, selector: &Selector) -> bool {
    match *selector {
        Simple(ref simple_selector) => matches_simple_selector(element, simple_selector),
        _ => false,
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