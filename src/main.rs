use player::class::*;
use player::stats::*;

// Modules Player
mod player;

#[derive(Default)]
struct GameState {
  pub player_name: Option<String>,
  pub player_class: Option<Class>,
  pub player_stats: Option<BaseStats>,
}

fn main() {
  let mut console_reader = std::io::stdin().lock();

  // load the state from json
  let mut state = GameState::default();

  // if state name is empty, get name
  let name = match state.player_name {
    Some(name) => name,
    None => {
      let name = player::name::name_choose();
      state.player_name = Some(name.clone());
      name
      // save state
    }
  };

  // if state class is empty, get class
  // Player Class
  let (class, _stats) = class_choose(&mut console_reader);
  println!("You have chosen the class: {class:?}")
}
