package me.nettee.brene.dom;

import java.util.ArrayList;
import java.util.List;

public abstract class Node {

    protected enum NodeType {
        ELEMENT,
        TEXT,
        COMMENT,
    }

    private NodeType type;
    protected List<Node> children;

    protected Node(NodeType type) {
        this.type = type;
        this.children = new ArrayList<>();
    }

    protected void setChildren(List<Node> children) {
        this.children = children;
    }

    List<Node> getChildren() {
        return children;
    }

    public static Element element(String tagName, AttrMap attributes, List<Node> children) {
        return new Element(tagName, attributes, children);
    }

    public static Text text(String data) {
        return new Text(data);
    }

    public static Comment comment(String data) {
        return new Comment(data);
    }

    public void prettyPrint() {
        printRec(0);
    }

    private void printRec(int indent) {
        if (type == NodeType.ELEMENT) {
            Element element = (Element) this;
            printIndent(indent);
            System.out.println(String.format("<%s>", element.getTagName()));
            for (Node child : element.getChildren()) {
                child.printRec(indent + 1);
            }
            printIndent(indent);
            System.out.println(String.format("</%s>", element.getTagName()));

        } else if (type == NodeType.TEXT) {
            Text text = (Text) this;
            printIndent(indent);
            System.out.println(text.getText());

        } else if (type == NodeType.COMMENT) {
            // Do nothing

        } else {
            throw new AssertionError();
        }
    }

    private void printIndent(int indent) {
        for (int i = 0; i < indent; i++) {
            System.out.print("    ");
        }
    }
}
