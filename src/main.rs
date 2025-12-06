pub mod ai;
pub mod weapon_params;

fn main() {
    let model_path = ai::get::get_model();
    println!("AI model is ready at: {:?}", model_path);
}
