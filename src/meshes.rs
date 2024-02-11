use ggez::{Context, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Color, DrawMode, LineCap, LineJoin, StrokeOptions};

pub fn stopped (width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.circle(DrawMode::Stroke(StrokeOptions::DEFAULT.with_line_width(width)), Vec2::new(0.500, 0.500), 0.4, 0.0001, color);
    mb.line(&[Vec2::new(0.3125, 0.3125), Vec2::new(0.4275, 0.3125)], width, color);
    mb.line(&[Vec2::new(0.4275, 0.3125), Vec2::new(0.4275, 0.6875)], width, color);
    mb.line(&[Vec2::new(0.4275, 0.6875), Vec2::new(0.3125, 0.6875)], width, color);
    mb.line(&[Vec2::new(0.3125, 0.6875), Vec2::new(0.3125, 0.3125)], width, color);

    mb.line(&[Vec2::new(0.5625, 0.3125), Vec2::new(0.6875, 0.3125)], width, color);
    mb.line(&[Vec2::new(0.6875, 0.3125), Vec2::new(0.6875, 0.6875)], width, color);
    mb.line(&[Vec2::new(0.6875, 0.6875), Vec2::new(0.5625, 0.6875)], width, color);
    mb.line(&[Vec2::new(0.5625, 0.6875), Vec2::new(0.5625, 0.3125)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}

pub fn done (width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.circle(DrawMode::Stroke(StrokeOptions::DEFAULT.with_line_width(width)), Vec2::new(0.500, 0.500), 0.4, 0.0001, color);
    mb.line(&[Vec2::new(0.325, 0.500), Vec2::new(0.450, 0.625)], width, color);
    mb.line(&[Vec2::new(0.450, 0.625), Vec2::new(0.700, 0.375)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}
pub fn in_progress (width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.circle(DrawMode::Stroke(StrokeOptions::DEFAULT.with_line_width(width)), Vec2::new(0.500, 0.500), 0.4, 0.0001, color);
    mb.line(&[Vec2::new(0.500, 0.500), Vec2::new(0.500, 0.250)], width, color);
    mb.line(&[Vec2::new(0.500, 0.500), Vec2::new(0.625, 0.625)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}
pub fn cross(width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(&[Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)], width, color);
    mb.line(&[Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}
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

pub fn play_once(width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(&[Vec2::new(0.125, 0.125), Vec2::new(0.125, 0.875)], width, color);
    mb.line(&[Vec2::new(0.125, 0.875), Vec2::new(0.625, 0.500)], width, color);
    mb.line(&[Vec2::new(0.625, 0.500), Vec2::new(0.125, 0.125)], width, color);

    mb.line(&[Vec2::new(0.625, 0.125), Vec2::new(0.625, 0.875)], width, color);
    mb.line(&[Vec2::new(0.625, 0.875), Vec2::new(0.875, 0.875)], width, color);
    mb.line(&[Vec2::new(0.875, 0.875), Vec2::new(0.875, 0.125)], width, color);
    mb.line(&[Vec2::new(0.875, 0.125), Vec2::new(0.625, 0.125)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}

pub fn play_many(width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(&[Vec2::new(0.250, 0.125), Vec2::new(0.250, 0.875)], width, color);
    mb.line(&[Vec2::new(0.250, 0.875), Vec2::new(0.750, 0.500)], width, color);
    mb.line(&[Vec2::new(0.750, 0.500), Vec2::new(0.250, 0.125)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}

pub fn pause(width: f32, color: Color, ctx: &Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(&[Vec2::new(0.125, 0.125), Vec2::new(0.125, 0.875)], width, color);
    mb.line(&[Vec2::new(0.125, 0.875), Vec2::new(0.375, 0.875)], width, color);
    mb.line(&[Vec2::new(0.375, 0.875), Vec2::new(0.375, 0.125)], width, color);
    mb.line(&[Vec2::new(0.375, 0.125), Vec2::new(0.125, 0.125)], width, color);

    mb.line(&[Vec2::new(0.625, 0.125), Vec2::new(0.625, 0.875)], width, color);
    mb.line(&[Vec2::new(0.625, 0.875), Vec2::new(0.875, 0.875)], width, color);
    mb.line(&[Vec2::new(0.875, 0.875), Vec2::new(0.875, 0.125)], width, color);
    mb.line(&[Vec2::new(0.875, 0.125), Vec2::new(0.625, 0.125)], width, color);
    graphics::Mesh::from_data(ctx, mb.build())
}