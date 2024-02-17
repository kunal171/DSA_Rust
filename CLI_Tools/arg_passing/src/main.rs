use clap::{ arg, builder::Str, Command};

fn main() {
    let matches = Command::new("MyApp")
        .version("0.1.0")
        .author("Hackerman Jones <hckrmnjones@hack.gov>")
        .about("Teaches argument parsing")
        .arg(arg!(--file <VALUE>).required(true).value_name("A cool file"))
        .arg(arg!(--num <VALUE>).required(false).value_name("Five less than your favorite number"))
        .get_matches();

    let myfile = matches.get_one::<String>("file").expect("required");
    println!("The file passed is: {}", myfile);

    let num_str = matches.get_one::<String>("num");
    match num_str {
        None => println!("No idea what your favorite number is."),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Your favorite number must be {}.", n + 5),
                Err(_) => println!("That's not a number! {}", s),
            }
        }
    }
}
