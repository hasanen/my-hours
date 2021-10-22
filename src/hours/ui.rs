use read_input::prelude::*;
pub fn ask_input<T: std::str::FromStr>(question: &str) -> T {
    println!("{}", question);
    let user_input = input::<String>().get();
    match user_input.parse::<T>() {
        Ok(value) => return value,
        Err(_err) => {
            println!("Error occured while parsing value {}", user_input);
            std::process::exit(1);
        }
    }
}
