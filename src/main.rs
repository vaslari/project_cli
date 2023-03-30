// 1. Importaciones
use std::env;
use std::io::{self, Write};
use serde::{Serialize, Deserialize};
use reqwest::Error;
use serde_json::json;

// 2. Estructuras y derivaciones
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub context: String,
    pub max_tokens: u32,
    pub model: String,
    pub restricted_responses: bool,
}


// 3. Implementación de funciones (No hay en este caso)

// 4. Funciones independientes
fn prompt_for_config() -> Config {
    let mut config = Config {
        context: String::new(),
        max_tokens: 50,
        restricted_responses: false,
        model: "default".to_string(),
    };

    println!("Por favor, ingrese la configuración deseada:");

    print!("Contexto: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut config.context).expect("Error al leer la entrada del usuario");
    config.context = config.context.trim().to_string();

    print!("Cantidad máxima de tokens (por defecto: 50): ");
    io::stdout().flush().unwrap();
    let mut max_tokens_input = String::new();
    io::stdin().read_line(&mut max_tokens_input).expect("Error al leer la entrada del usuario");
    if let Ok(value) = max_tokens_input.trim().parse::<u32>() {
        config.max_tokens = value;
    }

    print!("¿Restringir respuestas? (sí/no, por defecto: no): ");
    io::stdout().flush().unwrap();
    let mut restrict_input = String::new();
    io::stdin().read_line(&mut restrict_input).expect("Error al leer la entrada del usuario");
    config.restricted_responses = restrict_input.trim().eq_ignore_ascii_case("sí");

    print!("Modelo a utilizar [code, default]: ");
    io::stdout().flush().unwrap();
    let mut model_input = String::new();
    io::stdin().read_line(&mut model_input).expect("Error al leer la entrada del usuario");
    let model_input = model_input.trim();
    if model_input.eq_ignore_ascii_case("code") {
        config.model = "text-davinci-codex".to_string();
    } else {
        config.model = "text-davinci-002".to_string();
    }

    println!("Configuración elegida: {:#?}", config);
    config
}


async fn chat_gpt(query: &str, api_key: &str, config: &Config) -> Result<String, Error> {
    let url = format!("https://api.openai.com/v1/engines/{}/completions", config.model);
    let prompt = format!("{}{}{}", config.context, " Mi pregunta es: ", query);
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "prompt": prompt,
            "max_tokens": config.max_tokens,
        }))
        .send()
        .await?;
    let json_resp = response.json::<serde_json::Value>().await?;
    println!("Respuesta JSON completa: {:#?}", json_resp);
    let answer = json_resp["choices"][0]["text"].as_str().unwrap_or("");
    Ok(answer.trim().to_string())
}


// 5. Función main
#[tokio::main]
async fn main() {
    let api_key = env::var("OPENAI_API_KEY").expect("Debe configurar la variable de entorno OPENAI_API_KEY");

    let config = prompt_for_config();

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
            Ok(answer) => println!("Respuesta: {}", answer),
            Err(error) => eprintln!("Error al comunicarse con la API de ChatGPT: {:?}", error),
        }
    }
}