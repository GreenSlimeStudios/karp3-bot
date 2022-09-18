use std::fmt::Debug;
// use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use chrono;
use rand::Rng;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
// use serenity::futures::TryFutureExt;
// use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(ping, dekarpdelaspecial, meme)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // We are verifying if the bot id is the same as the message author id.
        if msg.author.id != ctx.cache.current_user_id() {
            // Some lang actions
            let nick: String = match msg.author_nick(&ctx).await {
                Some(v) => v,
                None => {
                    println!("error reading nickname");
                    "error".to_string()
                }
            };
            // let channel_id = match msg.channel(&ctx).await {
            //     Some(v) => v,
            //     None => {
            //         println!("error reading channel");
            //         "error".to_string()
            //     }
            // };
            println!(
                "{} {}, {} - {}: {:?}",
                chrono::offset::Local::now(),
                msg.channel(&ctx).await.unwrap(),
                msg.author,
                nick,
                msg.content
            );
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open("logs.txt")
                .unwrap();

            if let Err(e) = writeln!(
                file,
                "{} {}, {} - {}: {:?}",
                chrono::offset::Local::now(),
                msg.channel(&ctx).await.unwrap(),
                msg.author,
                nick,
                msg.content
            ) {
                eprintln!("Couldn't write to file: {}", e);
            }
        }
    }
    // Since data is located in Context, this means you are also able to use it within events!
}

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
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn dekarpdelaspecial(ctx: &Context, msg: &Message) -> CommandResult {
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
#[command]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    let resp = reqwest::get("https://reddit-meme-api.herokuapp.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // .text()?;
    let urls: Vec<&str> = resp.split("https://").collect();
    let mut url = "https://".to_string() + urls[urls.len() - 2];
    url = url.replace("\",\"", "");
    // url = url.replace("%..", "");
    msg.reply(&ctx, &url).await?;
    println!("reply: {}", &url);

    Ok(())
}
