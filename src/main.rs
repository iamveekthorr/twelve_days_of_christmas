fn main() {
    let mut lyrics = String::new();
    let gifts: [&str; 12] = [
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

    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    // Generate the 12 days of christmas using range
    for day in 1..=12 {
        lyrics += &base_lyrics(days[day - 1]);
        match day {
            1 => {
                lyrics += &gifts[0];
            }
            _ => {
                lyrics += &gifts[day - 1];
                lyrics += ", \n";

                for gift in (1..=day).rev() {
                    if gift == 1 {
                        lyrics += "and ";
                    }
                    lyrics += &gifts[gift - 1];
                    lyrics += ",\n";
                }
            }
        }

        lyrics += "\n";
    }

    println!("{lyrics}");
}

fn base_lyrics(day: &str) -> String {
    format!("on the {day} day of christmas my true love sent to me\n")
}
