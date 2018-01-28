//! A file that holds logical operations in order to run the game.

use interpreter::token::tokenize;
use protos::master::Game;
use screen::Interfaceable;

#[allow(unused_mut)]
pub fn eval<I: Interfaceable>(src: &mut I, mut game: Game) -> Result<Game, &str> {
    let token_string = src.prompt();
    let _tokens = tokenize(&token_string);
    Ok(game)
}
