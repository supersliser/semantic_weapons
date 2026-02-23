use semantic_weapons::ai;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    
    // Check for -prompt flag
    if args.contains(&String::from("-prompt")) {
        // Generate prompt from character description and print without running the model
        let character_description = args[1..].iter()
            .filter(|arg| arg.as_str() != "-prompt")
            .cloned()
            .collect::<Vec<_>>()
            .join(" ");
        
        let prompt = ai::description::prompt::get_full_prompt(character_description);
        println!("{}", prompt);
        
        // Copy to clipboard
        if let Ok(mut clipboard) = arboard::Clipboard::new() {
            let _ = clipboard.set_text(prompt);
            println!("\n[Prompt copied to clipboard]");
        } else {
            eprintln!("Warning: Could not copy to clipboard");
        }
        return;
    }
    
    // Normal AI execution
    let character_description = args[1..].join(" ");
    println!("Character description: {}", character_description);

    let model_path = ai::description::get::get_model();
    println!("AI model is ready at: {:?}", model_path);
    let tokenizer_path = ai::description::get::get_tokenizer();
    println!("Tokenizer is ready at: {:?}", tokenizer_path);

    let model = ai::load::load_model(&model_path);
    let tokenizer = ai::load::load_tokenizer(&tokenizer_path);
    let prompt = ai::description::prompt::get_full_prompt(character_description);
    let tokens = ai::run::generate_tokens(prompt, &tokenizer);
    println!("Generated tokens");
    let file_count = std::fs::read_dir("./output_tests").unwrap().filter(|x|String::from(x.as_ref().unwrap().file_name().to_str().unwrap()).contains("description_ai_output")).count();
    ai::run::generate_json(tokens, model, &tokenizer, String::from(format!("./output_tests/description_ai_output_{}.txt", file_count+1)));
}