use semantic_weapons::ai;
fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let model_path = ai::get::get_model();
    println!("AI model is ready at: {:?}", model_path);
    let tokenizer_path = ai::get::get_tokenizer();
    println!("Tokenizer is ready at: {:?}", tokenizer_path);
    let character_description = args[1..].join(" ");
    println!("Character description: {}", character_description);

    let model = ai::load::load_model(&model_path);
    let tokenizer = ai::load::load_tokenizer(&tokenizer_path);
    let prompt = ai::description_prompt::get_full_prompt(character_description);
    let tokens = ai::run::generate_tokens(prompt, &tokenizer);
    println!("Generated tokens");
    ai::run::generate_json(tokens, model, &tokenizer, String::from("./description_ai_output.txt"));
}