use axum::extract::Path;

// Task 1 + Extra

pub async fn cube_the_bits(Path(nums): Path<String>) -> String {
    let mut numbers: Vec<&str> = nums.split('/').collect();

    if numbers.last().unwrap().is_empty() {
        numbers.pop();
    }

    println!("{numbers:?}");

    let numbers = numbers
        .iter()
        .map(|n| n.parse::<i64>().unwrap())
        .fold(0, |acc, n| acc ^ n)
        .pow(3)
        .to_string();

    numbers
}