use std::io;
use owo_colors::OwoColorize;

pub fn name_choose(){
    // Player choose name
    let mut name = String::new();
    println!("What's your name ?");
    match io::stdin().read_line(&mut name) {
        Ok(_) => {
            println!("Your character is called {name} !", name = name.bright_yellow());
        }
        Err(error) => println!("error: {error}", error = error.bright_red()),
    }
    return;
}