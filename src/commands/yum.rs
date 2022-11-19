use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Dość <:the_rock:982328750240833536><:the_rock:982328750240833536><:the_rock:982328750240833536>".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("yum").description("A tasty command")
}
