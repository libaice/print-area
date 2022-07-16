// 定义trait 以及 打印面积的方法
trait areaType {
    fn print_area(&self);
}

// 定义长方体的struct
struct Rectangle {
    width: f32,
    height: f32,
}


// 定义正方体的struct
struct Square {
    width: f32,
}

// 定义圆的struct
struct Circle {
    radius: f32,
}

// 定义三角形的struct
struct Triangle {
    base: f32,
    height: f32,
}


// Impl  ==========================================

impl areaType for Rectangle {
    fn print_area(&self) {
        println!("the Rectangle area is {}", self.height * self.width);
    }
}

impl areaType for Square {
    fn print_area(&self) {
        println!("the Square area is {}", self.width * self.width);
    }
}


impl areaType for Circle {
    fn print_area(&self) {
        println!("the Circle area is {}", 3.1415 * self.radius.powf(2.0));
    }
}

impl areaType for Triangle {
    fn print_area(&self) {
        println!("the Triangle area is {}", self.base * self.height / 2.0);
    }
}


// use generic type
fn area_generic_type<T: areaType>(item: T) {
    item.print_area();
}


fn main() {
    // 构造 长方形，使用泛型打印长方形的面积
    let rect = Rectangle { width: 2.2, height: 2.1 };
    area_generic_type(rect);


    //构造 正方形，使用泛型打印长方形的面积
    let squre = Square { width: 9.0 };
    area_generic_type(squre);

    // 构造 圆形
    let circle = Circle { radius: 4.0 };
    area_generic_type(circle);

    // 构造 三角形

    let triangle = Triangle { base: 2.0, height: 3.0 };
    area_generic_type(triangle);
}
