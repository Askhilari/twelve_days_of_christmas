fn main(){
    let a = [
        "A patridge in a pear tree", 
        "Two Turtle Doves", 
        "Three French Hens", 
        "Four Calling Birds",
        "Five Golden Rings",
        "Six Geese a-laying",
        "Seven swans a-swimming",
        "Eight Malds a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
        ];

    for i in 1..(a.len()+1) {
        for j in 0..i{
            println!("{}", a[j]);
        }
        println!("");
    }
}
