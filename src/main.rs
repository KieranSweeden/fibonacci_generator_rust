use std::io;

fn main() {
    intro();

    loop {
        let index = prompt_user_for_index();
        println!("{} was the fibonacci index provided", &index);

        let number_at_index = generate_fibonacci_number_at_index(index);
        println!(
            "The fibonacci number at index {} is {}",
            &index, number_at_index
        );
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
