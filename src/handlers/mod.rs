pub mod message_handler;
pub mod interaction_create_handler;
pub mod ready_handler;

use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        message_handler::handle_message(ctx, msg).await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create_handler::handle_interaction(ctx, interaction).await;
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        ready_handler::ready(ctx, ready).await;
    }
}