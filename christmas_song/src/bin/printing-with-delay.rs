use std::{thread, time};

fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a leaping",
        "Nine ladies dancing",
        "Eight maids a milking",
        "Seven swans a swimming",
        "Six geese a laying",
        "Five golden rings",
        "Four calling birds",
        "Three French hens",
        "Two turtle doves",
        "A partridge in a pear tree",
    ];

    let delay = time::Duration::from_millis(500); // 0.5 second delay

    for day in 0..days.len() {
        println!("\n🎄 On the {} day of Christmas, my true love gave to me:", days[day]);
        thread::sleep(delay);

        for gift_index in (gifts.len() - day - 1)..gifts.len() {
            if gift_index == gifts.len() - 1 && day != 0 {
                println!("And {}", gifts[gift_index]);
            } else {
                println!("{}", gifts[gift_index]);
            }

            thread::sleep(delay); // delay between gifts
        }
    }
}