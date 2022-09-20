use std::fs::OpenOptions;
use std::io::Write;
use std::result;

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
#[commands(ping, dekarpdelaspecial, meme, calculate)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // We are verifying if the bot id is the same as the message author id.
        let mut nick: String = "karp3".to_string();
        if msg.author.id != ctx.cache.current_user_id() {
            nick = match msg.author_nick(&ctx).await {
                Some(v) => v,
                None => {
                    println!("error reading nickname");
                    "error".to_string()
                }
            };
            // Some lang actions
        }
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
    let url_2: Vec<&str> = urls[urls.len() - 2].split("%").collect();
    let url_3: Vec<&str> = url_2[0].split("%").collect();

    let mut url = "https://".to_string() + url_3[0];
    url = url.replace("\",\"", "");
    // url = url.replace("%..", "");
    msg.reply(&ctx, &url).await?;
    // println!("reply: {}", &url);

    Ok(())
}
#[command]
async fn calculate(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    println!("{:?}", args);

    let mut result: f32 = 0.0;
    let mut numbers: Vec<f32> = Vec::new();
    let mut operators: Vec<&str> = Vec::new();

    for i in 0..args.len() {
        match args[i] {
            "*" => operators.push(args[i]),
            "+" => operators.push(args[i]),
            "-" => operators.push(args[i]),
            "/" => operators.push(args[i]),
            _ => match args[i].parse::<f32>() {
                Ok(v) => numbers.push(v),
                Err(e) => {
                    msg.channel_id
                        .send_message(&ctx, |m| m.content(e.to_string()))
                        .await?;
                    return Ok(());
                }
            },
        }
    }

    // msg.channel_id
    //     .send_message(&ctx, |m| m.content(numbers[0]))
    //     .await?;

    if numbers.len() != operators.len() + 1 {
        msg.channel_id
            .send_message(&ctx, |m| m.content("invalid syntax"))
            .await?;
        return Ok(());
    }
    println!("{:?}", numbers);
    println!("{:?}", operators);

    for i in 0..numbers.len() {
        if i == 0 {
            result = numbers[0];
            continue;
        }
        result = match operators[i - 1] {
            "*" => result * numbers[i],
            "+" => result + numbers[i],
            "-" => result - numbers[i],
            "/" => result / numbers[i],
            _ => result,
        }
    }
    msg.channel_id
        .send_message(&ctx, |m| m.content(result.to_string()))
        .await?;

    Ok(())
}
