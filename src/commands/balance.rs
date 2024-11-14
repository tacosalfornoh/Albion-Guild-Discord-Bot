use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

// TODO: Implement the run function for the balance command
pub fn run(_options: &[ResolvedOption]) -> String {
    "Balance".to_string()
}

// TODO: Implement the CreateCommand for the balance command

pub fn register() -> CreateCommand {
    CreateCommand::new("balance").description("Replies with Balance!!")
}