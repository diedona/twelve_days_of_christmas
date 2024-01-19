fn main() {
    let gifts: Vec<&str> = get_gifts();
    let days: [usize; 12] = core::array::from_fn(|day| day + 1);

    for day in days {
        print_chorus(&day, &gifts);
    }
}

fn print_chorus(day: &usize, gifts: &[&str]) {
    println!("On the {} day of christmas,", format_day(day));
    println!("my true love gave to me...\n");

    print_gifts(day, gifts);
    println!("---");
}

fn format_day(day: &usize) -> &str {
    return match day {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "nineth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelvth",
        _ => "oopsie",
    };
}

fn print_gifts(day: &usize, gifts: &[&str]) {
    if *day == 0 {
        return;
    }

    let mut current_gift_idx = (*day - 1) as usize;
    loop {
        let current_gift = match gifts.get(current_gift_idx) {
            Some(gift) => gift.to_string(),
            None => format!("idx {} - an oopsie? gift out of range", current_gift_idx),
        };

        println!(
            "{}",
            format_current_gift_in_phrase(&current_gift, day, &current_gift_idx)
        );
        if current_gift_idx < 1 {
            break;
        }

        current_gift_idx -= 1;
    }
}

fn format_current_gift_in_phrase<'a>(
    current_gift: &String,
    day: &usize,
    current_gift_idx: &usize,
) -> String {
    if *day == 1 && *current_gift_idx == 0 {
        return format!("{}!!!", current_gift);
    }

    if *day > 1 && *current_gift_idx == 0 {
        return format!("... and {}!", current_gift.to_lowercase());
    }

    return format!("{},", current_gift);
}

fn get_gifts() -> Vec<&'static str> {
    return vec![
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
}
