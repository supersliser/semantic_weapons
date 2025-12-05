use hf_hub::{Repo, api::sync::Api};
use std::path::PathBuf;

pub fn get_model() -> PathBuf {
    let api = Api::new().expect("Failed to create API client");

    let repo = api.repo(Repo::new(
        "huggingface/Llama-3.2-1B-Instruct-GGUF".to_string(),
        hf_hub::RepoType::Model,
    ));

    let path = repo
        .get("llama-3.2-1b-instruct-q4_k_m.gguf")
        .expect("Failed to download AI model");

    println!("Model located at: {:?}", path);
    path
}
