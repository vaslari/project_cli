// 0. Crate attributes

// 1. Imports
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use serde_json::json;
use console::Term;
use crossterm::event::{self, KeyEvent, KeyCode};
use clap::App;
use reqwest::Client;


// 2. Structures and Derivations
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub context: String,
    pub max_tokens: u32,
    pub model: String,
    pub verbosity: Verbosity,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbosity: Verbosity::Default,
            context: String::new(),
            max_tokens: 50,
            model: "gpt-3.5-turbo".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Verbosity {
    Default,
    Full,
    Extended,
}


// 3. Main Function
#[tokio::main]
async fn main() {
    let app = App::new("CLI Project IA")
        .version("0.1.1")
        .author("Alfonzo")
        .about("Interact with OpenAI's GPT")
        .subcommand(
            App::new("configure")
                .about("Configure GPT settings")
                .alias("c")
                .alias("-c"),
        )
        .subcommand(
            App::new("show-config")
                .about("Show current configuration")
                .alias("s")
                .alias("-s"),
        );

    let matches = app.clone().get_matches();

    let mut config = match load_config() {
        Some(config) => config,
        None => {
            let new_config = prompt_for_config();
            save_config(&new_config);
            new_config
        },
    };

    match matches.subcommand() {
        Some(("configure", _matches)) => {
            config = prompt_for_config();
            save_config(&config);
        }
        Some(("show-config", _matches)) => {
            print_config(&config);
        }
        _ => {}
    }

    configure_gpt(&config).await;
}


// 4. Subcommand Functions
fn prompt_for_config() -> Config {
    let mut config = Config {
        verbosity: select_verbosity("Output verbosity [default, full, extended]: "),
        context: String::new(),
        max_tokens: 50,
        model: "gpt-3.5-turbo".to_string(),
    };

    println!("\nThe next is an example of how you can give context to the queries.");
    println!("\nContext: Imagine you are a travel blogger and you want to write an article about your recent trip to Japan.\nYou want to generate some ideas for the article using this tool.
    \nPrompt: Generate three ideas for my Japan travel article.
    \nClarification: By providing the context that the writer is a travel blogger and the topic is about their recent trip to Japan, the prompt becomes more specific and focused.\nThis will help the tool to generate more relevant and useful ideas for the article.");
    
    println!("\nPlease, introduce desire configuration:");
    println!("\nContext: ");

    let context_input = read_input();
    config.context = context_input.trim().to_string();

    println!("Maximum number of tokens (default: 50): ");
    let max_tokens_input = read_input();
    config.max_tokens = max_tokens_input.trim().parse::<u32>().unwrap_or(50);

    // Cambios realizados aquí:
    println!("Model and model option:");
    config.model = select_model_and_option();
    
    println!("Selected model option: {}", config.model);

    println!("Chosen configuration: {:#?}", config);
    config
}


fn select_model_and_option() -> String {
    let model = select_from_list("Choose a model:", &[
        "GPT-3.5",
        "GPT-3",
        "GPT-4",
        "Codex",
    ]);

    let model_options = match model {
        "GPT-3" => &[
            "text-davinci-002",
            "text-curie-002",
            "text-babbage-002",
            "text-ada-002",
        ],
        "GPT-4" => &[
            "text-davinci-004",
            "text-curie-004",
            "text-babbage-004",
            "text-ada-004",
        ],
        "GPT-3.5" => &[
            "text-davinci-003",
            "text-curie-003",
            "text-babbage-003",
            "text-ada-003",
        ],
        "Codex" => &[
            "code-davinci-002",
            "code-curie-002",
            "code-babbage-002",
            "code-ada-002",
        ],
        _ => {
            println!("Invalid model selected. Using default model.");
            &["text-davinci-003", "", "", ""]
        }
    };

    let model_option = select_from_list("Choose an option:", model_options);
    model_option.to_string()
}

fn build_api_url(model: &str) -> String {
    format!("https://api.openai.com/v1/engines/{}/completions", model)
}


fn save_config(config: &Config) {
    let config_json = serde_json::to_string_pretty(config).unwrap();
    let mut file = File::create("config.json").expect("Unable to create config file");
    file.write_all(config_json.as_bytes())
        .expect("Unable to write config to file");
    println!("Configuration saved to config.json");
}


fn load_config() -> Option<Config> {
    let config_path = Path::new("config.json");
    if config_path.exists() {
        let file = File::open(config_path).ok()?;
        let config: Config = serde_json::from_reader(file).ok()?;
        Some(config)
    } else {
        None
    }
}

/*async fn update_config(api_key: &str) -> Result<(), Error> {
    let mut config = load_config().unwrap_or_default();

    let engine = select_model(&config.model);
    let verbosity = config.verbosity;
    let max_tokens = config.max_tokens;
    let context = config.context;
    
    config.model = engine;
    config.verbosity = verbosity;
    config.max_tokens = max_tokens;
    config.context = context;

    save_config(&config);

    println!("Config updated successfully.");
    Ok(())
}*/



// 5. Independent Functions
fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading user input");
    input
}

fn print_config(config: &Config) {
    println!("Current configuration: {:#?}", config);
}
/*fn chat_gpt(config: &Config, prompt: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let model = &config.model;
    let api_url = build_api_url(model);
    
    let url: String;

    match config.model.as_str() {
        "text-davinci-002" | "text-curie-002" | "text-babbage-002" | "text-ada-002" |
        "text-davinci-003" | "text-curie-003" | "text-babbage-003" | "text-ada-003" |
        "text-davinci-004" | "text-curie-004" | "text-babbage-004" | "text-ada-004" => {
            url = format!("https://api.openai.com/v1/engines/{}/completions", config.model);
        }
        "code-davinci-002" | "code-curie-002" | "code-babbage-002" | "code-ada-002" => {
            url = format!("https://api.openai.com/v1/engines/{}/completions", config.model);
        }
        _ => {
            eprintln!("Invalid model selected. Using default model.");
            url = "https://api.openai.com/v1/engines/gpt-3.5-turbo/completions".to_string();
        }
    }
    
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": format!("{}{}{}", config.context, "\n", prompt),
            "max_tokens": config.max_tokens,
            "temperature": 0.7,
            "n": 1,
        }))
        .send()
        .await?;
    
    let json_resp = response.json::<serde_json::Value>().await?;

    match config.verbosity {
        Verbosity::Full => {
            println!("Full JSON response: {:#?}", json_resp);
        }
        Verbosity::Extended => {
            print_config(config);
            println!("Full JSON response: {:#?}", json_resp);
        }
        _ => {}
    }

    println!("Full JSON response: {:#?}", json_resp);
    let answer = json_resp["choices"][0]["text"].as_str().unwrap_or("");
    
    Ok(answer.trim().to_string())
}*/

async fn chat_gpt(config: &Config, prompt: &str, api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let model = &config.model;
    let api_url = build_api_url(model);

    let response = client
        .post(&api_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": format!("{}{}{}", config.context, "\n", prompt),
            "max_tokens": config.max_tokens,
            "temperature": 0.7,
            "n": 1,
        }))
        .send()
        .await?;

    let json_resp = response.json::<serde_json::Value>().await?;

    match config.verbosity {
        Verbosity::Full => {
            println!("Full JSON response: {:#?}", json_resp);
            print_config(config);
        }
        Verbosity::Extended => {
            print_config(config);
        }
        _ => {}
    }

    let answer = json_resp["choices"][0]["text"].as_str().unwrap_or("");
    Ok(answer.trim().to_string())
}



async fn configure_gpt(config: &Config) {
    let api_key = env::var("OPENAI_API_KEY").expect("You must set the OPENAI_API_KEY environment variable");

    loop {
        print!("Enter your query ('exit' to quit): ");
        io::stdout().flush().unwrap();
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Error reading user input");
        query = query.trim().to_string();
        if query.eq_ignore_ascii_case("exit") {
            break;
        }

        match chat_gpt(&config, &query, &api_key).await {
            Ok(answer) => {
                println!("Answer: {}", answer);
            }
            Err(error) => eprintln!("Failed to communicate with the ChatGPT API: {:?}", error),
        }
    }
}


// 6. Helper Functions
fn select_from_list(prompt: &str, options: &[&'static str]) -> &'static str {
    let term = Term::stdout();
    let mut selected = 0;

    loop {
        term.clear_last_lines(options.len() + 1).unwrap();
        println!("{}", prompt);

        for (idx, option) in options.iter().enumerate() {
            if idx == selected {
                println!("> {}", option);
            } else {
                println!("  {}", option);
            }
        }

        match event::read().unwrap() {
            event::Event::Key(KeyEvent {
                code: KeyCode::Up,
                ..
            }) => {
                if selected > 0 {
                    selected -= 1;
                }
            }
            event::Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => {
                if selected < options.len() - 1 {
                    selected += 1;
                }
            }
            event::Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                break;
            }
            _ => {}
        }
    }

    options[selected]
}

fn select_verbosity(prompt: &str) -> Verbosity {
    let options = &["default", "full", "extended"];
    let selected = select_from_list(prompt, options);
    match selected {
        "default" => Verbosity::Default,
        "full" => Verbosity::Full,
        "extended" => Verbosity::Extended,
        _ => Verbosity::Default,
    }
}

fn select_model(model: &str) -> String {
    match model {
        "text-davinci-002" | "text-curie-002" | "text-babbage-002" | "text-ada-002" |
        "text-davinci-003" | "text-curie-003" | "text-babbage-003" | "text-ada-003" |
        "text-davinci-004" | "text-curie-004" | "text-babbage-004" | "text-ada-004" |
        "code-davinci-002" | "code-curie-002" | "code-babbage-002" | "code-ada-002" => model.to_string(),
        _ => {
            eprintln!("Invalid model selected. Using default model.");
            "text-davinci-003".to_string()
        }
    }
}