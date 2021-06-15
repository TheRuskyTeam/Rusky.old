use serenity::framework::standard::macros::group;
pub mod information;
use information::*;

#[group]
#[commands(ping)]
struct Information;
