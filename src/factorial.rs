pub fn factorial(number: i64) -> i64 {
    if number < 2 {
        return number;
    }

    return number * factorial(number - 1);
}
