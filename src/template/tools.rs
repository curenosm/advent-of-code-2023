pub fn parse_number_list(numbers_str: &str) -> Vec<u32> {
    numbers_str
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(clean_row)
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

pub fn clean_row(row: &str) -> &str {
    if row.ends_with('\n') || row.ends_with('\r') {
        row[..row.len() - 1].trim()
    } else {
        row.trim()
    }
}
