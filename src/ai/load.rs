use std::{fs, path::PathBuf};

use candle_core::quantized::gguf_file;

use candle_transformers::models::quantized_llama::ModelWeights;
use tokenizers::Tokenizer;

pub fn load_model(model_path: &PathBuf) -> ModelWeights {
    println!("Loading model from: {:?}", model_path);
    let mut model_file = fs::File::open(model_path).expect("Failed to open model file");
    let model_content = gguf_file::Content::read(&mut model_file).expect("Failed to get model content");
    ModelWeights::from_gguf(model_content, &mut model_file, &candle_core::Device::Cpu).expect("Failed to load Model")
}

pub fn load_tokenizer(tokenizer_path: &PathBuf) -> Tokenizer {
    println!("Loading tokenizer from: {:?}", tokenizer_path);
    let tokenizer_file = fs::read_to_string(tokenizer_path).expect("Failed to load tokenizer from file");
    let tokenizer : Tokenizer = serde_json::from_str(&tokenizer_file).expect("Failed to parse tokenizer");
    tokenizer
}