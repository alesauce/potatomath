use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::io;

fn main() {
    println!("Welcome to Potatomath!");
    let people = get_people_input_from_user();
    let total_bags_required: i32 = calculate_required_bags_of_potatoes(people);
    let total_cost: f32 = calculate_total_cost(total_bags_required);
    println!("Your total cost per month is: ${total_cost}. Don't forget the multivitamin!")
}

fn get_people_input_from_user() -> i32 {
    println!("Please input the number of people you are feeding.");
    let mut people_input = String::new();
    io::stdin()
        .read_line(&mut people_input)
        .expect("Failed to read line");
    println!("You input {} people.", people_input);
    let people: i32 = match people_input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error parsing string '{}': {}", people_input, e);
            0
        }
    };
    people
}

fn calculate_total_cost(total_bags_required: i32) -> f32 {
    // TODO: maybe just convert all numbers into Decimals since I'm using them pretty much
    // everywhere, and just convert back to i32 as the last step before returning output
    // Cost of a 5 pound bag of potatoes at Wal-mart. Source:
    // https://web.archive.org/web/20240529023336/https://www.walmart.com/ip/Fresh-Whole-Russet-Potatoes-5lb-bag/10447837
    let cost_potatoes: f32 = 2.47;
    cost_potatoes * (total_bags_required as f32)
}

fn calculate_required_bags_of_potatoes(people: i32) -> i32 {
    let calories_per_person = dec!(2000);
    let total_calories_required = calories_per_person * Decimal::new(people.into(), 0);
    // Approx. 15 servings per bag of potatoes, 110 calories per serving. Source:
    // https://web.archive.org/web/20240529023336/https://www.walmart.com/ip/Fresh-Whole-Russet-Potatoes-5lb-bag/10447837
    let calories_per_bag_of_potatoes = dec!(1650);
    let total_bags_required = (total_calories_required / calories_per_bag_of_potatoes).ceil();
    total_bags_required
        .to_i32()
        .expect("Unable to convert calculated bags required to i32.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_cost() {
        assert_eq!(calculate_total_cost(25), 61.75);
    }

    #[test]
    fn test_calculate_required_bags_of_potatoes() {
        assert_eq!(calculate_required_bags_of_potatoes(20), 25);
    }
}
