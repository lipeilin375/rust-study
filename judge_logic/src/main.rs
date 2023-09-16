fn main() {
    // basic logic judgement method
    // ! Notice: you can't use RUST to check if a variable is exist or not equl to 0
    // ! If you want, you have to judge it by yourself before judgement
    let number = 12;
    if number % 2 == 1 {
        println!("The number is Odd -> {}", number);
    } else if number % 2 == 0 {
        println!("The number is Even -> {}", number);
    } else {
        println!("I don't know whether it is Odd or Even -> {}", number)
    }
    println!("**************************************************");
    let tnum1 = -3;
    let tnum2 = 3;
    let numstatus1 = if tnum1 > 0 { 1 } else { -1 };
    let numstatus2 = if tnum2 > 0 { 1 } else { -1 };
    println!("The first number's status is: {}", numstatus1);
    println!("The second number's status is: {}", numstatus2);
}
