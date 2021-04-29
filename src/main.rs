use askama::Template;

#[derive(Template)]
#[template(path = "main.html")]
struct Main;

fn main() {
    println!("{:?}", Main.render());
}
