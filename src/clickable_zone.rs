use ggez::graphics;
use ggez::graphics::{Canvas, Rect};
use ggez::mint::{Point2, Vector2};

pub struct ClickableZone {
    pub position: Point2<f32>,
    pub size: Vector2<f32>,
    pub mesh_for_draw: Option<graphics::Mesh>,
    pub mesh_for_draw_at_hover: Option<graphics::Mesh>
}

impl ClickableZone {
    pub fn new(position: Point2<f32>, size: Vector2<f32>) -> Self {
        Self {
            position,
            size,
            mesh_for_draw: None,
            mesh_for_draw_at_hover: None
        }
    }
    pub fn set_mesh_for_draw(&mut self, mesh: graphics::Mesh) {
        self.mesh_for_draw = Some(mesh)
    }
    pub fn set_mesh_for_draw_at_hover(&mut self, mesh: graphics::Mesh) {
        self.mesh_for_draw_at_hover = Some(mesh)
    }
    pub fn in_clickable_zone(&self, position: Point2<f32>) -> bool {
        position.x >= self.position.x
            && position.y >= self.position.y
            && position.x <= self.position.x + self.size.x
            && position.y <= self.position.y + self.size.y
    }

    fn draw_mesh(&self, canvas: &mut Canvas, mesh: &graphics::Mesh) {
        canvas.draw(
            mesh,
            graphics::DrawParam::new()
                .dest_rect(Rect::new(
                    self.position.x,
                    self.position.y,
                    self.size.x,
                    self.size.y
                ))
        )
    }
    pub fn draw(&self, mouse_position: Point2<f32>, canvas: &mut Canvas) {
        if self.in_clickable_zone(mouse_position) {
            match &self.mesh_for_draw_at_hover {
                Some(mesh) => {
                    self.draw_mesh(canvas, &mesh)
                },
                None    => {},
            }
        }
        else {
            match &self.mesh_for_draw {
                Some(mesh) => {
                    self.draw_mesh(canvas, &mesh)
                },
                None    => {},
            }
        }
    }
}