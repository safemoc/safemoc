use async_openai::{
    Client,
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs,
    },
};
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;
    let _client = build_client();

    let system_message = ChatCompletionRequestSystemMessageArgs::default()
        .content("你的身份是我的女儿，请用中文与我沟通")
        .build()?;
    let mut message: Vec<ChatCompletionRequestMessage> = vec![system_message.into()];
    loop {
        let mut input = String::new();

        if io::stdin().read_line(&mut input)? == 0 {
            break;
        }
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        let user_message = ChatCompletionRequestUserMessageArgs::default()
            .content(input)
            .build()?;

        message.push(user_message.into());

        let request = CreateChatCompletionRequestArgs::default()
            .model("deepseek-flash")
            .messages(message.clone())
            .build()?;

        let response = _client.chat().create(request).await?;

        if let Some(choice) = response.choices.first() {
            if let Some(content) = &choice.message.content {
                println!("{}", content);

                let assistant_message = ChatCompletionRequestAssistantMessageArgs::default()
                    .content(content.clone())
                    .build()?;

                message.push(assistant_message.into());
            }
        }
    }

    Ok(())
}

pub fn build_client() -> Client<OpenAIConfig> {
    let api_key = std::env::var("API_KEY").expect("请先设置 DEEPSEEK_API_KEY 环境变量");
    let config = OpenAIConfig::new()
        .with_api_key(api_key)
        .with_api_base("https://api.deepseek.com");

    Client::with_config(config)
}
