use std::io;

pub fn read_user_input() -> Result<i32, String> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => parse_input_to_int(input.replace("\n", "")),
        Err(_) => Err(String::from("Couldn't read input"))
    }
}

fn parse_input_to_int(input: String) -> Result<i32, String> {
    match input.parse::<i32>() {
        Ok(result) => Ok(result),
        Err(_) => Err(String::from("Couldn't parse number"))
    }
}
