pub struct Renderer {
    //
}

pub trait Render {
    fn on_start(&self, renderer: &mut Renderer);
    fn on_update(&self, renderer: &mut Renderer);
}

impl Renderer {
    pub fn draw(&mut self) {
        //
    }
}
