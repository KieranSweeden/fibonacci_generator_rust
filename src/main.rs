use std::io;

fn main() {
    intro();

    loop {
        let index = prompt_user_for_index();

        println!("{} was the fibonacci index provided", index);
    }
}

fn prompt_user_for_index() -> i32 {
    loop {
        println!("Please provide the index of the fibonacci number to generate:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input was invalid");

        let input = input.trim().parse::<i32>();

        match input {
            Result::Ok(number) => break number,
            Result::Err(_) => {
                println!("Input provided was not a valid number.");
                continue;
            }
        }
    }
}

fn intro() {
    println!("================");
    println!("Welcome to the fibonacci generator!");
    println!("================");
}
