// 1. Importaciones
use std::env;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use reqwest::Error;
use serde_json::json;
use console::Term;
use crossterm::event::{self, KeyEvent, KeyCode};
use clap::App;

// 2. Estructuras y derivaciones
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub context: String,
    pub max_tokens: u32,
    pub model: String,
    pub restricted_responses: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            context: String::new(),
            max_tokens: 50,
            model: "gpt-3.5-turbo".to_string(),
            restricted_responses: false,
        }
    }
}

// 3. Implementación de funciones (No hay en este caso)

// 4. Funciones independientes
fn prompt_for_config() -> Config {
    let mut config = Config {
        context: String::new(),
        max_tokens: 50,
        restricted_responses: false,
        model: "gpt-3.5-turbo".to_string(),
    };

    println!("\nPor favor, ingrese la configuración deseada:");

    println!("Here is an example of how you can give context to the queries:");
    println!("context: ");
    let context_input = read_input();
    config.context = context_input.trim().to_string();

    println!("Maximum number of tokens (default: 50): ");
    let max_tokens_input = read_input();
    config.max_tokens = max_tokens_input.trim().parse::<u32>().unwrap_or(50);

/*
    println!("Verbosity [true/false, minimal, normal, extended] (default: normal): ");
    let verbosity_input = read_input();
    config.restricted_responses = match verbosity_input.trim().to_lowercase().as_str() {
        "true" | "false" => true,
        "minimal" => true,
        "normal" => false,
        "extended" => false,
        _ => false,
    };
*/
    println!("Model to use:");

    let model = select_from_list("Choose a model:", &[
        "GPT-3.5",        
        "GPT-3",
        "GPT-4",
        "Codex",
    ]);
    
    config.model = model.to_string();
    
    println!("Model chosen: {}", config.model);
    
    println!("Select a model option:");
    
    match config.model.as_str() {
        "GPT-3" => {
            let model_option = select_from_list("Choose an option:", &[
                "text-davinci-002",
                "text-curie-002",
                "text-babbage-002",
                "text-ada-002",
            ]);
            config.model = model_option.to_string();
        }
        "GPT-4" => {
            let model_option = select_from_list("Choose an option:", &[
                "text-davinci-004",
                "text-curie-004",
                "text-babbage-004",
                "text-ada-004",
            ]);
            config.model = model_option.to_string();
        }
        "GPT-3.5" => {
            let model_option = select_from_list("Choose an option:", &[
                "text-davinci-003",
                "text-curie-003",
                "text-babbage-003",
                "text-ada-003",
            ]);
            config.model = model_option.to_string();
        }
        "Codex" => {
            let model_option = select_from_list("Choose an option:", &[
                "code-davinci-002",
                "code-curie-002",
                "code-babbage-002",
                "code-ada-002",
            ]);
            config.model = model_option.to_string();
        }
        _ => {
            println!("Invalid model selected. Using default model.");
            config.model = "text-davinci-003".to_string();
        }
    }
    
    println!("Selected model option: {}", config.model);

    println!("Configuración elegida: {:#?}", config);
    config
}


fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error al leer la entrada del usuario");
    input
}


async fn chat_gpt(query: &str, api_key: &str, config: &Config) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let response = client
        .post(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "messages": [
                {"role": "system", "content": config.context},
                {"role": "user", "content": query}
            ],
            "max_tokens": config.max_tokens,
            "model": config.model, // Agrega esta línea
        }))
        .send()
        .await?;
    let json_resp = response.json::<serde_json::Value>().await?;
    println!("Respuesta JSON completa: {:#?}", json_resp);
    let answer = json_resp["choices"][0]["message"]["content"].as_str().unwrap_or("");
    Ok(answer.trim().to_string())
}



async fn configure_gpt(config: &Config) {
    let api_key = env::var("OPENAI_API_KEY").expect("Debe configurar la variable de entorno OPENAI_API_KEY");

    loop {
        print!("Ingrese su consulta ('exit' para salir): ");
        io::stdout().flush().unwrap();
        let mut query = String::new();
        io::stdin().read_line(&mut query).expect("Error al leer la entrada del usuario");
        query = query.trim().to_string();
        if query.eq_ignore_ascii_case("exit") {
            break;
        }

        match chat_gpt(&query, &api_key, &config).await {
            Ok(answer) => {
                println!("Respuesta: {}", answer);
            }
            Err(error) => eprintln!("Error al comunicarse con la API de ChatGPT: {:?}", error),
        }
    }
}


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


// 5. Función main
#[tokio::main]
async fn main() {
    let app = App::new("CLI Project IA")
        .version("0.1.1")
        .author("Alfonzo")
        .about("Interact with OpenAI's GPT")
        .subcommand(
            App::new("configure")
                .about("Configure GPT settings")
                .alias("c"),
        );

    let matches = app.clone().get_matches();

    let mut config = Config {
        context: String::new(),
        max_tokens: 50,
        restricted_responses: false,
        model: "gpt-3.5-turbo".to_string(),
    };

    match matches.subcommand() {
        Some(("configure", _matches)) => {
            config = prompt_for_config();
        }
        _ => {}
    }

    configure_gpt(&config).await;
}
