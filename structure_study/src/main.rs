#[allow(unused)]
fn main() {
    let o1 = Rect {
        width: 25,
        height: 35,
        length: 45,
    };
    let o2 = Rect { height: 55, ..o1 };
    struct Point(u32, u32, u32);
    let o2p = Point(o2.width, o2.height, o2.length);
    // Format print structure
    println!("o1 is: {:#?}", o1); // ! This print method is used under the 23 line
    // Only print structure
    println!("o2 is: {:?}", o2); // ! This print method is used under the 23 line
    println!();
    let info = o2.sqaure();
    println!("The three sqaure is: {}+{}+{}", info.s1, info.s2, info.s3);
    println!("The volume is: {}", o2.vol(2));
    println!();
    println!("o2 Point location is: {} {} {}", o2p.0, o2p.1, o2p.2);
}

#[derive(Debug)] // This define is to debug when you want display the variable under this structure's format
struct Rect {
    width: u32,
    height: u32,
    length: u32,
}

impl Rect {
    fn sqaure(&self) -> Reback {
        let s1 = self.width * self.height;
        let s2 = self.width * self.length;
        let s3 = self.height * self.length;
         Reback { s1, s2, s3 }
    }
    fn vol(&self, w: u32) -> u32 {
        let volume = self.height * self.width * self.length;
        volume*w
    }
}

struct Reback {
    s1: u32,
    s2: u32,
    s3: u32,
}
