pub trait GetName {
    fn get_name(&self) -> &str;
}
pub trait GetArea<T> {
    fn get_area(&self) -> T;
}

pub struct Circle {
    name: String,
    radius: f32,
}
impl Circle {
    pub fn new(radius: f32) -> Self {
        Circle {
            name: "Circle".to_owned(),
            radius,
        }
    }
}
impl GetName for Circle {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
impl GetArea<f32> for Circle {
    fn get_area(&self) -> f32 {
        self.radius.powi(2) * std::f32::consts::PI
    }
}

pub struct Square {
    name: String,
    side: u16,
}
impl Square {
    pub fn new(side: u16) -> Self {
        Square {
            name: "Square".to_owned(),
            side,
        }
    }
}
impl GetName for Square {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }
}
impl GetArea<u16> for Square {
    fn get_area(&self) -> u16 {
        self.side.pow(2)
    }
}

pub fn draw_shape(shape: &dyn GetName) {
    println!("This is a drawing by trait of: {}", shape.get_name());
}

pub fn print_area<T>(shape: &impl GetArea<T>)
where
    T: std::fmt::Display,
{
    println!("Area by trait is: {}", shape.get_area());
}
