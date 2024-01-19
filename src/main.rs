fn main() {
    let phrases: Vec<&str> = get_phrases_days_of_christmas();
    let days: [u8; 4] = [1, 2, 3, 4];

    for day in days {
        print_chorus(&day, &phrases);
    }
}

fn print_chorus(day: &u8, phrases: &Vec<&str>) {
    println!("On the {day} day of christmas,");
    println!("my true love gave to me...\n");

    print_gifts(day, phrases);
    println!("---");
}

fn print_gifts(day: &u8, phrases: &Vec<&str>) {
    let mut current_day = *day as usize;
    current_day -= 1;

    loop {
        let current_phrase = phrases
            .get(current_day)
            .expect("current_day out of range?");
       
        println!("{}", format_current_phrase(&current_phrase, &day, &current_day));
        if current_day < 1 {
            break;
        }

        current_day -= 1;
    }
}

fn format_current_phrase<'a>(
    current_phrase: &str,
    day: &u8,
    current_day: &usize
) -> String {

    if day > &1 && current_day == &0 {
        return format!("... and {}!", current_phrase.to_lowercase());
    }

    return format!("{}, ", current_phrase);
}

fn get_phrases_days_of_christmas<'a>() -> Vec<&'a str> {
    return vec![
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying"
    ];
}
