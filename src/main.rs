use rand::Rng;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: dice <dice type> <number of rolls> [<dice type> <number of rolls> ...]");
        return;
    }

    let mut total_sum: u32 = 0;

    // Use a loop variable `i` with the correct type `usize`
    for mut i: uszie in 1..args.len() {
        let dice_type: &String = &args[i];
        let num_rolls: u32 = args[i + 1].parse().expect("Invalid number of rolls");
        i += 1; // Increment `i` to move to the next element

        let roll_sum: u32 = roll_dice(dice_type, num_rolls);
        total_sum += roll_sum;
        println!("{} rolls of {}: {}", num_rolls, dice_type, roll_sum);
    }

    println!("Total sum: {}", total_sum);
}

fn roll_dice(dice_type: &str, num_rolls: u32) -> u32 {
    let sides: u32 = match dice_type {
        "d4" => 4,
        "d5" => 5,
        "d6" => 6,
        "d8" => 8,
        "d10" => 10,
        "d12" => 12,
        "d20" => 20,
        _ => panic!("Invalid dice type: {}", dice_type),
    };

    let mut sum: u32 = 0;
    for _ in 0..num_rolls {
        sum += rand::thread_rng().gen_range(1..=sides);
    }
    sum
}