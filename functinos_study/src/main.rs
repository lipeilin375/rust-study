fn main() {
    let mut x: i32 = -12;
    x += 12;
    println!("The origin x value is: {}", x);
    let y = {
        let m = x;
        m + 12
    };
    println!("The y value with functional calculate is: {}", y);
    println!("And the x value after functional calculate is: {}", x);
    fn five() {
        let m = 0;
        println!("The \"five\" function's output is: {}", m + 5);
    }
    five();
    let p = plus(12, 17);
    println!("The p value with function calculate is: {}", p);
    println!("UNDER THIS MESSAGE IS THE INFORMATION OF THE DOC FUNCTION");
    doc_test();
}

/// ### The function that you could get the plus action's result
/// for example:
/// ```
/// plus(1,2);
/// ```
/// Then, you will get a return as `3`

/* When you let your cursor hover the function */
/* The upon doc will show as .md file's preview */
fn plus(a: i32, b: i32) -> i32 {
    return a + b;
}

/// The instruction beneath the function won't seen at the upper function's preview
/// and it will also reback a error information to you~
fn doc_test() {
    println!("Hello world!");
}
