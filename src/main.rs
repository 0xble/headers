use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");
    let padding = (0..(60 - input.len()) / 2).map(|_| " ").collect::<String>();

    let output = format!(
        "{}\n{}{}{}{}\n{}",
        "    ////////////////////////////////////////////////////////////////",
        "    //",
        padding.clone(),
        input.to_uppercase(),
        padding + "//",
        "    ////////////////////////////////////////////////////////////////"
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}
