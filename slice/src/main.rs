fn main() {
    // Character string slice
    println!("The result below is character string slice below");
    let a = "hello,world";
    let s = &a[0..5];
    for i in s.chars() {
        print!("{}", i);
    }
    println!();
    let m = &a[..5];
    for i in m.chars() {
        print!("{}", i);
    }
    println!();
    // Array slice
    println!("The result below is array slice methos");
    let l = ["h", "e", "l", "l", "o", ",", "w", "o", "r", "l", "d"];
    let r = &l[6..];
    for i in r.iter() {
        print!("{}", i);
    }
    println!();
}
