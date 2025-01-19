struct Circle {
    radius: f64,
}

struct Rectangle{
    width: f64,
    height: f64,
}

trait Shape {
    fn area(&self) -> f64; // 面積を計算するメソッド
    fn perimeter(&self) -> f64; // 周囲長を計算するメソッド
}

impl Shape for Circle {
    fn area(&self) -> f64{
        std::f64::consts::PI * self.radius * self.radius
    }
    fn perimeter(&self) -> f64{
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl Shape for Rectangle{
    fn area(&self) -> f64{
        self.width * self.height
    }
    fn perimeter(&self)-> f64{
    2.0*(self.width + self.height)
    }
}

fn print_shape_info<T: Shape>(shape: &T){
    println!("Area:{}",shape.area());
    println!("Perimeter:{}", shape.perimeter());
}

pub fn circle_rectangle(){
    let circle = Circle{radius: 6.0};
    let rectangle = Rectangle{width:5.0, height:7.0};

    println!("Circle");
    print_shape_info(&circle);

    println!("Rectangle");
    print_shape_info(&rectangle);
}