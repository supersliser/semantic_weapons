use std::path::PathBuf;

use hf_hub::{
    Repo,
    api::sync::{Api, ApiBuilder},
};

pub fn get_model() -> PathBuf {
    let api = get_auth();

    let repo = api.repo(Repo::new(
        "MaziyarPanahi/Mistral-7B-Instruct-v0.3-GGUF".to_string(),
        hf_hub::RepoType::Model,
    ));

    let path = repo
        .get("Mistral-7B-Instruct-v0.3.Q4_K_M.gguf")
        .expect("Failed to download AI model");

    println!("Model located at: {:?}", path);
    path
}

pub fn get_tokenizer() -> PathBuf {
    let api = get_auth();

    let repo = api.repo(Repo::new(
        "mistralai/Mistral-7B-Instruct-v0.3".to_string(),
        hf_hub::RepoType::Model,
    ));

    let path = repo
        .get("tokenizer.json")
        .expect("Failed to download tokenizer");

    println!("Tokenizer located at: {:?}", path);
    path
}

fn get_auth() -> Api {
    dotenvy::dotenv().ok();
    ApiBuilder::new()
        .with_token(Some(
            std::env::var("HF_KEY").expect("HF_KEY not set in .env"),
        ))
        .build()
        .unwrap()
}
