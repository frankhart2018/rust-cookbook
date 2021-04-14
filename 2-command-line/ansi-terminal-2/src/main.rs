use ansi_term::{Colour, Style};

fn main() {
    // Colored text
    println!("This is {} in color, {} in color, {} in color",
             Colour::Red.paint("red"),
             Colour::Blue.paint("blue"),
             Colour::Green.paint("green"));

    // Bold text
    println!("{} and this is not",
              Style::new().bold().paint("This is Bold"));

    // Bold and colored text
    println!("{}, {} and {}",
             Colour::Yellow.paint("This is colred"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and coloured"));
}
