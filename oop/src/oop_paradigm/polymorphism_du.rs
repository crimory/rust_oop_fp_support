struct InnerShape {
    name: String,
}

pub struct Circle {
    shape: InnerShape,
    radius: f32,
}
impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle {
            shape: InnerShape {
                name: "Circle".to_owned(),
            },
            radius,
        }
    }
    fn get_area(&self) -> f32 {
        self.radius.powi(2) * std::f32::consts::PI
    }
}

pub struct Square {
    shape: InnerShape,
    side: u16,
}
impl Square {
    pub fn new(side: u16) -> Self {
        Square {
            shape: InnerShape {
                name: "Square".to_owned(),
            },
            side,
        }
    }
    fn get_area(&self) -> u16 {
        self.side.pow(2)
    }
}

pub enum Shape {
    Circle(Circle),
    Square(Square),
}
impl Shape {
    pub fn draw_me(&self) {
        let name = match self {
            Shape::Circle(c) => &c.shape.name,
            Shape::Square(s) => &s.shape.name,
        };
        println!("This is a drawing by discriminated union of: {}", name);
    }
    pub fn print_area(&self) {
        let area = match self {
            Shape::Circle(c) => c.get_area(),
            Shape::Square(s) => s.get_area() as f32,
        };
        println!("Area by discriminated union is: {}", area);
    }
}
