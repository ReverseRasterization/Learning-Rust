fn main() {
    let numerical_words = ["second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lines = [
    "And a partridge in a pear tree!",
    "Two turtle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leaping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,"
    ];

    println!("On the first day of christmas\nMy true love sent to me\nA partridge in a pear tree.");

    let mut i = 1;
    for word in numerical_words {
        println!("On the {0} day of Christmas\nmy true love sent to me", word);

        let mut a = i;

        loop {

            if a == 0 {
                println!("{0}", lines[a]);

                break;
            }

            println!("{0}", lines[a]);
            a -= 1;

            
        }

        i += 1;
    }
}
