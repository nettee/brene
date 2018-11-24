package me.nettee.brene;

import me.nettee.brene.dom.AttrMap;
import me.nettee.brene.dom.Element;
import me.nettee.brene.dom.Node;
import me.nettee.brene.dom.Text;
import org.apache.commons.lang3.tuple.ImmutablePair;
import org.apache.commons.lang3.tuple.Pair;

import java.util.ArrayList;
import java.util.List;
import java.util.function.Predicate;

import static com.google.common.base.Preconditions.checkState;

public class HtmlParser {

    private char[] input;
    private int pos;

    private HtmlParser(String source) {
        this.input = source.toCharArray();
        this.pos = 0;
    }

    public static Node parse(String source) {
        HtmlParser parser = new HtmlParser(source);
        return parser.parse();
    }

    private Node parse() {
        List<Node> nodes = parseNodes();

        if (nodes.size() == 1) {
            return nodes.get(0);
        } else {
            return Node.element("html", new AttrMap(), nodes);
        }
    }

    private List<Node> parseNodes() {
        List<Node> nodes = new ArrayList<>();
        while (true) {
            consumeWhitespace();
            if (eof() || startsWith("</")) {
                break;
            }
            nodes.add(parseNode());
        }
        return nodes;
    }

    private Node parseNode() {
        char c = nextChar();
        if (c == '<') {
            return parseElement();
        } else {
            return parseText();
        }
    }

    private Element parseElement() {
        checkState(consumeChar() == '<');
        String tagName = parseName();
        AttrMap attrs = parseAttributes();
        checkState(consumeChar() == '>');

        List<Node> children = parseNodes();

        checkState(consumeChar() == '<');
        checkState(consumeChar() == '/');
        checkState(parseName().equals(tagName));
        checkState(consumeChar() == '>');

        return Node.element(tagName, attrs, children);
    }

    private Text parseText() {
        return Node.text(consumeWhile(c -> c != '<'));
    }

    private AttrMap parseAttributes() {
        AttrMap attributes = new AttrMap();
        while (true) {
            consumeWhitespace();
            if (nextChar() == '>') {
                break;
            }
            Pair<String, String> attr = parseAttr();
            attributes.insert(attr.getLeft(), attr.getRight());
        }
        return attributes;
    }

    private Pair<String, String> parseAttr() {
        String name = parseName();
        checkState(consumeChar() == '=');
        String value = parseAttrValue();
        return new ImmutablePair<>(name, value);
    }

    private String parseAttrValue() {
        char openQuote = consumeChar();
        checkState(openQuote == '"' || openQuote == '\'');
        String value = consumeWhile(c -> c != openQuote);
        checkState(consumeChar() == openQuote);
        return value;
    }

    private String parseName() {
        return consumeWhile(((Predicate<Character>) Character::isAlphabetic)
                .or(Character::isDigit));
    }

    private String consumeWhitespace() {
        return consumeWhile(Character::isWhitespace);
    }

    private String consumeWhile(Predicate<Character> test) {
        StringBuilder sb = new StringBuilder();
        while (!eof() && test.test(nextChar())) {
            sb.append(consumeChar());
        }
        return sb.toString();
    }

    private char consumeChar() {
        return input[pos++];
    }

    private boolean startsWith(String s) {
        return new String(input, pos, input.length - pos).startsWith(s);
    }

    private char nextChar() {
        return input[pos];
    }

    private boolean eof() {
        return pos >= input.length;
    }
}


