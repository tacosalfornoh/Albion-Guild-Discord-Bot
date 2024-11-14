use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

// TODO: Implement the run function for the ping command
pub fn run(_options: &[ResolvedOption]) -> String {
    "Pong!".to_string()
}

// TODO: Implement the CreateCommand for the ping command
pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Replies with Pong!!")
}
