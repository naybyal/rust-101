// 12 Days of Christmas

fn main() {
    let days = [
        "first", "second", "third",
        "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth"
    ];

    let stanzas = [
        "partridge in a pear tree","turtle doves,", "French hens,", "calling birds,",
        "golden rings,", "geese a-laying,", "swans a-swimming,",
        "maids a-milking,", "ladies dancing,", "lords a-leaping,",
        "pipers piping,", "drummers drumming,"
    ];

    let count = [
        "One", "Two", "Three", "Four", "Five", "Six",
        "Seven", "Eight", "Nine", "Ten", "Eleven", "Twelve"
    ];

    for (day_index, day) in days.iter().enumerate() {
        if day_index == 0 {
            println!("\nOn the first day of Christmas,\nmy love gave to me\nA partridge in a pear tree.\n");
        }
        else {
            println!("On the {day} day of Christmas,\nmy true love gave to me");
            for stanza_index in (1..=day_index).rev() {
                println!("{} {}", count[stanza_index], stanzas[stanza_index]);
            }
            if day_index < 11 { println!("And a partridge in a pear tree.\n"); }
            else if day_index == 11 { println!("And a partridge in a pear tree!"); }
        }
    }
}