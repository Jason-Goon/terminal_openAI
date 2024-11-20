use openai_api_rust::*;
use openai_api_rust::chat::*;
use std::io::{self, Write}; 

fn main() {
    println!("Loading API key from environment...");
    let auth = Auth::from_env().expect("Failed to load OPENAI_API_KEY from environment");
    println!("API key loaded. You can start chatting with OpenAI. Type 'quit' to exit.");

    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    // Buffer conversation history
    let mut messages = Vec::new();

    loop {
        print!("You: ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure "You: " prompt is displayed before user types

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        let user_input = user_input.trim();

        if user_input.eq_ignore_ascii_case("quit") {
            println!("Exiting chat...");
            break;
        }

        // Add  message to conversation 
        messages.push(Message { role: Role::User, content: user_input.to_string() });

        // Prepare chat body and history
        let body = ChatBody {
            model: "gpt-4-0125-preview".to_string(),
            max_tokens: Some(150),
            temperature: Some(0.7_f32),
            top_p: Some(1.0),
            n: Some(1),
            stream: Some(false),
            stop: None,
            presence_penalty: None,
            frequency_penalty: None,
            logit_bias: None,
            user: None,
            messages: messages.clone(), 
        };

    
        match openai.chat_completion_create(&body) {
            Ok(response) => {
                if let Some(choice) = response.choices.first() {
                    if let Some(message) = &choice.message {
                        println!("AI: {}", message.content);
                        messages.push(Message { role: Role::Assistant, content: message.content.clone() });
                    } else {
                        println!("No message content found in the response.");
                    }
                } else {
                    println!("No choices found in the response.");
                }
            },
            Err(e) => println!("Failed to create chat completion: {}", e),
        }
    }
}
