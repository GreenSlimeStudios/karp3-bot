use async_recursion::async_recursion;
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
#[commands(ping, dekarpdelaspecial, meme, calculate, calculate_verbose, bongal)]
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
    // let token = std::env::var("KARPTOKEN").expect("token");
    let token = std::fs::read_to_string("token.txt").expect("The file could not be read");
    // let token = "MTAyMDcyMzU0Nzg3NTg0MDAzMA.GsS0y-.4ARNaQCzYzpPDEY_yciDSrm2chqrdxoVSyYnK0";
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
    let args: Vec<String> = msg
        .content
        .split(" ")
        .skip(1)
        .map(|f| f.to_string())
        .collect();

    let mut result: f32 = 0.0;
    match calculate_section(&msg, &ctx, &args, false).await {
        Some(v) => {
            result = v;
        }
        None => {
            msg.channel_id
                .send_message(&ctx, |m| m.content("error running calculation"))
                .await?;
        }
    }

    // msg.channel_id
    //     .send_message(&ctx, |m| m.content(result.to_string()))
    //     .await?;
    msg.reply(&ctx, result.to_string()).await?;
    Ok(())
}

#[command]
async fn calculate_verbose(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<String> = msg
        .content
        .split(" ")
        .skip(1)
        .map(|f| f.to_string())
        .collect();

    let mut result: f32 = 0.0;
    match calculate_section(&msg, &ctx, &args, true).await {
        Some(v) => {
            result = v;
        }
        None => {
            msg.channel_id
                .send_message(&ctx, |m| m.content("error running calculation"))
                .await?;
        }
    }

    // msg.channel_id
    //     .send_message(&ctx, |m| m.content(result.to_string()))
    //     .await?;
    msg.reply(&ctx, result.to_string()).await?;
    Ok(())
}

#[async_recursion]
async fn calculate_section(
    msg: &Message,
    ctx: &Context,
    suply_args: &Vec<String>,
    verbose: bool,
) -> Option<f32> {
    let mut args: Vec<String> = suply_args.into_iter().map(|f| f.to_string()).collect();
    let mut result_f: f32 = 0.0;
    // println!("{:?}", args);
    while args.contains(&"(".to_string()) {
        let mut index_open: usize = 0;
        let mut open_counter: usize = 0;
        let mut close_counter: usize = 0;
        let mut index_close: usize = 0;
        for i in 0..args.len() {
            if args[i] == "(" {
                open_counter += 1;
                if open_counter == 1 {
                    index_open = i;
                }
            }
            if args[i] == ")" {
                close_counter += 1;
                if close_counter == open_counter {
                    index_close = i;
                    break;
                }
            }
        }
        // println!("open index at position: {}", index_open);
        // println!("close index at position: {}", index_close);

        let mut new_args: Vec<String> = args.clone().into_iter().skip(index_open + 1).collect();
        for _i in 0..(args.len() - index_close) {
            new_args.pop();
        }
        // println!("new args: {:?}", new_args);
        match calculate_section(&msg, &ctx, &new_args, verbose).await {
            Some(v) => {
                result_f = v;
            }
            None => (),
        };
        args[index_open] = result_f.to_string();

        // println!("args before () deletion: {:?}", args);
        for _i in 0..index_close - index_open {
            args.remove(index_open + 1);
        }
        // println!("args after () deletion: {:?}", args);
    }

    let mut result: f32 = 0.0;
    let mut numbers: Vec<f32> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    for i in 0..args.len() {
        match args[i].as_str() {
            "*" => operators.push(args[i].clone()),
            "+" => operators.push(args[i].clone()),
            "-" => operators.push(args[i].clone()),
            "/" => operators.push(args[i].clone()),
            _ => match args[i].parse::<f32>() {
                Ok(v) => numbers.push(v),
                Err(e) => {
                    match msg
                        .channel_id
                        .send_message(&ctx, |m| m.content(e.to_string()))
                        .await
                    {
                        Ok(_v) => (),
                        Err(e) => println!("error: {}", e),
                    }

                    return None;
                }
            },
        }
    }

    if numbers.len() != operators.len() + 1 {
        match msg.reply(&ctx, "invalid syntax").await {
            Ok(_v) => (),
            Err(e) => println!("error: {}", e),
        }

        return None;
    }
    // println!("{:?}", numbers);
    // println!("{:?}", operators);

    while operators.contains(&"*".to_string()) || operators.contains(&"/".to_string()) {
        let mut _index: usize = 0;
        let mut index_m: usize = 10000;
        let mut index_d: usize = 10000;
        let mut is_div: bool = true;
        for i in 0..operators.len() {
            if operators[i] == "*" {
                index_m = i;
                break;
            }
        }
        for i in 0..operators.len() {
            if operators[i] == "/" {
                index_d = i;
                break;
            }
        }
        if index_d < index_m {
            _index = index_d;
            // is_div = true;
        } else {
            _index = index_m;
            is_div = false;
        }

        let num: f32 = if is_div {
            numbers[_index] / numbers[_index + 1]
        } else {
            numbers[_index] * numbers[_index + 1]
        };

        if verbose {
            let message: String = if is_div { "dividing " } else { "multiplying " }.to_string()
                + numbers[_index].to_string().as_str()
                + " by "
                + numbers[_index + 1].to_string().as_str();
            match msg
                .channel_id
                .send_message(&ctx, |m| m.content(message))
                .await
            {
                Ok(_v) => (),
                Err(e) => println!("error: {}", e),
            }
        }

        numbers.remove(_index + 1);
        numbers[_index] = num;
        operators.remove(_index);
    }

    for i in 0..numbers.len() {
        if i == 0 {
            result = numbers[0];
            continue;
        }
        match operators[i - 1].as_str() {
            "-" => {
                if verbose {
                    let message: String = "substracting ".to_string()
                        + numbers[i].to_string().as_str()
                        + " from "
                        + result.to_string().as_str();
                    match msg
                        .channel_id
                        .send_message(&ctx, |m| m.content(message))
                        .await
                    {
                        Ok(_v) => (),
                        Err(e) => println!("error: {}", e),
                    }
                }
                result = result - numbers[i];
            }
            "+" => {
                if verbose {
                    let message: String = "adding ".to_string()
                        + numbers[i].to_string().as_str()
                        + " to "
                        + result.to_string().as_str();
                    match msg
                        .channel_id
                        .send_message(&ctx, |m| m.content(message))
                        .await
                    {
                        Ok(_v) => (),
                        Err(e) => println!("error: {}", e),
                    }
                }
                result = result + numbers[i];
            }
            _ => {
                result = result;
            }
        }
    }

    Some(result)
}

#[command]
async fn bongal(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let bongal: String = to_bongal(args[0].to_string());

    msg.reply(&ctx, bongal).await?;

    Ok(())
}

fn to_bongal(decimal: String) -> String {
    let mut decimal = decimal;
    let mut result: String = String::new();

    let mut value_int: u128 = 0;
    let mut value_after = 0.0;
    let is_under: bool = decimal.contains("-");
    if is_under {
        decimal.remove(0);
    }

    if decimal.contains(".") {
        let cropped_decimal: Vec<&str> = decimal.split(".").collect();
        value_int = cropped_decimal[0]
            .parse::<u128>()
            .expect("error parsing whole part of decimal to bongal");

        value_after = cropped_decimal[1]
            .parse::<f32>()
            .expect("error parsing the rest of decimal to bongal");
    } else {
        value_int = decimal
            .parse::<u128>()
            .expect("error parsing value to bongal");
    }
    if is_under {
        result += "-";
    }

    while value_int > 1 {
        println!("{}", value_int);
        result += match value_int % 27 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            10 => "A",
            11 => "B",
            12 => "C",
            13 => "D",
            14 => "E",
            15 => "F",
            16 => "G",
            17 => "H",
            18 => "I",
            19 => "J",
            20 => "K",
            21 => "L",
            22 => "M",
            23 => "N",
            24 => "O",
            25 => "P",
            26 => "R",
            27 => "S",
            _ => "U",
        };
        value_int = value_int / 27;
    }

    return result;
}

fn from_bongal(bongal: String) -> f32 {
    return 1.0;
}
