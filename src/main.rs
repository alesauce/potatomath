use rust_decimal::prelude::*;
use rust_decimal_macros::dec;
use std::io;

fn main() {
    println!("Welcome to Potatomath!");
    println!("Please input the number of people you are feeding.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let people = extract_user_input(input);
    println!("You input {} people.", people);
    let total_bags_required: Decimal = calculate_required_bags_of_potatoes(people);
    let total_cost: Decimal = calculate_total_cost(total_bags_required);
    println!("Your total cost per month is: ${total_cost}. Don't forget the multivitamin!")
}

fn extract_user_input(input: String) -> Decimal {
    let people: Decimal = match Decimal::from_str_exact(input.trim()) {
        Ok(n) => n,
        Err(e) => {
            panic!("Error parsing input string '{}': {}", input.trim(), e);
        }
    };
    if !people.is_integer() {
        panic!("You must supply a whole number of people (no fractions or decimals).")
    } else if people.is_sign_negative() {
        panic!("Number of people cannot be negative!")
    }
    people
}

fn calculate_required_bags_of_potatoes(people: Decimal) -> Decimal {
    let calories_per_person = dec!(2000);
    let total_calories_required = calories_per_person * people;
    // Approx. 15 servings per bag of potatoes, 110 calories per serving. Source:
    // https://web.archive.org/web/20240529023336/https://www.walmart.com/ip/Fresh-Whole-Russet-Potatoes-5lb-bag/10447837
    let calories_per_bag_of_potatoes = dec!(1650);
    (total_calories_required / calories_per_bag_of_potatoes).ceil()
}

fn calculate_total_cost(total_bags_required: Decimal) -> Decimal {
    // Cost of a 5 pound bag of potatoes at Wal-mart. Source:
    // https://web.archive.org/web/20240529023336/https://www.walmart.com/ip/Fresh-Whole-Russet-Potatoes-5lb-bag/10447837
    let cost_potatoes: Decimal = dec!(2.47);
    cost_potatoes * total_bags_required
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Number of people cannot be negative!")]
    fn extract_user_input_passed_negative_number_panics() {
        let bad_input: String = String::from("-25");
        extract_user_input(bad_input);
    }

    #[test]
    #[should_panic(
        expected = "You must supply a whole number of people (no fractions or decimals)."
    )]
    fn extract_user_input_passed_non_integer_panics() {
        let bad_input: String = String::from("25.4759");
        extract_user_input(bad_input);
    }

    #[test]
    #[should_panic(
        expected = "Error parsing input string 'not a number': Invalid decimal: unknown character"
    )]
    fn extract_user_input_passed_non_number_panics() {
        let bad_input: String = String::from("not a number");
        extract_user_input(bad_input);
    }

    #[test]
    fn test_extract_user_input() {
        let good_input: String = String::from("75");
        let output: Decimal = extract_user_input(good_input);
        assert_eq!(dec!(75), output);
    }

    #[test]
    fn test_calculate_total_cost() {
        assert_eq!(calculate_total_cost(dec!(25)), dec!(61.75));
    }

    #[test]
    fn test_calculate_required_bags_of_potatoes() {
        assert_eq!(calculate_required_bags_of_potatoes(dec!(20)), dec!(25));
    }
}
