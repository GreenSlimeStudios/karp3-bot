use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("numberinput")
        .description("Test command for number input")
        .create_option(|option| {
            option
                .name("int")
                .description("An integer from 5 to 10")
                .kind(CommandOptionType::Integer)
                .min_int_value(5)
                .max_int_value(10)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("number")
                .description("A float from -3.3 to 234.5")
                .kind(CommandOptionType::Number)
                .min_number_value(-3.3)
                .max_number_value(234.5)
                .required(true)
        })
}
pub fn run(_options: &[CommandDataOption]) -> String {
    match &_options[0].value {
        Some(v) => v.to_string(),
        None => "nothin".to_string(),
    }
    // "Dość <:the_rock:982328750240833536><:the_rock:982328750240833536><:the_rock:982328750240833536>".to_string()
}
