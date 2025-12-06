pub mod ai;
pub mod weapon_params;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let model_path = ai::get::get_model();
    println!("AI model is ready at: {:?}", model_path);
    let tokenizer_path = ai::get::get_tokenizer();
    println!("Tokenizer is ready at: {:?}", tokenizer_path);
    let character_description = args[1..].join(" ");

    let model = ai::load::load_model(&model_path);
    let tokenizer = ai::load::load_tokenizer(&tokenizer_path);
    let prompt = ai::prompt::get_full_prompt(character_description);
    let tokens = ai::run::generate_tokens(prompt, &tokenizer);
    ai::run::generate_json(tokens, model, &tokenizer);
}