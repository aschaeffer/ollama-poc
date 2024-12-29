use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Optional model name
    #[arg(short, long)]
    model: Option<String>,

    /// The prompt
    #[arg(short, long)]
    prompt: Option<String>,

    /// Hostname of ollama
    #[arg(long)]
    host: Option<String>,

    /// Port of ollama
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let ollama = if let (Some(host), Some(port)) = (args.host, args.port) {
        Ollama::new(host, port)
    } else {
        Ollama::default()
    };

    let model = args.model.unwrap_or("tinyllama:latest".to_string());
    let prompt = args.prompt.unwrap_or("Why is the sky blue?".to_string());

    println!(">>> {}", prompt);

    match ollama.generate(GenerationRequest::new(model, prompt)).await {
        Ok(response) => {
            println!("{}", response.response);
            Ok(())
        }
        Err(e) => {
            println!("Error: {}", e);
            Err(e.into())
        }
    }
}
