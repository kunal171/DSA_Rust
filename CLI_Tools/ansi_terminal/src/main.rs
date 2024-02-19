use ansi_term::{Style, Colour};

fn main() {
    //Printing colored text to the Terminal
    {
        println!("This is {} in color, {} in color and {} in color",
        Colour::Red.paint("red"),
        Colour::Blue.paint("blue"),
        Colour::Green.paint("green"));
    }

    //Bold text in Terminal
    {
        println!("{} and this is not",
        Style::new().bold().paint("This is Bold"));
    }

    //Bold and colored text in terminal
    {
        println!("{}, {} and {}",
             Colour::Yellow.paint("This is colored"),
             Style::new().bold().paint("this is bold"),
             Colour::Yellow.bold().paint("this is bold and colored"));
    }
}
    
