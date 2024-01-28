use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::Color;

pub fn left_arrow(width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(&[Vec2::new(0.125, 0.500), Vec2::new(0.500, 0.750)], width, color);
    mb.line(&[Vec2::new(0.500, 0.625), Vec2::new(0.875, 0.625)], width, color);
    mb.line(&[Vec2::new(0.875, 0.625), Vec2::new(0.875, 0.375)], width, color);
    mb.line(&[Vec2::new(0.875, 0.375), Vec2::new(0.500, 0.375)], width, color);
    mb.line(&[Vec2::new(0.500, 0.375), Vec2::new(0.500, 0.250)], width, color);
    mb.line(&[Vec2::new(0.500, 0.250), Vec2::new(0.125, 0.500)], width, color);
    mb.line(&[Vec2::new(0.500, 0.750), Vec2::new(0.500, 0.625)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}