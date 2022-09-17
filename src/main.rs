// use std::env;

use rand::Rng;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
// use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping, sussy)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::default()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = "MTAyMDcyMzU0Nzg3NTg0MDAzMA.GsS0y-.4ARNaQCzYzpPDEY_yciDSrm2chqrdxoVSyYnK0";
    // let intents = GatewayIntents::privileged();
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    match client.start().await {
        Err(e) => println!("bad news :( error: {:?}", e),
        Ok(_v) => println!("we're in!"),
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("ping from {}", msg.author.id);
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn sussy(ctx: &Context, msg: &Message) -> CommandResult {
    println!("baka");
    msg.channel_id
        .send_message(&ctx, |m| m.content("baka"))
        .await
        .unwrap();
    for _i in 0..25 {
        let num: u8 = rand::thread_rng().gen_range(0..=1);
        match num {
            0 => {
                msg.reply(&ctx, "🗿").await?;
            }
            1 => {
                msg.reply(&ctx, "<@519553926958284800> fart").await?;
            }
            _ => (),
        }
    }
    Ok(())
}
