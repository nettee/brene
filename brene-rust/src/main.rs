pub mod dom;
pub mod html;
pub mod css;
pub mod style;

fn main() {
    let html_source = "<html> <head> <title> DOM class </title> </head>\
     <body> <h1> DOM class 1 </h1> <p> hello, world! </p> </body> </html>";
    let dom_tree = html::Parser::parse(String::from(html_source));
    dom::pretty_print(&dom_tree);

    let css_source = "h1, h2, h3 { margin: auto; color: #cc0000; } \
    div.note { margin-bottom: 20px; padding: 10px; } \
    #answer { display: none; }";
    let stylesheet = css::parse(String::from(css_source));
    stylesheet.pretty_print();

    let styled_dom_tree = style::style_tree(&dom_tree, &stylesheet);
    println!("{:?}", styled_dom_tree);
}

