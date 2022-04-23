
fn main() {
 
    let square = square::Square::new(10, 20);
    print_area(&square);

    let circle = circle::Circle::new(10);
    print_area(&circle);

    let trianglef = trianglef::Trianglef::new(20, 10);
    print_area(&trianglef);
}

///面积
trait Area {

    ///计算面积
    fn calculate(&self) -> f32; 
}

mod square {
    use crate::Area;

    ///正（长）方形
    pub struct Square {
        width: u32,
        height: u32
    }

    impl Square {
        pub fn new(width: u32, height:u32) -> Square {
            Square {
                width,
                height,
            }
        }
    }

    impl Area for Square {

        fn calculate(&self) -> f32 {
          (self.width * self.height) as f32 
        }
    }
}


///圆形
mod circle {
    use crate::Area;
    use std::f32::consts::PI;

    pub struct Circle {

        ///直径
        diameter: u32
    }

    impl Circle {
        pub fn new(diameter: u32) -> Circle {
            Circle{diameter}
        }
    }

    impl Area for Circle {

        fn calculate(&self) -> f32 {
            (self.diameter as f32) * PI
        }
}
}


mod trianglef{
    use crate::Area;
///三角形
    pub struct Trianglef {
        bottom: u32,
        height: u32
    }

    impl Trianglef {
        pub fn new(bottom:u32, height: u32) -> Trianglef{
            Trianglef{bottom, height}
        }
    }

    impl Area for Trianglef {

        fn calculate(&self) -> f32 {
            (self.bottom * self.height) as f32
        }
    }
}


fn print_area<T: Area>(draw: &T){
    println!("Print area={}", draw.calculate())
}