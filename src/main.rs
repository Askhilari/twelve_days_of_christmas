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

    println!("Simple triangle pattern");
    triangle(5);
    println!("Reverse triangle pattern");
    reverse_triangle(5);
    println!("Triangle pattern with only odd starts in rows");
    odd_stars(5);
    println!("Diamond Patterns");
    diamond_pattern(5);
    
}


    // Following code prints * in a triangle 1 - 5 format.
fn triangle(n: usize) {
    for i in 1..n+2 {
        for _j in 1..i {
            print!("*");
        }
        println!("");
    }
}

    // Following code prints the above pattern in reverse
fn reverse_triangle(n: usize) {    
    for i in (1..n+2).rev(){
        for _j in 1..i{
            print!("*");
        }

        println!("");
    }
}

// The following code prints odd number of stars in each line
fn odd_stars(n: usize) {
    for i in 0..n {
        for _j in 0..(2*i + 1) {
            print!("*");
        }
        println!("");
    }
}

fn diamond_pattern(n: usize){
    let mut space = n;
    for i in 1..n{
        print!("{}", " ".repeat(space));
        for _j in 1..(2*i){
            print!("*");
        }
        space -= 1;
        println!(" ");
    }

    for i in (1..=n).rev(){
        print!("{}", " ".repeat(space));
        for _j in 1..(2*i){
            print!("*");
        }
        space += 1;
        println!(" ");
    }
}

