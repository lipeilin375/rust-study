use std::fmt::Debug;

/// Book <enum> structure
///
/// ``` paper -> a book made from wood```
///
/// ``` elect -> a book only refers to the electronic version```
#[derive(Debug)]
enum Book {
    Paper,
    Elect,
}
#[derive(Debug)]
enum Info {
    Name(String),
    Sex(bool),
}
#[derive(Debug)]
enum Light {
    Color { num: i32 },
}
enum Option<T> {
    Some(T),
    None,
}
fn main() {
    let p_book = Book::Paper;
    let e_book = Book::Elect;
    println!("{:?}", p_book);
    println!("{:#?}", e_book);
    println!();
    let stu1_name = Info::Name("student1".to_string());
    let stu1_sex = Info::Sex(true);
    println!("{:?}", stu1_name);
    println!("{:#?}", stu1_sex);
    println!();
    let l_color1 = Light::Color { num: 23 };
    let l_color2 = Light::Color { num: 54 };
    println!("{:?}", l_color1);
    println!("{:#?}", l_color2);
    println!();
    println!(
        "The state of the color1 in light is: {}",
        light_color_judge(l_color1)
    );
    println!();
    let some = Option::Some("abc".to_string());
    match some {
        Option::Some(t) => {
            println!("The content of some is: {}", t);
        }
        Option::None => {
            println!("The some is None!");
        }
    }
    let nan: Option<&str> = Option::None;
    match nan {
        Option::Some(t) => {
            println!("The content of nan is: {}", t);
        }
        Option::None => {
            println!("The nan is None!");
        }
    }
    println!();
    // * More interests on match!
    for i in 0..3 {
        let num = i;
        match num {
            0 => {
                println!("The num is 0!!");
            }
            1 => {
                println!("The num is 1!!");
            }
            _ => {
                println!("The num is not 1 or 0!!");
            }
        }
    }
    println!();
    // * if-let grammar
    // let mul = Info::Sex(true);
    let mul = Info::Name("Name".to_string());
    if let Info::Name(name) = mul {
        println!("The name is: {}", name);
    } else if let Info::Sex(sex) = mul {
        println!("The sex is: {}", sex);
    } else {
        println!("{:?}", mul);
    }
}

fn light_color_judge(i: Light) -> bool {
    // ! For <enum>, only can use match method to get the value in variable
    match i {
        Light::Color { num } => num >= 30,
    }
}
