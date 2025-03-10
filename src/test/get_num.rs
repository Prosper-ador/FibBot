use regex::Regex;
pub fn extract_numbers(text: &str) -> Vec<u64> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.find_iter(text)
        .filter_map(|m| m.as_str().parse::<u64>().ok())
        .collect()
}
/*pub fn extract_numbers(text: &str) -> Vec<u32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    for c in text.chars() {
        if c.is_ascii_digit() {  // More explicit check
            current_number.push(c);
        } else if !current_number.is_empty() {
            if let Ok(num) = current_number.parse::<u32>() {
                numbers.push(num);
            }
            current_number.clear();
        }
    }

    if !current_number.is_empty() {
        if let Ok(num) = current_number.parse::<u32>() {
            numbers.push(num);
        }
    }

    numbers
}*/