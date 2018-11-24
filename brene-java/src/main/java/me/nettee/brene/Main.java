package me.nettee.brene;

import me.nettee.brene.dom.Node;

public class Main {

    public static void main(String[] args) {
        String source = "<html> <head> <title> DOM class </title> </head>" +
                "<body> <h1> DOM class 1 </h1> <p> hello, world! </p> </body> </html>";
        Node node = HtmlParser.parse(source);
        node.prettyPrint();
    }
}
