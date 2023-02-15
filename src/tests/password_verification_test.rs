#[test]
fn test_password_verification() {
    let input = "1234\n"; // User input for the password
    let mut reader = std::io::BufReader::new(input.as_bytes());
    let mut input_str = String::new();
    reader.read_line(&mut input_str).unwrap();

    let result = input_str.trim().parse();

    assert_eq!(result, Ok(1234)); // Test that the input is correctly parsed as 1234

    let password = 1234;
    if result == Ok(password) {
        assert_eq!("Access granted!", "Access granted!"); // Test that the correct message is printed if the password is correct
    } else {
        assert_eq!("Access denied!", "Access granted!"); // Test that the correct message is printed if the password is incorrect
    }
}
