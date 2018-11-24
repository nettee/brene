package me.nettee.brene.dom;

import java.util.HashSet;
import java.util.List;
import java.util.Optional;
import java.util.Set;

public class Element extends Node {

    private String tagName;
    private AttrMap attributes;

    Element(String tagName, AttrMap attributes, List<Node> children) {
        super(NodeType.ELEMENT);
        this.tagName = tagName;
        this.attributes = attributes;
        setChildren(children);
    }

    String getTagName() {
        return tagName;
    }

    Optional<String> getId() {
        return attributes.get("id");
    }

    Set<String> getClasses() {
        Optional<String> classOptional = attributes.get("class");
        if (classOptional.isPresent()) {
            Set<String> res = new HashSet<>();
            for (String part : classOptional.get().split(" ")) {
                res.add(part);
            }
            return res;
        } else {
            return new HashSet<>();
        }
    }
}
