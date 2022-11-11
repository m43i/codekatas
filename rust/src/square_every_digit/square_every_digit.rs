fn square_digits(num: u64) -> u64 {
    num.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .map(|c| c.pow(2))
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::square_digits;

    #[test]
    fn test_square_digits() {
        assert_eq!(square_digits(9119), 811181, "\nFailed with num 9119");
    }
}
