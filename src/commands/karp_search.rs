use search_with_google::search;
// use search_with_google::blocking::search;
use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

// use tokio::runtime::Runtime;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("karp-search")
        .description("discover the internet with karp-search")
        .create_option(|option| {
            option
                .name("query")
                .description("An integer from 5 to 10")
                .kind(CommandOptionType::String)
                .set_autocomplete(false)
                .required(true)
        })
}

// #[async_trait]
pub async fn run(_options: &[CommandDataOption]) -> String {
    match &_options[0].value {
        Some(v) => {
            // println!("searching: {}", v);
            let results = search(v.to_string().as_str(), 3, None).await;
            if let Ok(result_list) = results {
                // println!(
                //     "Title : {}\nLink : {}",
                //     result_list[0].title, result_list[0].link
                // );
                println!("{}", result_list.len());
            }
            v.to_string()
        }
        None => "nothin".to_string(),
    }
    // "Dość <:the_rock:982328750240833536><:the_rock:982328750240833536><:the_rock:982328750240833536>".to_string()
}
