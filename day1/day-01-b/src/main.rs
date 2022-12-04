fn main() {
    let mut calories: Vec<i32> = vec![];
    let mut current_calories = 0;
    include_str!("../input.txt").lines().for_each(|x| {
        if x.is_empty() {
            calories.push(current_calories);
            current_calories = 0
        } else {
            current_calories += x.parse::<i32>().unwrap()
        }
    });
    calories.sort();
    println!(
        "{}",
        calories[calories.len() - 1] + calories[calories.len() - 2] + calories[calories.len() - 3]
    );
}
