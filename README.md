# Rust OpenAI Chat Interface

This repository contains a Rust application that facilitates real-time chatting with OpenAI's GPT model. The application reads the API key from the environment, allows the user to send messages, and receives responses directly from OpenAI.

## About the Project

This Rust project utilizes the `openai_api_rust` crate to create an interactive chat application that communicates with OpenAI's API. Users can engage in a conversational exchange with OpenAI's AI models directly from the command line.

## Getting Started

To get this application running on your local machine, follow these steps:

```bash
# clone the Repository
git clone https://github.com/yourusername/rust-openai-chat.git

# navigate to the project directory
cd rust-openai-chat

# set environment variable
# you can get a key from the openai developer dashboard 
export OPENAI_API_KEY='your_openai_api_key'


# build with cargo
cargo build --release


# you can use cargo to run the application or just use the executable 
cargo run --release

# type 'quit' to exit the chat.
```

## License

This project is licensed under the Zero-Clause BSD License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to the OpenAI team for providing the API and documentation that made this project possible.
- Thanks to the Rust community for excellent resources and support.

```
