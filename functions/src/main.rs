fn main() {
    println!("Hello, world!");
    another_function(22, 'a');
}

fn another_function(x: i32, unit_lable: char) {
    println!("Her ein another function");
    println!("Value of arg is {x} ad label is {unit_lable}");
}
