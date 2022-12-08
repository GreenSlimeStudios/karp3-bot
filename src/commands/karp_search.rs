use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use serpapi_search_rust::serp_api_search::SerpApiSearch;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("karp-search")
        .description("discover the internet with karp-search")
        .create_option(|option| {
            option
                .name("query")
                .description("what are you searchin for?")
                .kind(CommandOptionType::String)
                .set_autocomplete(false)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("number of results")
                .description("Number of search results")
                .kind(CommandOptionType::Integer)
                .min_int_value(1)
                .max_int_value(10)
                .required(true)
        })
}

// #[async_trait]
pub async fn run(_options: &[CommandDataOption]) -> String {
    match (&_options[0].value, &_options[1].value) {
        (Some(q), Some(n)) => {
            let api_key: String =
                std::fs::read_to_string("search_token.txt").expect("The file could not be read");
            let mut params = std::collections::HashMap::<String, String>::new();
            params.insert("q".to_string(), q.to_string().clone());
            params.insert("location".to_string(), "United States,Poland".to_string());

            let search = SerpApiSearch::google(params, api_key);

            let results = search.json().await.unwrap();
            let organic_results = results["organic_results"].as_array().unwrap();
            let mut msg: String = String::new();
            for i in 0..n.to_string().parse::<usize>().unwrap() {
                msg += organic_results[i]["link"].clone().to_string().as_str();
            }
            println!("{}", msg);
            // let links: Vec<&str> = link.split("\"").collect();
            // links[0].to_string()
            msg
        }
        _ => "nothin".to_string(),
    }
    // "Dość <:the_rock:982328750240833536><:the_rock:982328750240833536><:the_rock:982328750240833536>".to_string()
}
