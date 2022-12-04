fn main() {
    let mut max_calories = 0;
    let mut current_calories = 0;
    include_str!("../input.txt")
        .lines()
        .for_each(|x|
            if x.is_empty() {
                if current_calories > max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0
            } else {
                current_calories += x.parse::<i32>().unwrap()
            }
        );
    println!("{}", max_calories);
}
