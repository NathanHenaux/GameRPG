use owo_colors::OwoColorize;
use std::io;

pub fn name_choose() -> String {
  // Player choose name
  let mut name = String::new();
  loop {
    println!("What's your name ?");
    match io::stdin().read_line(&mut name) {
      Ok(_) => {
        // todo : check if name is valid
        println!("Your character is called {name} !", name = name.bright_yellow());
        return name;
      }
      Err(error) => println!("error: {error}", error = error.bright_red()),
    }
  }
}

// todo : some tests
