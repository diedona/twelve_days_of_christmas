fn main() {
    let phrases: [&str; 4] = get_phrases_days_of_christmas();
    let days: [u8; 12] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    for day in days {
        println!("{day}");
    }
}

fn get_phrases_days_of_christmas<'a>() -> [&'a str; 4] {
    return [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
    ];
}
