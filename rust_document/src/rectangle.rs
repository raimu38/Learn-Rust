#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32{
        self.width * self.height
    }
} 
pub fn rectangle(){
    let rect1 =  Rect{width: 50, height:30};
    println!("rect1: {:#?}",rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());
}

