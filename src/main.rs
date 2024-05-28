use std::io;

fn calculate_total_cost(people: i32) -> i32 {
    // Cost of a 5 pound bag of potatoes at Wal-mart. Source:
    // https://www.perplexity.ai/search/cost-of-a-eldJfjumS3if3xm5yIQ4GA
    let cost_potatoes: i32 = 3;
    let pounds_potatoes_per_person: i32 = 5;
    let cost_per_person: i32 = cost_potatoes * pounds_potatoes_per_person;
    people * cost_per_person
}

fn main() {
    println!("Please input the number of people you are feeding.");
    let mut people_input = String::new();
    io::stdin()
        .read_line(&mut people_input)
        .expect("Failed to read line");
    println!("You input {people_input} people.");
    let num_people: i32 = match people_input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error parsing string '{}': {}", people_input, e);
            0
        }
    };
    let total_cost: i32 = calculate_total_cost(num_people);
    println!("Your total cost per month is: ${total_cost}. Don't forget the multivitamin!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_cost() {
        assert_eq!(calculate_total_cost(5), 75);
    }
}
