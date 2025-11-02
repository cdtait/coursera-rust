use coursera_rust::reuters_code::generate_reuters_code;

#[test]
fn test_basic_reuters_code_generation() {
    assert_eq!(generate_reuters_code("AAPL", "202511"), "AAPL202511");
    assert_eq!(generate_reuters_code("GOOGL", "202512"), "GOOGL202512");
}

#[test]
fn test_empty_inputs() {
    assert_eq!(generate_reuters_code("", ""), "");
    assert_eq!(generate_reuters_code("AAPL", ""), "AAPL");
    assert_eq!(generate_reuters_code("", "202511"), "202511");
}

#[test]
fn test_special_characters() {
    assert_eq!(generate_reuters_code("ES-", "202512"), "ES-202512");
    assert_eq!(generate_reuters_code("NQ/", "202512"), "NQ/202512");
}

#[test]
fn test_lowercase_inputs() {
    assert_eq!(generate_reuters_code("aapl", "202511"), "aapl202511");
    assert_eq!(generate_reuters_code("googl", "202512"), "googl202512");
}
