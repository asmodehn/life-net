use macroquad::prelude::{Color, IVec2, Image, UVec2};

pub trait Drawable {
    fn draw(&self, position_in_screen: IVec2);
}

pub trait Updatable {
    fn update(&mut self, image: &Image);
}

pub(crate) trait Quad {
    // TODO : make it drawable (careful with views)
    fn get_dimensions(&self) -> UVec2;
    fn get_background(&self) -> Color; // TODO : move into drawable ??

    fn set_dimensions(&mut self, dimensions: UVec2);

    fn set_background(&mut self, color: Color);

    //TODO : make scale another trait (storing scale changes to keep original pixel-perfect)
    fn scale(&mut self, factor: u32);
}

pub trait Placed {
    // TODO : instead of "plceD" adjective. think about "Place" as noun. and add an option to all element that may have a place...
    fn get_position(&self) -> IVec2;

    fn set_position(&mut self, position: IVec2);

    fn translate(&mut self, displacement: IVec2);
}
