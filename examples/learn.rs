use std::fmt::{Display, Formatter};
struct Rectangle{
    with:u32,
    height:u32
}
enum Shape{
    Rectangle(Rectangle),
    Triangle,
    Circle,
}

enum Color{
    Red,
    Green,
    Blue,
}
fn main()  {
    let Colo = Color::Red;
    let  red = match Colo {
        Color::Red => {1}
        Color::Green => {2}
        Color::Blue => {3}
    };
    println!("{}",red);
   let _shape = Shape::Rectangle(Rectangle{with:10,height:20});
    let i = match _shape {
        Shape::Rectangle(rectangle) => {
            println!("{}", rectangle.with);
           8
        },
        _ => {
            8
        }
    };
    let a = i;
    let _string = "这个是一个字符串"; // &str
    let string1:&'static str = "xsxsx"; //  程序的生命周期内都有效 &str
    let a1 = "新的字符串".to_string(); //  具有所有权的字符串
    println!("{}--{}",string1,a1);
    let a3 = a1.as_str();
    let a2 = _string.to_string() + a1.as_str();
    println!("{:?}--a2",a);
}
// help: if this is intentional, prefix it with an underscore: `_string`