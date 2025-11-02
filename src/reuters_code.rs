pub fn generate_reuters_code(underlying_name: &str, contract_month: &str) -> String {
    format!("{}{}", underlying_name, contract_month)
}
