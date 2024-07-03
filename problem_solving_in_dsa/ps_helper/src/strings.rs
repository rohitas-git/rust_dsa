pub fn sorted_string(input: &str) -> String {
    if input.is_empty() {
        return input.to_string();
    }
    let mut row = input.chars().map(|c| c as u8).collect::<Vec<u8>>();
    row.sort();
    row.iter().map(|&v| v as char).collect::<String>()
}

