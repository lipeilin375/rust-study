fn main() {
    let list = ["R", "P", "E", "A", "T", "T", "E", "S", "T"];
    // * basic while repeat method
    let mut i = 0;
    while i != 4 {
        println!("It's the {} time in while repeat.", i + 1);
        i += 1;
    }
    // * while array repeat method
    let mut i = 0;
    while list[i] != "T" {
        print!("{}", list[i]);
        i += 1;
    }
    // * basic for repeat method
    for i in 0..5 {
        print!("It's {} time repeat by for.", i);
    }
    // * iterator for repeat method
    for i in list.iter() {
        print!("{}", i);
    }
    // * timer iterator for repeat method
    for i in list.iter().take(5) {
        print!("{}", i);
    }
    // * loop basic repeat
    let mut i = 0;
    loop {
        print!("{}", list[i]);
        if i >= 4 {
            break;
        }
        i += 1;
    }
    // * loop to value repeat
    let mut i = 0;
    let five = loop {
        if list[i] == "T" {
            break list[i];
        }
        i += 1;
    };
    print!("The fifth item is: {}.", five);
}
