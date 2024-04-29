trait  Operation{
    fn add(&mut self)->Self;
}
impl  Operation for i32{
    fn add(&mut self) ->Self{
        *self+=1;
        *self
    }
}
fn main() {
    let mut a = 8;
    println!("{}",a.add());
    println!("Hello, world{}!",a);
    // println!("Hello, world!");
}