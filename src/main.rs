fn main() {
    say_hello();
    let x = 1;
    let y = 2;
    say_the_sum(x, y);
    say_a_number(x as i32);
    println!("Hello, world! new message");
}

fn say_hello () {
    println!("Hello!");
    say_a_number(15);
}

fn say_a_number(number:i32){
    println!("Number is {}", number);
}

fn say_the_sum(a: u8, b: u8){
    let sum = a + b;
    println!("The sum is {}", sum);
}