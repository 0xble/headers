use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let l_padding = (0..(60 - input.len()) / 2).map(|_| " ").collect::<String>();

    let mut r_padding = l_padding.clone();
    if input.len() % 2 != 0 {
        r_padding.push(' ');
    }

    let output = format!(
        "{}\n{}{}{}{}{}\n{}",
        "////////////////////////////////////////////////////////////////",
        "//",
        l_padding,
        input.to_uppercase(),
        r_padding,
        "//",
        "////////////////////////////////////////////////////////////////"
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}
