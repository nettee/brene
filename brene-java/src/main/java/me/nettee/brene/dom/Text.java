package me.nettee.brene.dom;

public class Text extends Node {

    private final String text;

    Text(String data) {
        super(NodeType.TEXT);
        this.text = data;
    }

    String getText() {
        return text;
    }
}
