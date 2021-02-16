pub use abstract_game_match::IGameMatch;
use bye_match::ByeGameMatch;
pub use factories::{GameMatchCreator, GameMatchTransformer};
use normal_match::NormalGameMatch;
use unfinished_match::UnfinishedGameMatch;

mod abstract_game_match;
mod normal_match;
mod unfinished_match;
mod factories;
mod bye_match;

