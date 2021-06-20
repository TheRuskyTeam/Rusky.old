use serenity::framework::standard::macros::group;
pub mod information;
mod utils;
use information::*;
use utils::*;

#[group]
#[description("Comandos de informação")]
#[commands(ping, botinfo)]
struct Information;
#[group]
#[description("Comandos para ajudar")]
#[commands(userinfo)]
struct Utils;
