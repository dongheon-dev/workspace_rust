fn main() {
    let x = 5;
    println!("x = {}", x)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_ok() {
        let result = 1 + 1;
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fail() {
        let result = 1 + 1;
        assert_eq!(result, 3);
    }
}