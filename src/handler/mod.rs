use serenity::client::{Client, Context, EventHandler};
use serenity::{async_trait, futures::stream::Collect};
use std::collections;
use std::path::Path;
pub struct ChilliBot {
    commands: collections::VecDeque<String>,
    cooldowns: i32,
    aliases: String,
    owner: i32,
    prefix: u32,
}
