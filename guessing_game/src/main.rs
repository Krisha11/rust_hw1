fn random_int() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let mut seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    seed ^= seed << 21;
    seed ^= seed >> 35;
    seed ^= seed << 4;
    seed
}

fn read_int() -> u64 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input: u64 = input.trim().parse().unwrap();
    input
}

fn try_guess(answer : u64) -> bool {
    let input = read_int();
    if input > answer {
        println!("Answer < Your Number")
    }
    else if input < answer {
        println!("Answer > You Number")
    }
    else {
        println!("You win!")
    }
    answer == input
}


fn main() {
    println!("Hello, user!");
    println!("Guess the number from 1 to 100!");
    let answer = random_int() % 100 + 1;
    while !try_guess(answer) {};
    println!("Goodbye!");
}
