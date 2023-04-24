use std::io;
mod factorial;
fn main() {
    println!("Please enter the number of factorial you are looking for:");

    let mut expected_factorial: String = String::new();

    io::stdin()
        .read_line(&mut expected_factorial)
        .expect("Failed to read line");

    let expected_factorial: i64 = expected_factorial
        .trim()
        .parse()
        .expect("Numbers only, please.");

    let answer = factorial::factorial(expected_factorial);

    println!("The answer is: {answer}")
}
