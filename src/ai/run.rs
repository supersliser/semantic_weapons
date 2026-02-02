use std::{io::Write};

use candle_core::Tensor;
use candle_transformers::{generation::LogitsProcessor, models::quantized_llama::ModelWeights};

pub fn generate_tokens(prompt: String, tokenizer: &tokenizers::Tokenizer) -> Vec<u32> {
    let tokens = tokenizer
        .encode(prompt, true)
        .expect("Unable to encode tokens");
    tokens.get_ids().to_vec()
}

pub fn generate_json(mut tokens: Vec<u32>, mut model: ModelWeights, tokenizer: &tokenizers::Tokenizer, filename: String) -> () {
    let mut logits_processor = LogitsProcessor::new(22102004, Some(0.7), None);
    let mut file = std::fs::File::create(filename).unwrap();
    for _ in 0..2000 {
        let input = Tensor::new(tokens.as_slice(), &candle_core::Device::cuda_if_available(0).unwrap_or(candle_core::Device::Cpu))
            .expect("Unable to create tensor")
            .unsqueeze(0)
            .expect("Unable to unsqueeze tensor");

        let logits = model.forward(&input, 0).expect("Failed to generate logits").squeeze(0).expect("Failed to squeeze logits");
        let next_token = logits_processor.sample(&logits).expect("Failed to sample next token");
        tokens.push(next_token);

        if let Some(text) = tokenizer.id_to_token(next_token) {
            let clean_text = text
            .replace(' ', " ")
            .replace("<0x0A>", "\n")
            .replace('Ġ', " ")
            .replace('Ċ', "\n")
            .replace('▁', " ");
            print!("{}", clean_text);
            file.write(clean_text.as_bytes());
            std::io::Write::flush(&mut std::io::stdout()).expect("Unable to flush stdout");
        }
    }
}