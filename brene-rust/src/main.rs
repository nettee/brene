use html::Parser;

pub mod dom;
pub mod html;

fn main() {
    let source = "<html> <head> <title> DOM class </title> </head>\
     <body> <h1> DOM class 1 </h1> <p> hello, world! </p> </body> </html>";
    let html = Parser::parse(String::from(source));
    dom::pretty_print(&html);
}
