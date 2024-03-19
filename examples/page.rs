use htrs::htmx::*;
use htrs::style::*;
use htrs::*;

pub struct User {
    username: String,
    password: String,
}

fn login_button(user: User) -> Element {
    div((
        button(&user.username)
            .padding(Edges::all(px(16)))
            .border(Edges::all(rem(2.0)))
            .margin(Edges::all(px(0)).top(rem(4.0)))
            .post("/login")
            .trigger("onClick")
            .swap(Swap::Outer)
            .target(Target::This),
        image("/wade.jpg")
    ))
}

pub fn main() {
    println!(
        "{}",
        login_button(User {
            username: "Jw2476".to_string(),
            password: "".to_string()
        })
        .render()
    );
}
