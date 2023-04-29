use std::io;

fn main() {
    intro();

    loop {
        let index = prompt_user_for_index();
        println!("Generating fibonacci number at index position {}", &index);

        let number_at_index = generate_fibonacci_number_at_index(index);
        println!(
            "The fibonacci number at index {} is {}",
            &index, number_at_index
        );

        let user_wants_to_retry = prompt_user_to_retry();
        match user_wants_to_retry {
            true => continue,
            false => {
                println!("Goodbye.");
                break;
            }
        }
    }
}

fn prompt_user_to_retry() -> bool {
    loop {
        println!("Would you like to retry? (Yes = Y | No = N)");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input was invalid.");

        let input = input.trim();

        match input {
            "Y" => break true,
            "N" => break false,
            _ => {
                println!("Input was not Y or N.");
                continue;
            }
        }
    }
}

fn generate_fibonacci_number_at_index(index: i32) -> i32 {
    let range = 0..=index - 1;
    range.fold((0, 1), |acc, _| (acc.1, acc.0 + acc.1)).0
}

fn prompt_user_for_index() -> i32 {
    loop {
        println!("Please provide the index of the fibonacci number to generate:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Input was invalid.");

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
