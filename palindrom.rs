fn main() {
    println!("Word is it palindrom: {:?}", palindrom("alla "));
}

fn palindrom(s: &str) -> bool {
    let rev_s: String = s
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .rev()
        .collect();
    let remove_whitespace: String = s
        .to_lowercase()
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    println!("rev_s ->{:?}", &rev_s);
    if rev_s == remove_whitespace {
        true
    } else {
        false
    }
}
