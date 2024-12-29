use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ollama = Ollama::default();

    let model = "tinyllama:latest".to_string();
    let prompt = "Why is the sky blue?".to_string();

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
