pub struct ConstructedShape {
    name: String,
}
impl ConstructedShape {
    pub fn new(name: String) -> Self {
        ConstructedShape { name }
    }
    fn get_description(&self) -> String {
        format!("This is a basic drawing of: {}", self.name)
    }
    pub fn draw_me(&self) {
        println!("{}", self.get_description());
    }
}