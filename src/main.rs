mod commands;
mod lists;

use lists::*;

use async_recursion::async_recursion;
use serenity::model::prelude::{Member, ReactionType, UserId};
use serenity::utils::parse_username;
use songbird::SerenityInit;
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

use chrono;
use chrono::{Duration, Utc};
use rand::Rng;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{Args, CommandResult, Delimiter, StandardFramework};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use serpapi_search_rust::serp_api_search::SerpApiSearch;

mod user;
use sqlx::Row;
use user::DcUser;

#[group]
#[commands(
    ping,
    // dekarpdelaspecial,
    help,
    meme,
    calculate,
    calculate27,
    calculate_verbose,
    bongal,
    decimal,
    ksearch,
    jumpscare,
    dm,
    activity,
    aaa,
    join,
    leave,
    play,
    play2,
    stop,
    ryndyndyn,
    is_deafned,
    moc,
    huj,
    russian_roulette,
    writein,
    bet,
    update_moc,
    moce,
)]
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
                    // println!("error reading nickname");
                    msg.author.name.clone()
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

        let cat = ReactionType::try_from("<:oo:1044950628549537882>").unwrap();
        let dripczak = ReactionType::try_from("<:dripczak:1026584223492096080>").unwrap();
        let _the_rock = ReactionType::try_from("<:the_rock:982328750240833536>").unwrap();
        let the_beans = ReactionType::try_from("<:the_beans:1009140627272900648>").unwrap();
        let jaslo = ReactionType::try_from("<:jaslo:1044185772556812288>").unwrap();
        let igor = ReactionType::try_from("<:igor:1045744129625292842>").unwrap();

        if msg.is_own(&ctx) {
            return;
        }

        if msg.content.to_lowercase().contains("cześć")
            && msg.content.to_lowercase().contains("chłop")
        {
            match msg.react(&ctx, igor.clone()).await {
                Ok(_v) => (),
                Err(e) => {
                    println!("error adding igor reaction: {}", e);
                }
            }
            match msg
                .reply(
                    &ctx,
                    "<:igor:1045744129625292842> Cześć chłopczyku :leftwards_hand: :smirk:",
                )
                .await
            {
                Ok(_) => (),
                Err(_) => (),
            }
        }

        for word in get_sus_list() {
            if msg
                .content
                .to_lowercase()
                .contains(word.to_lowercase().as_str())
            {
                if rand::thread_rng().gen_range(0..3) == 0 {
                    match msg
                        .channel_id
                        .send_message(&ctx, |m| {
                            m.content("<:the_rock:982328750240833536>".to_string())
                        })
                        .await
                    {
                        Ok(_v) => (),
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }

        for word in get_cat_list() {
            if msg
                .content
                .to_lowercase()
                .contains(word.to_lowercase().as_str())
            {
                match msg.react(&ctx, cat.clone()).await {
                    Ok(_v) => (),
                    Err(e) => {
                        println!("error adding cat reaction: {}", e);
                    }
                }
            }
        }
        for word in get_thick_list() {
            if msg
                .content
                .to_lowercase()
                .contains(word.to_lowercase().as_str())
            {
                match msg.react(&ctx, the_beans.clone()).await {
                    Ok(_v) => (),
                    Err(e) => {
                        println!("error adding jaslo reaction: {}", e);
                    }
                }
                match msg.react(&ctx, jaslo.clone()).await {
                    Ok(_v) => (),
                    Err(e) => {
                        println!("error adding jaslo reaction: {}", e);
                    }
                }
            }
        }

        for word in get_drip_list() {
            if msg
                .content
                .to_lowercase()
                .contains(word.to_lowercase().as_str())
            {
                match msg.react(&ctx, dripczak.clone()).await {
                    Ok(_v) => (),
                    Err(e) => {
                        println!("error adding drip reaction: {}", e);
                    }
                }
            }
        }

        let url = "postgres://dbuser:sprzedamopla@localhost:5432/postgres";
        let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
        // sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let id: String = msg.author.id.as_u64().to_string();
        println!("id: {id}");
        let mut dcuser: DcUser = DcUser::new(id);
        dcuser.get_user_data_or_create_user(&pool).await;

        if !is_dm(&msg, &ctx).await {
            dcuser.handle_passive_income(&pool).await;
        }
    }
    async fn ready(&self, ctx: Context, _ready: Ready) {
        println!("{} is connected!", _ready.user.name);

        let guild_ids = _ready.user.guilds(&ctx).await.unwrap();
        for guild_id in &guild_ids {
            let _commands =
                GuildId::set_application_commands(&guild_id.id, &ctx.http, |commands| {
                    commands
                        .create_application_command(|command| commands::ping::register(command))
                        .create_application_command(|command| commands::yum::register(command))
                        .create_application_command(|command| {
                            commands::numberinput::register(command)
                        })
                        .create_application_command(|command| {
                            commands::karp_search::register(command)
                        })
                })
                .await;

            // println!(
            //     "I now have the following guild slash commands: {:#?}",
            //     _commands
            // );
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                "yum" => commands::yum::run(&command.data.options),
                "rock" => commands::yum::run(&command.data.options),
                "numberinput" => commands::numberinput::run(&command.data.options),
                "karp-search" => commands::karp_search::run(&command.data.options).await,
                _ => "not implemented :(".to_string(),
            };

            // Runtime::new().unwrap();
            // let content = search("rust", 3, None).await.unwrap().len().to_string();

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // let url="postgres://dbuser:sprzedamopla@localhost:5432/postgres";
    // let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
    // sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    tracing_subscriber::fmt::init();
    let framework = StandardFramework::default()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    // let token = std::env::var("KARPTOKEN").expect("token");
    let token = std::fs::read_to_string("token.txt").expect("The file could not be read");
    // let intents = GatewayIntents::privileged();
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    match client.start().await {
        Err(e) => println!("bad news :( error: {:?}", e),
        Ok(_v) => println!("we're in!"),
    }
}

#[command]
async fn jumpscare(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "https://cdn.discordapp.com/attachments/1043236508603265148/1079377473353023499/fishjumpscare.mp4").await?;

    Ok(())
}
#[command]
async fn activity(ctx: &Context, msg: &Message) -> CommandResult {
    let guildid = msg.guild_id.unwrap();
    let guild = guildid.to_guild_cached(&ctx).unwrap();
    let presences = guild.presences;
    for (id, presence) in presences {
        if presence.activities.is_empty() {
            continue;
        }
        let mut con = id.to_user(&ctx).await.unwrap().name + ": ";
        for activity in presence.activities {
            con += activity.name.as_str();
            match activity.details {
                Some(v) => {
                    con += " (";
                    con += v.as_str();
                    con += ")";
                }
                None => (),
            }
            con += ", ";
            if activity.name.to_lowercase() == "league of legends" {
                match guildid.ban(&ctx, id, 0).await {
                    Ok(_) => {
                        let ban_message = "Banned ".to_string()
                            + id.to_user(&ctx).await.unwrap().name.as_str()
                            + " for playing Leauge Of Legends";
                        msg.channel_id
                            .send_message(&ctx, |m| m.content(ban_message))
                            .await
                            .unwrap();
                    }
                    Err(e) => {
                        let ban_message = "Failed banning ".to_string()
                            + id.to_user(&ctx).await.unwrap().name.as_str()
                            + " for playing Leauge Of Legends {"
                            + e.to_string().as_str()
                            + "}";
                        msg.channel_id
                            .send_message(&ctx, |m| m.content(ban_message))
                            .await
                            .unwrap();
                    }
                };
            }
        }
        msg.channel_id
            .send_message(&ctx, |m| m.content(con))
            .await
            .unwrap();
    }
    Ok(())
}
#[command]
async fn aaa(ctx: &Context, msg: &Message) -> CommandResult {
    let guildid = msg.guild_id.unwrap();
    let _role = guildid
        .create_role(&ctx, |r| r.hoist(true).name("aaa"))
        .await;

    Ok(())
}
#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let mess = msg
        .channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| e.title("Pong").description("Pong"))
                .components(|c| {
                    c.create_action_row(|row| row.create_button(|cb| cb.label("Pong").custom_id(1)))
                })
        })
        .await?;
    let i = mess.await_component_interaction(&ctx).await;
    match i {
        Some(v) => {
            println!("{:?}", v.data.custom_id);
        }
        None => (),
    }
    msg.reply(&ctx, "pong!").await?;

    Ok(())
}
#[command]
async fn dm(ctx: &Context, msg: &Message) -> CommandResult {
    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    msg.delete(&ctx).await?;
    let mut message: String = String::new();
    if words.len() > 1 {
        let userid: u64;
        if words[0].starts_with("<@") {
            userid = parse_username(words[0]).unwrap();
        } else {
            userid = words[0].parse::<u64>()?;
        }
        for i in 1..words.len() {
            message += words[i];
            message += " ";
        }
        let user = UserId(userid).to_user(&ctx).await;
        match user {
            Ok(user) => {
                user.direct_message(&ctx, |m| m.content(message)).await?;
            }
            Err(e) => {
                msg.channel_id
                    .send_message(&ctx, |m| m.content(e))
                    .await
                    .unwrap();
            }
        }
    } else {
        msg.channel_id
            .send_message(&ctx, |m| m.content("Not valid input data"))
            .await
            .unwrap();
    }
    Ok(())
}
#[command]
async fn tr(ctx: &Context, msg: &Message) -> CommandResult {
    // let user: User = User::from(UserId(2000));
    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    if words.len() > 0 {
        let userid = parse_username(words[0]);
        match userid {
            Some(id) => {
                let user = UserId(id).to_user(&ctx).await.unwrap();
                msg.reply(&ctx, user.avatar_url().unwrap()).await?;
                return Ok(());
            }
            None => (),
        }
    }
    msg.reply(ctx, msg.author.avatar_url().unwrap()).await?;

    Ok(())
}
#[command]
async fn ksearch(ctx: &Context, msg: &Message) -> CommandResult {
    let parameters: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let mut q: String = String::new();
    for i in 0..(parameters.len() - 1) {
        q += parameters[i + 1];
    }
    let api_key: String =
        std::fs::read_to_string("search_token.txt").expect("The file could not be read");

    let mut params = std::collections::HashMap::<String, String>::new();
    params.insert("q".to_string(), q);
    params.insert("location".to_string(), "Poland".to_string());

    let search = SerpApiSearch::google(params, api_key);

    let results = search.json().await.unwrap();
    let organic_results: &Vec<serde_json::Value>;

    match results["organic_results"].as_array() {
        Some(v) => {
            organic_results = v;
        }
        None => {
            msg.reply(&ctx, "no search results found".to_string())
                .await?;
            return Ok(());
        }
    }
    let mut message: String = String::new();

    for i in 0..parameters
        .first()
        .unwrap()
        .parse::<usize>()
        .unwrap()
        .to_string()
        .parse::<usize>()
        .unwrap()
    {
        if i < organic_results.len() {
            message += organic_results[i]["link"].clone().to_string().as_str();
        }
    }

    msg.reply(&ctx, message.replace("\"\"", "\n").replace("\"", "").trim())
        .await?;

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

    let mut result: f64 = 0.0;
    match calculate_section(&msg, &ctx, &args, false, false).await {
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

    let mut result: f64 = 0.0;
    match calculate_section(&msg, &ctx, &args, true, false).await {
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
async fn calculate27(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<String> = msg
        .content
        .split(" ")
        .skip(1)
        .map(|f| f.to_string())
        .collect();

    let mut result: f64 = 0.0;
    match calculate_section(&msg, &ctx, &args, false, true).await {
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
    msg.reply(
        &ctx,
        match to_bongal(result.to_string()) {
            Some(v) => v,
            None => "error".to_string(),
        },
    )
    .await?;
    Ok(())
}

#[async_recursion]
async fn calculate_section(
    msg: &Message,
    ctx: &Context,
    suply_args: &Vec<String>,
    verbose: bool,
    is_bongal: bool,
) -> Option<f64> {
    let mut args: Vec<String> = suply_args.into_iter().map(|f| f.to_string()).collect();
    let mut result_f: f64 = 0.0;
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
        match calculate_section(&msg, &ctx, &new_args, verbose, is_bongal).await {
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

    let mut result: f64 = 0.0;
    let mut numbers: Vec<f64> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    for i in 0..args.len() {
        match args[i].as_str() {
            "*" => operators.push(args[i].clone()),
            "+" => operators.push(args[i].clone()),
            "-" => operators.push(args[i].clone()),
            "/" => operators.push(args[i].clone()),
            _ => {
                if is_bongal {
                    match from_bongal(args[i].clone()) {
                        Some(v) => numbers.push(v),
                        None => {
                            msg.reply(&ctx, "invalid bongal->decimal conversion")
                                .await
                                .unwrap();
                            return None;
                        }
                    }
                    // numbers.push(from_bongal(args[i].clone()));
                } else {
                    match args[i].parse::<f64>() {
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
                    }
                }
            }
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

        let num: f64 = if is_div {
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
    let bongal: String;
    match to_bongal(args[0].to_string()) {
        Some(v) => {
            bongal = v;
        }
        None => {
            msg.reply(&ctx, "invalid bongal").await.unwrap();
            return Ok(());
        }
    }

    msg.reply(&ctx, bongal).await?;

    Ok(())
}
#[command]
async fn decimal(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let decimal: f64;

    match from_bongal(args[0].to_string()) {
        Some(v) => decimal = v,
        None => {
            msg.reply(&ctx, "invalid bongal->decimal conversion")
                .await
                .unwrap();
            return Ok(());
        }
    }
    msg.reply(&ctx, decimal).await?;

    Ok(())
}

fn to_bongal(decimal: String) -> Option<String> {
    let mut decimal = decimal;
    let mut result: String = String::new();

    let mut value_int: u128;
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

        value_after = ("0.".to_string() + cropped_decimal[1])
            .as_str()
            .parse::<f64>()
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
        // println!("{}", value_int);
        match value_int % 27 {
            0 => result += "0",
            1 => result += "1",
            2 => result += "2",
            3 => result += "3",
            4 => result += "4",
            5 => result += "5",
            6 => result += "6",
            7 => result += "7",
            8 => result += "8",
            9 => result += "9",
            10 => result += "α",
            11 => result += "β",
            12 => result += "γ",
            13 => result += "δ",
            14 => result += "ρ",
            15 => result += "F",
            16 => result += "η",
            17 => result += "∅",
            18 => result += "c",
            19 => result += "K",
            20 => result += "ʎ",
            21 => result += "u",
            22 => result += "V",
            23 => result += "Ś",
            24 => result += "O",
            25 => result += "π",
            26 => result += "P",
            _ => return None,
        }
        value_int = value_int / 27;
    }
    let result_arr: Vec<String> = result.chars().map(|f| f.to_string()).rev().collect();
    result = "".to_string();
    for i in 0..result_arr.len() {
        result += result_arr[i].as_str();
    }

    if decimal.contains(".") {
        result += ".";
        for _i in 0..5 {
            value_after *= 27.0;

            let value_after_string: String = value_after.to_string().clone();
            let value_str: Vec<&str> = value_after_string.split(".").collect();

            // println!("{value_after} {}", value_str[0]);

            match value_str[0] {
                "0" => result += "0",
                "1" => result += "1",
                "2" => result += "2",
                "3" => result += "3",
                "4" => result += "4",
                "5" => result += "5",
                "6" => result += "6",
                "7" => result += "7",
                "8" => result += "8",
                "9" => result += "9",
                "10" => result += "α",
                "11" => result += "β",
                "12" => result += "γ",
                "13" => result += "δ",
                "14" => result += "ρ",
                "15" => result += "F",
                "16" => result += "η",
                "17" => result += "∅",
                "18" => result += "c",
                "19" => result += "K",
                "20" => result += "ʎ",
                "21" => result += "u",
                "22" => result += "V",
                "23" => result += "Ś",
                "24" => result += "O",
                "25" => result += "π",
                "26" => result += "P",
                _ => return None,
            }
            value_after = value_after - value_after.floor();
        }
    }

    return Some(result);
}

fn from_bongal(bongal: String) -> Option<f64> {
    let mut result: f64 = 0.0;
    let mut is_below = false;
    // if is_below{

    // }
    let mut chars_after: Vec<String> = Vec::new();
    let mut chars_int: Vec<String>;
    if bongal.contains(".") {
        let cropped: Vec<&str> = bongal.split(".").collect();
        chars_int = cropped[0].chars().map(|f| f.to_string()).collect();
        chars_after = cropped[1].chars().map(|f| f.to_string()).collect();
    } else {
        chars_int = bongal.chars().map(|f| f.to_string()).collect();
    }
    if bongal.contains(&"-") {
        is_below = true;
        chars_int.remove(0);
    }
    chars_int.reverse();

    for i in 0..chars_int.len() {
        match chars_int[i].as_str() {
            "0" => result += 0.0 * (27u64.pow(i as u32)) as f64,
            "1" => result += 1.0 * (27u64.pow(i as u32)) as f64,
            "2" => result += 2.0 * (27u64.pow(i as u32)) as f64,
            "3" => result += 3.0 * (27u64.pow(i as u32)) as f64,
            "4" => result += 4.0 * (27u64.pow(i as u32)) as f64,
            "5" => result += 5.0 * (27u64.pow(i as u32)) as f64,
            "6" => result += 6.0 * (27u64.pow(i as u32)) as f64,
            "7" => result += 7.0 * (27u64.pow(i as u32)) as f64,
            "8" => result += 8.0 * (27u64.pow(i as u32)) as f64,
            "9" => result += 9.0 * (27u64.pow(i as u32)) as f64,
            "α" => result += 10.0 * (27u64.pow(i as u32)) as f64,
            "β" => result += 11.0 * (27u64.pow(i as u32)) as f64,
            "γ" => result += 12.0 * (27u64.pow(i as u32)) as f64,
            "δ" => result += 13.0 * (27u64.pow(i as u32)) as f64,
            "ρ" => result += 14.0 * (27u64.pow(i as u32)) as f64,
            "F" => result += 15.0 * (27u64.pow(i as u32)) as f64,
            "η" => result += 16.0 * (27u64.pow(i as u32)) as f64,
            "∅" => result += 17.0 * (27u64.pow(i as u32)) as f64,
            "c" => result += 18.0 * (27u64.pow(i as u32)) as f64,
            "K" => result += 19.0 * (27u64.pow(i as u32)) as f64,
            "ʎ" => result += 20.0 * (27u64.pow(i as u32)) as f64,
            "u" => result += 21.0 * (27u64.pow(i as u32)) as f64,
            "V" => result += 22.0 * (27u64.pow(i as u32)) as f64,
            "Ś" => result += 23.0 * (27u64.pow(i as u32)) as f64,
            "O" => result += 24.0 * (27u64.pow(i as u32)) as f64,
            "π" => result += 25.0 * (27u64.pow(i as u32)) as f64,
            "P" => result += 26.0 * (27u64.pow(i as u32)) as f64,
            _ => {
                return None;
            }
        }
        // println!("from {}", chars_int[i]);
    }

    for i in 0..chars_after.len() {
        match chars_after[i].as_str() {
            "1" => result += 1.0 / (27u64.pow(i as u32 + 1)) as f64,
            "2" => result += 2.0 / (27u64.pow(i as u32 + 1)) as f64,
            "3" => result += 3.0 / (27u64.pow(i as u32 + 1)) as f64,
            "4" => result += 4.0 / (27u64.pow(i as u32 + 1)) as f64,
            "5" => result += 5.0 / (27u64.pow(i as u32 + 1)) as f64,
            "6" => result += 6.0 / (27u64.pow(i as u32 + 1)) as f64,
            "7" => result += 7.0 / (27u64.pow(i as u32 + 1)) as f64,
            "8" => result += 8.0 / (27u64.pow(i as u32 + 1)) as f64,
            "9" => result += 9.0 / (27u64.pow(i as u32 + 1)) as f64,
            "α" => result += 10.0 / (27u64.pow(i as u32 + 1)) as f64,
            "β" => result += 11.0 / (27u64.pow(i as u32 + 1)) as f64,
            "γ" => result += 12.0 / (27u64.pow(i as u32 + 1)) as f64,
            "δ" => result += 13.0 / (27u64.pow(i as u32 + 1)) as f64,
            "ρ" => result += 14.0 / (27u64.pow(i as u32 + 1)) as f64,
            "F" => result += 15.0 / (27u64.pow(i as u32 + 1)) as f64,
            "η" => result += 16.0 / (27u64.pow(i as u32 + 1)) as f64,
            "∅" => result += 17.0 / (27u64.pow(i as u32 + 1)) as f64,
            "c" => result += 18.0 / (27u64.pow(i as u32 + 1)) as f64,
            "K" => result += 19.0 / (27u64.pow(i as u32 + 1)) as f64,
            "ʎ" => result += 20.0 / (27u64.pow(i as u32 + 1)) as f64,
            "u" => result += 21.0 / (27u64.pow(i as u32 + 1)) as f64,
            "V" => result += 22.0 / (27u64.pow(i as u32 + 1)) as f64,
            "Ś" => result += 23.0 / (27u64.pow(i as u32 + 1)) as f64,
            "O" => result += 24.0 / (27u64.pow(i as u32 + 1)) as f64,
            "π" => result += 25.0 / (27u64.pow(i as u32 + 1)) as f64,
            "P" => result += 26.0 / (27u64.pow(i as u32 + 1)) as f64,
            _ => {
                return None;
            }
        }
        // println!("from {}", chars_int[i]);
    }
    if is_below {
        result *= -1.0;
    }
    return Some(result);
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    join_fn(&ctx, &msg).await
}
async fn join_fn(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            msg.reply(ctx, "Not in a voice channel").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let _handler = manager.join(guild_id, connect_to).await;

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            msg.channel_id
                .say(&ctx.http, format!("Failed: {:?}", e))
                .await?;
        }

        msg.channel_id.say(&ctx.http, "Left voice channel").await?;
    } else {
        msg.reply(ctx, "Not in a voice channel").await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    play_fn(&ctx, &msg, args).await
}
async fn play_fn(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Must provide a URL to a video or audio")
                .await?;
            return Ok(());
        }
    };

    if !url.starts_with("http") {
        msg.channel_id
            .say(&ctx.http, "Must provide a valid URL")
            .await?;
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id
                    .say(&ctx.http, "Error sourcing ffmpeg")
                    .await?;

                return Ok(());
            }
        };

        handler.play_source(source);

        msg.channel_id.say(&ctx.http, "Playing song").await?;
    } else {
        msg.channel_id
            .say(&ctx.http, "Not in a voice channel to play in")
            .await?;
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play2(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    play2_fn(&ctx, &msg, args).await
}
async fn play2_fn(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    join_fn(&ctx, &msg).await.unwrap();
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            msg.channel_id
                .say(&ctx.http, "Must provide a URL to a video or audio")
                .await?;
            return Ok(());
        }
    };
    let mut cmd = Command::new("yt-dlp");
    cmd.arg("--get-url");
    cmd.arg("-f 140");
    cmd.arg(url);
    let url: String;
    match cmd.output() {
        Ok(o) => {
            url = String::from_utf8(o.stdout).unwrap().trim().to_string();
            // println!("{}", String::from_utf8_unchecked(o.stdout));
        }
        Err(e) => {
            println!("error: {e}");
            url = "".to_string();
        }
    }
    println!("{url}");

    if !url.starts_with("http") {
        msg.channel_id
            .say(&ctx.http, "Must provide a valid URL")
            .await?;
        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(&url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                msg.channel_id
                    .say(&ctx.http, "Error sourcing ffmpeg")
                    .await?;

                return Ok(());
            }
        };

        handler.play_source(source);

        msg.channel_id.say(&ctx.http, "gut").await?;
    } else {
        msg.channel_id
            .say(&ctx.http, "Not in a voice channel to play in")
            .await?;
    }

    Ok(())
}
#[command]
async fn ryndyndyn(ctx: &Context, msg: &Message) -> CommandResult {
    join_fn(&ctx, &msg).await?;
    play2_fn(
        &ctx,
        &msg,
        Args::new(
            "https://www.youtube.com/watch?v=EgAOqt8I5ac",
            &[Delimiter::Single(' ')],
        ),
    )
    .await
}
#[command]
async fn stop(ctx: &Context, msg: &Message) -> CommandResult {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    if let Some(handler_lock) = manager.get(msg.guild_id.unwrap()) {
        let mut handler = handler_lock.lock().await;
        handler.stop();
    }
    Ok(())
}
#[command]
async fn is_deafned(ctx: &Context, msg: &Message) -> CommandResult {
    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    if let Some(handler_lock) = manager.get(msg.guild_id.unwrap()) {
        let handler = handler_lock.lock().await;
        msg.reply(&ctx, handler.is_deaf()).await?;
    }
    Ok(())
}

#[command]
async fn moc(ctx: &Context, msg: &Message) -> CommandResult {
    let url = "postgres://dbuser:sprzedamopla@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();
    // sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // let id: String = msg.author.id.as_u64().to_string();
    let id: String;
    let przed: String;

    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    if words.len() > 0 {
        let userid = parse_username(words[0]);
        match userid {
            Some(uid) => {
                id = uid.to_string();
                przed = "ten użytkownik ma ".to_string();
            }
            None => {
                id = msg.author.id.as_u64().to_string();
                przed = "masz ".to_string();
            }
        }
    } else {
        id = msg.author.id.as_u64().to_string();
        przed = "masz ".to_string();
    }

    println!("id: {id}");
    let mut dcuser: DcUser = DcUser::new(id);
    dcuser.get_user_data_or_create_user(&pool).await;

    msg.reply(
        &ctx,
        przed + dcuser.power.to_string().as_str() + " milionów mocy",
    )
    .await?;

    Ok(())
}
#[command]
async fn moce(ctx: &Context, msg: &Message) -> CommandResult {
    let url = "postgres://dbuser:sprzedamopla@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await.unwrap();

    let q = "SELECT * FROM db_users ORDER BY power DESC";
    let query = sqlx::query(q);

    let rows = query.fetch_all(&pool).await;
    match rows {
        Ok(r) => {
            let mut mess: String = "".to_string();
            for row in r {
                let pow: i64 = row.get("power");
                let id: String = row.get("id");
                mess += format!("<@{}>  {}\n", id, pow).as_str();
            }
            msg.channel_id
                .send_message(&ctx, |m| m.content(mess))
                .await?;
        }
        Err(e) => {
            // self.create_user(pool).await;
            msg.reply(&ctx, e).await?;
        }
    }

    let id: String;
    let przed: String;

    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    if words.len() > 0 {
        let userid = parse_username(words[0]);
        match userid {
            Some(uid) => {
                id = uid.to_string();
                przed = "ten użytkownik ma ".to_string();
            }
            None => {
                id = msg.author.id.as_u64().to_string();
                przed = "masz ".to_string();
            }
        }
    } else {
        id = msg.author.id.as_u64().to_string();
        przed = "masz ".to_string();
    }

    println!("id: {id}");
    let mut dcuser: DcUser = DcUser::new(id);
    dcuser.get_user_data_or_create_user(&pool).await;

    msg.reply(
        &ctx,
        przed + dcuser.power.to_string().as_str() + " milionów mocy",
    )
    .await?;

    Ok(())
}

#[command]
async fn huj(ctx: &Context, msg: &Message) -> CommandResult {
    let members: Vec<Member> = msg
        .guild_id
        .unwrap()
        .members(&ctx, Some(1000), None)
        .await
        .unwrap()
        .into_iter()
        .filter(|m| m.user.bot == false)
        .collect();

    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    if words.len() > 0 {
        let res = words[0].parse::<u8>();
        match res {
            Ok(mut n) => {
                if n > 6 {
                    n = 6;
                }
                for _ in 0..n {
                    let num = rand::thread_rng().gen_range(0..members.len());
                    msg.channel_id
                        .send_message(&ctx, |m| {
                            m.content(format!("<@{}>", members[num].user.id.as_u64()))
                        })
                        .await?;
                }
                return Ok(());
            }
            Err(_) => (),
        }
    }
    let num = rand::thread_rng().gen_range(0..members.len());
    msg.channel_id
        .send_message(&ctx, |m| {
            m.content(format!("<@{}>", members[num].user.id.as_u64()))
        })
        .await?;

    Ok(())
}
#[command]
async fn russian_roulette(ctx: &Context, msg: &Message) -> CommandResult {
    if is_dm(msg, ctx).await {
        msg.reply(&ctx, "you can only use that command in a server")
            .await?;
        return Ok(());
    }

    let url = "postgres://dbuser:sprzedamopla@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await;
    match pool {
        Ok(p) => {
            let id: String = msg.author.id.as_u64().to_string();
            let mut dcuser: DcUser = DcUser::new(id);
            dcuser.get_user_data_or_create_user(&p).await;

            dcuser.power += 25;
            dcuser.update_user(&p).await;
        }
        Err(e) => {
            msg.reply(&ctx, format!("error: {e}")).await?;
            return Ok(());
        }
    }

    let members = msg
        .guild_id
        .unwrap()
        .members(&ctx, Some(1000), None)
        .await
        .unwrap();
    let num = rand::thread_rng().gen_range(0..6) as u8;

    if num == 5 {
        msg.reply(
            &ctx,
            "You hit the jackpot! 🔫 <:bronnie:1088395064440533012>",
        )
        .await?;

        let now = Utc::now();
        let time: String = (now + Duration::hours(24)).to_rfc3339();
        members
            .into_iter()
            .find(|x| x.user.id.as_u64() == msg.author.id.as_u64())
            .unwrap()
            .edit(&ctx, |m| m.disable_communication_until(time))
            .await?;
        // msg.guild_id.unwrap().ban
    } else {
        msg.reply(&ctx, "nothing happned...").await?;
    }

    // sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let url = UserId(1020723547875840030)
        .to_user(&ctx)
        .await
        .unwrap()
        .avatar_url()
        .unwrap();
    msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| e
            .colour(0x00ff00)
            // .thumbnail(profile.avatar.clone())
            .thumbnail(url)
            .url("https://www.youtube.com/watch?v=dQw4w9WgXcQ")
            .title(format!("Help from glorious karp"))
            .field("ping", "check if karp is alive", false)
            .field("help", "displays this command", false)
            .field("calculate <expression>", "calculates de thing in decimal values. Put space between every character exept for numbers", false)
            .field("calculate27 <expression>", "calculates de thing in bongal values. Put space between every character exept for numbers", false)
            .field("calculate_verbose <expression>", "same as calculate command but documents every step of the process", false)
            .field("bongal <decimal>", "converts decimal value into bongal value", false)
            .field("decimal <bongal>", "converts bongal value into decimal value", false)
            .field("tr [@user]", "returns the profile picture of you or the person specified", false)
            .field("ksearch <result count> <query>", "searches the web", false)
            .field("jumpscare", "jumpscare", false)
            .field("dm <@user> <message>", "sends an annonymous message to the person specified", false)
            .field("activity", "returns the activities of people in the server", false)
            .field("join", "joins the vc channel you are in", false)
            .field("leave", "leaves all vc channels on the server", false)
            .field("stop", "stops playing any current audio", false)
            .field("play <url>", "plays the audio from an url that leads to an audio file", false)
            .field("play2 <url>", "plays the audio from an url that leads to a youtube video", false)
            .field("ryndyndyn", "bad piggies theme song intensifies", false)
            .field("moc [@user]", "returns your or the person's specified current power lever", false)
            .field("huj [n]", "pings [n] number random members of the server n<=6", false)
            .field("russian_roulette", "you have a 1/6 chance to be muted for 24h, adds 25 power", false)
            .field("bet <power> <black/white>", "50/50 chance to gain/loose specified power", false)
        )
    }).await?;

    Ok(())
}
#[command]
async fn writein(ctx: &Context, msg: &Message) -> CommandResult {
    // msg.reply(&ctx, &msg.content).await?;
    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    if words.len() > 1 {
        let channel = ChannelId(words[0].parse::<u64>().unwrap());
        let msg_words: Vec<&str> = words.into_iter().skip(1).collect();
        let mut mess: String = "".to_string();
        for word in msg_words {
            mess += word;
        }
        channel.send_message(&ctx, |m| m.content(mess)).await?;
    }

    Ok(())
}
#[command]
async fn update_moc(ctx: &Context, msg: &Message) -> CommandResult {
    if *msg.author.id.as_u64() != 519553926958284800u64 {
        msg.reply(&ctx, "You dont have permission to run this command")
            .await?;
        return Ok(());
    }
    // msg.reply(&ctx, &msg.content).await?;
    let url = "postgres://dbuser:sprzedamopla@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await;
    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    match pool {
        Ok(pool) => {
            let id: &str = words[0];
            let moc: i64 = words[1].parse::<i64>().unwrap();
            let mut dcuser: DcUser = DcUser::new(id.to_string());
            dcuser.get_user_data_or_create_user(&pool).await;

            dcuser.power += moc;
            dcuser.update_user(&pool).await;
        }
        Err(e) => {
            msg.reply(&ctx, format!("error: {e}")).await?;
            return Ok(());
        }
    }

    Ok(())
}
#[command]
async fn bet(ctx: &Context, msg: &Message) -> CommandResult {
    let url = "postgres://dbuser:sprzedamopla@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await;
    let words: Vec<&str> = msg.content.split(" ").skip(1).collect();
    match pool {
        Ok(pool) => {
            if words.len() < 2 {
                msg.reply(&ctx, "missing argument").await?;
                return Ok(());
            }
            let id: String = msg.author.id.to_string();
            let moc: i64 = words[0].parse::<i64>().unwrap();
            let color: &str = words[1];
            let mut dcuser: DcUser = DcUser::new(id);
            dcuser.get_user_data_or_create_user(&pool).await;

            if moc < 1{
                msg.reply(
                    &ctx,
                        "Bruh.",
                )
                .await?;
                return Ok(());
            }

            if moc > dcuser.power{
                msg.reply(
                    &ctx,
                    format!(
                        "You don't have that much power. Your current power is {}.",
                        dcuser.moc()
                    ),
                )
                .await?;
                return Ok(());
            }
            let bet: u8 = match color {
                "black" => 0,
                "red" => 1,
                _ => {
                    msg.reply(&ctx, "invalid color").await?;
                    return Ok(());
                }
            };
            // msg.reply(&ctx, bet.to_string()).await?;
            // println!("======{bet}=======");
            let num = rand::thread_rng().gen_range(1..=5);
            for _ in 0..num {
                msg.channel_id
                    .send_message(&ctx, |m| m.content("the ball is rolling..."))
                    .await?;
                std::thread::sleep(std::time::Duration::from_millis(300));
            }

            if (rand::thread_rng().gen_range(0..=1) as u8) == 0 {
                let add_msg: String;
                if bet == 0 {
                    dcuser.power += moc;
                    add_msg = format!("You now have {} millionów mocy", dcuser.power);
                } else {
                    dcuser.power -= moc;
                    add_msg = format!("You have lost {} millionów mocy", moc);
                }
                msg.reply(
                    &ctx,
                    "the BALL landed on BLACK. ".to_string() + add_msg.as_str(),
                )
                .await?;
            } else {
                let add_msg: String;
                if bet == 1 {
                    dcuser.power += moc;
                    add_msg = format!("You now have {} millionów mocy", dcuser.power);
                } else {
                    dcuser.power -= moc;
                    add_msg = format!("You have lost {} millionów mocy", moc);
                }
                msg.reply(
                    &ctx,
                    "the BALL landed on RED. ".to_string() + add_msg.as_str(),
                )
                .await?;
            }

            dcuser.update_user(&pool).await;
        }
        Err(e) => {
            msg.reply(&ctx, format!("error: {e}")).await?;
            return Ok(());
        }
    }

    Ok(())
}

async fn is_dm(msg: &Message, ctx: &Context) -> bool {
    return !msg
        .channel(&ctx)
        .await
        .unwrap()
        .to_string()
        .starts_with("<#");
}
