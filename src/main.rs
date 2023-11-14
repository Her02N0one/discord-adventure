use dotenv::dotenv;
use serde::{ser::SerializeStruct, Deserialize, Serialize};
use serde_json;
use std::env;

use reqwest;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        prelude::{GuildChannel, PermissionOverwrite, PermissionOverwriteType, UserId},
        Permissions,
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // print out the channel details of the created channel to the console
    async fn channel_create(&self, _ctx: Context, _channel: &GuildChannel) {
        println!("Created channel {:?}", _channel);

        _channel.say(&_ctx.http, "Hello, World!").await.unwrap();
    }

    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        // Ignore messages from bots
        if msg.author.bot {
            return;        
        }

        // Ignore messages that don't start with the bot's prefix
        if !msg.content.starts_with("!") {
            return;
        }

        // Get the channel ID of the channel the message was sent in
        let channel_id = msg.channel_id;
        // Get the user ID of the user that sent the message
        let user_id = msg.author.id;

        // Get the channel object from the channel ID
        let channel = channel_id.to_channel(&ctx).await.unwrap();

        // Check if the channel is a DM channel
        

    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct HumanHomeChannel {
    channel_id: u64,
    user_id: Option<u64>,
}

impl HumanHomeChannel {
    fn new(channel_id: u64) -> HumanHomeChannel {
        HumanHomeChannel {
            channel_id,
            user_id: None,
        }
    }

    fn set_user_id(&mut self, user_id: u64) {
        self.user_id = Some(user_id);
    }

    fn get_user_id(&self) -> Option<u64> {
        self.user_id
    }

    fn get_channel_id(&self) -> u64 {
        self.channel_id
    }
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
}

async fn call_openai_api(request: ChatRequest) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_key = env::var("OPENAI_API_KEY").expect("Expected a key in the environment"); // Use an environment variable here

    let res = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await?
        .text()
        .await?;

    Ok(res)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_BOT_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::all();

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
