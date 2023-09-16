fn main() {
    method_1();

    method_2();
}

fn method_1() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x in method 1 is: {}", x);
}

fn method_2() {
    let x = "hello";

    let x = x.len();
    println!("The value of x in method 2 is: {}", x);
}
