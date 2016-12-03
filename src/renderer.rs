use subsystem::Subsystem;


pub type Point = (i32, i32);


pub trait Renderer: Subsystem {
    fn block(&mut self, pos: Point);
    fn text(&mut self, text: String, pos: Point);
}


pub trait Renderable {
    fn render(&self, pos: Point, renderer: &mut Renderer);
}