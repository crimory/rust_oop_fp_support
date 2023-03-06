pub struct Shape {
    pub name: String,
}
impl Shape {
    pub fn draw_me(&self) {
        println!("This is a basic drawing of: {}", self.name);
    }
}
