package me.nettee.brene.dom;

public class Comment extends Node {

    private final String content;

    Comment(String data) {
        super(NodeType.COMMENT);
        this.content = data;
    }
}
