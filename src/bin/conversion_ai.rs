use std::{env::Args, fmt::format, fs, io::Read};

use semantic_weapons::{ai, weapon_params::WeaponParams};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut json_file = fs::File::open(args[1].clone()).unwrap();

    let model_path = ai::conversion::get::get_model();
    println!("AI model is ready at: {:?}", model_path);
    let tokenizer_path = ai::conversion::get::get_tokenizer();
    println!("Tokenizer is ready at: {:?}", tokenizer_path);
    let mut json_str: String = String::from("");
    json_file.read_to_string(&mut json_str);
    let json = serde_json::from_str(&json_str).unwrap();

    let model = ai::load::load_model(&model_path);
    let tokenizer = ai::load::load_tokenizer(&tokenizer_path);
    let prompt = ai::conversion::prompt::get_full_prompt(json);
    println!("Full Prompt: {}", prompt);
    let tokens = ai::run::generate_tokens(prompt, &tokenizer);
    println!("Generated tokens");
    let file_count = std::fs::read_dir("./output_tests").unwrap().filter(|x|String::from(x.as_ref().unwrap().file_name().to_str().unwrap()).contains("conversion_ai_output")).count();
ai::run::generate_json(tokens, model, &tokenizer, String::from(format!("./output_tests/conversion_ai_output_{}.txt", file_count+1)));
}