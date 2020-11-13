fn main() {
    let days = ["second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth"];
    let verses = [
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds,",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,"
    ];

    println!("On the first day of Christmas my true love sent to me");
    println!("A partridge in a pear tree.");
    println!();

    let mut day_index = 0;

    while day_index < days.len() {
        println!("On the {} day of Christmas my true love sent to me", days[day_index]);

        let verse_concat = verses.iter().take(day_index + 1).rev();

        for verse in verse_concat {
            println!("{}", verse);
        }

        println!("And a partridge in a pear tree.");
        println!();
        day_index += 1;
    }
}
