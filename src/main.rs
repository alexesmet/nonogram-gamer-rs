#![allow(unused)]

mod grid;
mod description;

use std::path;

use clap::Parser;
use description::LevelDescriptionTemplate;
use ggez::glam::Vec2;
use ggez::winit::dpi::{Size, PhysicalSize, LogicalSize};
use ggez::{Context, ContextBuilder, GameResult, GameError, mint};
use ggez::graphics::{self, Color, Text, TextFragment, PxScale, TextLayout};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::mint::Point2;
use serde::{Serialize, Deserialize};
use crate::grid::GameState;

const CELL_SIZE: f32 = 50.0;
const MAIN_FONT: &'static str = "LiberationMono";

#[derive(Parser)]
#[command()]
struct Cli {
    pub level_path: String
}

fn main() -> GameResult {
    let cli = Cli::parse();
    let lvl_desc = LevelDescriptionTemplate::from_file(&cli.level_path)?;

    let resource_dir = if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("Nonogram-gamer", "Alexei + Dmitri")
        .add_resource_path(resource_dir)
        .build()?;

    let my_game = MyGame::new(&mut ctx, lvl_desc);

    // Run!
    event::run(ctx, event_loop, my_game);
}






struct MyGame {
    max_nums_in_rows: usize,
    max_nums_in_cols: usize,
    background_mesh: graphics::Mesh,
    cross_mesh: graphics::Mesh,
    cross_cells: Vec::<graphics::DrawParam>,
    dark_cells: Vec::<graphics::DrawParam>,
    game_state: GameState,
}

pub fn cell_num_to_coord(shift_in_cells: usize) -> f32 {
    shift_in_cells as f32 * CELL_SIZE
}

impl MyGame {
    pub fn new(ctx: &mut Context, lvl_desc: LevelDescriptionTemplate) -> MyGame {
        // Eager evaluation of screen size and margins
        let max_nums_in_rows = lvl_desc.rows.iter().map(|r| r.len()).max().unwrap_or(0);
        let max_nums_in_cols = lvl_desc.cols.iter().map(|r| r.len()).max().unwrap_or(0);
        let screen_size = LogicalSize::new(
            cell_num_to_coord(max_nums_in_rows + lvl_desc.cols.len()) + 2.0,
            cell_num_to_coord(max_nums_in_cols + lvl_desc.rows.len()) + 2.0,
        );
        ctx.gfx.window().set_inner_size(screen_size);
        // Load fonts
        ctx.gfx.add_font(MAIN_FONT, graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf").unwrap());
        // Prapare background
        let mb = &mut graphics::MeshBuilder::new();
        mb.rectangle(
            graphics::DrawMode::stroke(4.0),
            graphics::Rect::new(
                cell_num_to_coord(max_nums_in_rows), 
                cell_num_to_coord(max_nums_in_cols), 
                cell_num_to_coord(lvl_desc.cols.len()), 
                cell_num_to_coord(lvl_desc.rows.len())),
            graphics::Color::BLACK
        );
        for i in 1..lvl_desc.rows.len() {
            let h = cell_num_to_coord(max_nums_in_cols + i);
            mb.line(&[Vec2::new(0.0, h), Vec2::new(screen_size.width, h)], 2.0, Color::from_rgb(50, 99, 168));
        }
        for i in 1..lvl_desc.cols.len() {
            let w = cell_num_to_coord(max_nums_in_rows + i);
            mb.line(&[Vec2::new(w, 0.0), Vec2::new(w, screen_size.height)], 2.0, Color::from_rgb(50, 99, 168));
        }
        let background_mesh = graphics::Mesh::from_data(ctx, mb.build());
        // prepare cross texture
        let mb = &mut graphics::MeshBuilder::new();
        mb.line(&[Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0)], 0.05, Color::from_rgb(100, 100, 100));
        mb.line(&[Vec2::new(0.0, 1.0), Vec2::new(1.0, 0.0)], 0.05, Color::from_rgb(100, 100, 100));
        let cross_mesh = graphics::Mesh::from_data(ctx, mb.build());
        let game_state = grid::GameState::new(lvl_desc.into());
        MyGame {
            max_nums_in_rows,
            max_nums_in_cols,
            background_mesh,
            cross_mesh,
            game_state,
            cross_cells: Vec::<graphics::DrawParam>::new(),
            dark_cells: Vec::<graphics::DrawParam>::new()
        }
        // finally, we got to creating GAME STATE
    }

    fn board_cell(&self, x: usize, y: usize) -> graphics::Rect {
        graphics::Rect::new(
            cell_num_to_coord(self.max_nums_in_rows + x),
            cell_num_to_coord(self.max_nums_in_cols + y),
            CELL_SIZE,
            CELL_SIZE
        )
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...

        let pos = _ctx.mouse.position();

        let x_offset = cell_num_to_coord(self.max_nums_in_rows);
        let y_offset = cell_num_to_coord(self.max_nums_in_cols);

        let in_game_pos = mint::Point2::<f32>::from([pos.x - x_offset, pos.y - y_offset]);

        if in_game_pos.x >= 0.0
                && in_game_pos.y >= 0.0
                && in_game_pos.x <= cell_num_to_coord(self.game_state.width())
                && in_game_pos.y <= cell_num_to_coord(self.game_state.height()) 
        {
            let col_number = in_game_pos.x.div_euclid(CELL_SIZE) as usize;
            let row_number = in_game_pos.y.div_euclid(CELL_SIZE) as usize;
            
            if _ctx.mouse.button_just_pressed(MouseButton::Left) {
                if self.game_state.get(col_number, row_number) == grid::CellState::Filled {
                    self.game_state.set(col_number, row_number, grid::CellState::Empty)
                } else {
                    self.game_state.set(col_number, row_number, grid::CellState::Filled)
                }
            }
            if _ctx.mouse.button_just_pressed(MouseButton::Right) {
                if self.game_state.get(col_number, row_number) == grid::CellState::Crossed {
                    self.game_state.set(col_number, row_number, grid::CellState::Empty)
                } else {
                    self.game_state.set(col_number, row_number, grid::CellState::Crossed)
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);


        for (i,row) in self.game_state.lvl_desc().rows.iter().enumerate() {
            for (j,cell) in row.iter().rev().enumerate() {
                let dest_point = Vec2::new(
                    ((self.max_nums_in_rows - j - 1) as f32 + 0.5) * CELL_SIZE, 
                    ((self.max_nums_in_cols + i) as f32 + 0.5) * CELL_SIZE
                );
                let text = graphics::Text::new(format!("{}", cell.0))
                    .set_font(MAIN_FONT)
                    .set_layout(TextLayout::center())
                    .set_scale(CELL_SIZE / 2.0)
                    .clone();
                canvas.draw( &text, graphics::DrawParam::from(dest_point).color(Color::BLACK));
            }
        }

        for (i,col) in self.game_state.lvl_desc().cols.iter().enumerate() {
            for (j,cell) in col.iter().rev().enumerate() {
                let dest_point = Vec2::new(
                    ((self.max_nums_in_rows + i) as f32 + 0.5) * CELL_SIZE, 
                    ((self.max_nums_in_cols - j - 1) as f32 + 0.5) * CELL_SIZE,
                );
                let text = graphics::Text::new(format!("{}", cell.0))
                    .set_font(MAIN_FONT)
                    .set_layout(TextLayout::center())
                    .set_scale(CELL_SIZE / 2.0)
                    .clone();
                canvas.draw( &text, graphics::DrawParam::from(dest_point).color(Color::BLACK));
            }
        }

        for (x,y,cell) in self.game_state.iter() {
            use grid::CellState::*;
            match cell {
                Empty => {},
                Filled => {
                    canvas.draw(
                        &graphics::Quad,
                        graphics::DrawParam::new()
                            .dest_rect(self.board_cell(x, y))
                            .color(Color::BLACK)
                    );
                },
                Crossed => {
                    canvas.draw(
                        &self.cross_mesh,
                        graphics::DrawParam::new()
                            .dest_rect(self.board_cell(x, y))
                    );
                },
            }
        }

        canvas.draw(&self.background_mesh, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}
