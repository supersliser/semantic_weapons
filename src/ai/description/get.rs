use hf_hub::{
    Repo,
    api::sync::{Api, ApiBuilder},
};
use std::{fs, path::PathBuf};

pub fn get_model() -> PathBuf {
    let api = get_auth();

    let repo = api.repo(Repo::new(
        "bartowski/Llama-3.2-1B-Instruct-GGUF".to_string(),
        hf_hub::RepoType::Model,
    ));

    let path = repo
        .get("Llama-3.2-1B-Instruct-Q4_K_M.gguf")
        .expect("Failed to download AI model");

    println!("Model located at: {:?}", path);
    path
}

pub fn get_tokenizer() -> PathBuf {
    let api = get_auth();

    let repo = api.repo(Repo::new(
        "nicoboss/Llama-3.2-1B-Instruct-Uncensored".to_string(),
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
    let cache_dir = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".hf_cache");
    fs::create_dir_all(&cache_dir).expect("Failed to create local cache directory");
    ApiBuilder::new()
        .with_token(Some(
            std::env::var("HF_KEY").expect("HF_KEY not set in .env"),
        ))
        .with_cache_dir(cache_dir)
        .build()
        .unwrap()
}
