
// On the first day of Christmas,
// my true love gave to me
// A partridge in a pear tree.
// On the second day of Christmas,
// my true love gave to me
// Two turtle doves,
// And a partridge in a pear tree.
// On the third day of Christmas,
// my true love gave to me
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the fourth day of Christmas,
// my true love gave to me
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the fifth day of Christmas,
// my true love gave to me
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the sixth day of Christmas,
// my true love gave to me
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the seventh day of Christmas,
// my true love gave to me
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the eighth day of Christmas,
// my true love gave to me
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the ninth day of Christmas,
// my true love gave to me
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the tenth day of Christmas,
// my true love gave to me
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the eleventh day of Christmas,
// my true love gave to me
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree.
// On the twelfth day of Christmas,
// my true love gave to me
// Twelve drummers drumming,
// Eleven pipers piping,
// Ten lords a-leaping,
// Nine ladies dancing,
// Eight maids a-milking,
// Seven swans a-swimming,
// Six geese a-laying,
// Five golden rings,
// Four calling birds,
// Three French hens,
// Two turtle doves,
// And a partridge in a pear tree!

fn main() {
    let counts = ["first", "second", "third", "fourth",
    "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", 
    "twelfth",];

    let gifts = [
        "And a partridge in a pear tree",
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

    let first_gift = "A partridge in a pear tree.\n";

    for i in 1..=12 {
        println!("On the {0} day of Christmas, my true love gave to me", 
            counts[i - 1]);

        if i == 1 {
            println!("{}", first_gift);
        }
        else {
            for j in (1..=i).rev() {
                print!("{0}", gifts[j - 1]);
                println!{"{0}", if j > 1 {"," } else {".\n"}}
            }
        }
    }

    
}


