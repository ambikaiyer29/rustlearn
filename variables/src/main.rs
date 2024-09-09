fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    let tup = (1, 2.0, 500);

    println!("Value of first index {}", tup.0);

    let (x,y,z) = tup;

    println!("Value of 2nds index {y}");
}
