use game::board::Board;
use subsystem::Subsystem;


pub type Point = (i32, i32);


pub trait Renderer: Subsystem {
    fn block(&mut self, pos: Point);
    fn text(&mut self, pos: Point, text: String);
    fn board(&mut self, pos: Point,board: Board);
}


pub trait Renderable {
    fn render(&self, pos: Point, renderer: &mut Renderer);
}