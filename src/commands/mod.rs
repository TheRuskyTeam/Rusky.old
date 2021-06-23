use serenity::framework::standard::macros::group;
pub mod information;
mod utils;
use information::*;
use utils::*;
#[group]
#[summary("Comandos de informação")]
#[commands(ping, botinfo, covidstatus)]
struct Information;
#[group]
#[summary("Comandos para ajudar")]
#[commands(userinfo, calc)]
struct Utils;
