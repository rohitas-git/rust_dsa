
/// sorts a alphabetic string; assumed chars are either all lowercase or all uppercase. 
pub fn sorted_string(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    let mut row = input.chars().map(|c| c as u8).collect::<Vec<u8>>();
    row.sort();
    row.iter().map(|&v| v as char).collect::<String>()
}

/// converts a message into its binary representation
pub fn to_binary(message: &str) -> String {
    message
        .bytes()
        .map(|v| format!("{:07b}", v))
        .collect::<String>()
}
