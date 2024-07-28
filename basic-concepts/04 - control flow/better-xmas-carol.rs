fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
    ];
    let stanzas = [
        "a partridge in a pear tree", "two turtle doves,", "three French hens,", "four calling birds,", "five golden rings,", "six geese a-laying,", "seven swans a-swimming,", "eight maids a-milking,", "nine ladies dancing,", "ten lords a-leaping,", "eleven pipers piping,", "twelve drummers drumming,"
    ];
    for day_index in 0..12 {
        println!("\nOn the {} day of Christmas,\nmy true love gave to me", days[day_index]);
        for stanza_index in (0..=day_index).rev() {
            if day_index == 11 && stanza_index == 0 {
                println!("and {}!", stanzas[stanza_index]);
            } else if day_index > 0 && stanza_index == 0 {
                println!("and {}.", stanzas[stanza_index]);
            } else {
                println!("{}", stanzas[stanza_index]);
            }
        }
    }
}