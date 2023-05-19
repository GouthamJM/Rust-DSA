pub fn create_acronym(source:&str)->String{
    source
    .split_whitespace()
    .flat_map(|s| s.chars().nth(0))
    .collect::<String>()
    .to_ascii_uppercase()
}