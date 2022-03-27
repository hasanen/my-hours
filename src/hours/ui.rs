use read_input::prelude::*;
pub fn ask_input<T: std::str::FromStr>(question: &str) -> Option<T> {
    println!("{}", question);
    let user_input = input::<String>().get();
    if user_input.trim().is_empty() {
        return None;
    }
    match user_input.parse::<T>() {
        Ok(value) => Some(value),
        Err(_err) => {
            println!("Error occured while parsing value '{}'", user_input);
            std::process::exit(1);
        }
    }
}
