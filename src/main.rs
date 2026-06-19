mod openrouter;
mod tools;

use openrouter::{Message, call_openrouter};
use sqlx::sqlite::SqlitePoolOptions;
use std::env;
use std::io::{self, Write};
use tools::{define_tools, dispatch_tool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let api_key = env::var("OPENROUTER_API_KEY")?;
    let model = env::var("OPENROUTER_MODEL")?;
    let database_url = env::var("DATABASE_URL")?;

    let pool = SqlitePoolOptions::new().connect(&database_url).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS items (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT NOT NULL)"
    )
    .execute(&pool)
    .await?;

    let client = reqwest::Client::new();
    let tools = define_tools();

    print!("Tu: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let user_message = input.trim().to_string();

    let mut messages = vec![Message {
        role: "user".to_string(),
        content: Some(user_message),
        tool_calls: None,
        tool_call_id: None,
    }];

    loop {
        let response = call_openrouter(&client, &api_key, &model, &messages, &tools).await?;
        let assistant_message = response.choices[0].message.clone();

        if let Some(tool_calls) = assistant_message.tool_calls.clone() {
            messages.push(assistant_message);

            for tool_call in tool_calls {
                let result = dispatch_tool(
                    &tool_call.function.name,
                    &tool_call.function.arguments,
                    &pool,
                )
                .await;

                messages.push(Message {
                    role: "tool".to_string(),
                    content: Some(result),
                    tool_calls: None,
                    tool_call_id: Some(tool_call.id),
                });
            }
        } else {
            println!("{}", assistant_message.content.unwrap_or_default());
            break;
        }
    }

    Ok(())
}
