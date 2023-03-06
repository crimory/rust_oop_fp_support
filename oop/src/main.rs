mod oop_paradigm;
use oop_paradigm::basic;
use oop_paradigm::encapsulation;
use oop_paradigm::polymorphism_du;
use oop_paradigm::polymorphism_trait;

fn basic() {
    let circle = basic::Shape {
        name: "Circle".to_owned(),
    };
    circle.draw_me();
}

fn encapsulation() {
//    let constructed_circle = encapsulation::ConstructedShape {
//        name: "Circle".to_owned(),
//    };
    let constructed_circle = encapsulation::ConstructedShape::new("Constructed Circle".to_owned());
//    constructed_circle.get_description();
    constructed_circle.draw_me();
}

fn polymorphism_du() {
    let circle = polymorphism_du::Circle::new(3.0);
    let square = polymorphism_du::Square::new(5);
    let list = vec![
        polymorphism_du::Shape::Circle(circle),
        polymorphism_du::Shape::Square(square),
    ];
    list.iter().for_each(|s| s.print_area());
    list.iter().for_each(|s| s.draw_me());
}

fn polymorphism_trait() {
    let circle = polymorphism_trait::Circle::new(3.0);
    let square = polymorphism_trait::Square::new(5);
    polymorphism_trait::print_area(&circle);
    polymorphism_trait::print_area(&square);

    let list: Vec<Box<dyn polymorphism_trait::GetName>> = vec![
        Box::new(circle),
        Box::new(square),
    ];
    list.iter()
        .for_each(|s| polymorphism_trait::draw_shape(s.as_ref()));
}

fn main() {
    basic();
    println!("===================");
    encapsulation();
    println!("===================");
    polymorphism_du();
    println!("===================");
    polymorphism_trait();
}
