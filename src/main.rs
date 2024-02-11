#![allow(unused)]

mod game_state;
mod description;
mod clickable_zone;
mod grid;

mod meshes;
mod transaction;
mod strategy;
mod ai_player;
mod line;

use std::cell::Cell;
use std::path;

use clap::Parser;
use description::LevelDescriptionTemplate;
use ggez::glam::Vec2;
use ggez::winit::dpi::{Size, PhysicalSize, LogicalSize};
use ggez::{Context, ContextBuilder, GameResult, GameError, mint};
use ggez::graphics::{self, Color, Text, TextFragment, PxScale, TextLayout, Rect, Canvas};
use ggez::event::{self, EventHandler, MouseButton};
use ggez::mint::{Point2, Vector2};
use serde::{Serialize, Deserialize};

use crate::ai_player::AiPlayer;
use crate::game_state::{CellState, GameState};
use crate::clickable_zone::ClickableZone;
use crate::game_state::CellState::{Crossed, Empty, Filled};
use crate::transaction::TransactionBuilder;
use crate::strategy::simple::SimpleStrategy;


const CELL_SIZE: f32 = 100.0;
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

struct GameClickState {
    state: CellState,
    col: usize,
    row: usize,
    is_horizontal: bool,
    is_vertical: bool
}

struct MyGame {
    max_nums_in_rows: usize,
    max_nums_in_cols: usize,
    background_mesh: graphics::Mesh,
    cross_mesh: graphics::Mesh,
    transparent_cross_mesh: graphics::Mesh,
    game_state: GameState,
    game_zone: ClickableZone,
    undo_zone: ClickableZone,
    click_state: Option<GameClickState>,

    play_once_zone: ClickableZone,
    play_many_zone: ClickableZone,
    pause_zone: ClickableZone,
    ai_player: AiPlayer,

    done_mesh: graphics::Mesh,
    stopped_mesh: graphics::Mesh,
    in_progress_mesh: graphics::Mesh
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

        let screen_size_with_buttons_line = LogicalSize::new(
            cell_num_to_coord(max_nums_in_rows + lvl_desc.cols.len()) + 2.0,
            cell_num_to_coord(max_nums_in_cols + lvl_desc.rows.len() + 1) + 2.0,
        );

        ctx.gfx.window().set_inner_size(screen_size_with_buttons_line);
        // Load fonts
        ctx.gfx.add_font(MAIN_FONT, graphics::FontData::from_path(ctx, "/LiberationMono-Regular.ttf").unwrap());
        // Prepare background
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
        let cross_mesh = meshes::cross(0.05, Color::from_rgb(100, 100, 100), &ctx);
        let transparent_cross_mesh = meshes::cross(0.05, Color::from_rgba(100, 100, 100, 100), &ctx);
        let game_state = game_state::GameState::new(lvl_desc.into());

        let x_offset = cell_num_to_coord(max_nums_in_rows);
        let y_offset = cell_num_to_coord(max_nums_in_cols);

        let game_zone = ClickableZone::new(
            Point2::<f32>::from([x_offset, y_offset]),
            Vector2::<f32>::from([cell_num_to_coord(game_state.width()), cell_num_to_coord(game_state.height())])
        );



        let default_button_color = Color::from_rgb(0, 0, 0);
        let default_button_hover_color = Color::from_rgb(127, 127, 127);
        let width = 0.02;

        let mut undo_zone = ClickableZone::new(
            Point2::<f32>::from([
                cell_num_to_coord(0),
                cell_num_to_coord(max_nums_in_cols + game_state.height())
            ]),
            Vector2::<f32>::from([CELL_SIZE, CELL_SIZE]),
        );
        undo_zone.set_mesh_for_draw(meshes::left_arrow(width, default_button_color, &ctx));
        undo_zone.set_mesh_for_draw_at_hover(meshes::left_arrow(width, default_button_hover_color, &ctx));

        let mut play_once_zone = ClickableZone::new(
            Point2::<f32>::from([
                cell_num_to_coord(2),
                cell_num_to_coord(max_nums_in_cols + game_state.height())
            ]),
            Vector2::<f32>::from([CELL_SIZE, CELL_SIZE]),
        );
        play_once_zone.set_mesh_for_draw(meshes::play_once(width, default_button_color, &ctx));
        play_once_zone.set_mesh_for_draw_at_hover(meshes::play_once(width, default_button_hover_color, &ctx));

        let mut play_many_zone = ClickableZone::new(
            Point2::<f32>::from([
                cell_num_to_coord(3),
                cell_num_to_coord(max_nums_in_cols + game_state.height())
            ]),
            Vector2::<f32>::from([CELL_SIZE, CELL_SIZE]),
        );
        play_many_zone.set_mesh_for_draw(meshes::play_many(width, default_button_color, &ctx));
        play_many_zone.set_mesh_for_draw_at_hover(meshes::play_many(width, default_button_hover_color, &ctx));

        let mut pause_zone = ClickableZone::new(
            Point2::<f32>::from([
                cell_num_to_coord(4),
                cell_num_to_coord(max_nums_in_cols + game_state.height())
            ]),
            Vector2::<f32>::from([CELL_SIZE, CELL_SIZE]),
        );
        pause_zone.set_mesh_for_draw(meshes::pause(width, default_button_color, &ctx));
        pause_zone.set_mesh_for_draw_at_hover(meshes::pause(width, default_button_hover_color, &ctx));

        let mut ai_player = AiPlayer::new();
        ai_player.engines.push(Box::new( SimpleStrategy {}));

        let done_mesh = meshes::done(0.02, Color::from_rgb(0, 200, 83), &ctx);
        let stopped_mesh = meshes::stopped(0.02, Color::from_rgb(255, 23, 68), &ctx);
        let in_progress_mesh = meshes::in_progress(0.02, Color::from_rgb(254, 223, 88), &ctx);

        MyGame {
            max_nums_in_rows,
            max_nums_in_cols,
            background_mesh,
            cross_mesh,
            game_state,
            game_zone,
            undo_zone,
            click_state: None,
            play_once_zone,
            play_many_zone,
            pause_zone,
            ai_player,
            transparent_cross_mesh,
            stopped_mesh,
            done_mesh,
            in_progress_mesh
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

    fn row_description_cell(&self, x: usize, y: usize) -> graphics::Rect {
        graphics::Rect::new(
            cell_num_to_coord(x),
            cell_num_to_coord(self.max_nums_in_cols + y),
            CELL_SIZE,
            CELL_SIZE
        )
    }

    fn col_description_cell(&self, x: usize, y: usize) -> graphics::Rect {
        graphics::Rect::new(
            cell_num_to_coord(self.max_nums_in_rows + x),
            cell_num_to_coord(y),
            CELL_SIZE,
            CELL_SIZE
        )
    }

    fn button_cell(&self, x: usize) -> graphics::Rect {
        graphics::Rect::new(
            cell_num_to_coord(x),
            cell_num_to_coord(self.max_nums_in_cols + self.game_state.height()),
            CELL_SIZE,
            CELL_SIZE
        )
    }
}



impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...

        let pos = _ctx.mouse.position();

        {
            let mut builder = TransactionBuilder::new(self.game_state.grid());
            self.ai_player.try_perform_turn(self.game_state.lvl_desc(), &mut builder);
            let transaction = builder.to_transaction(self.game_state.grid());
            self.game_state.apply_transaction(&transaction);
        }

        if self.undo_zone.in_clickable_zone(pos) {
            if _ctx.mouse.button_just_pressed(MouseButton::Left) {
                self.game_state.undo();
                self.ai_player.restart_clock();
            }
        }

        if self.play_once_zone.in_clickable_zone(pos) {
            if _ctx.mouse.button_just_pressed(MouseButton::Left) {
                let mut builder = TransactionBuilder::new(self.game_state.grid());

                self.ai_player.play_single_turn_emergency(self.game_state.lvl_desc(), &mut builder);

                let transaction = builder.to_transaction(self.game_state.grid());

                self.game_state.apply_transaction(&transaction);

                self.ai_player.restart_clock();
            }
        }

        if self.play_many_zone.in_clickable_zone(pos) {
            if _ctx.mouse.button_just_pressed(MouseButton::Left) {
                self.ai_player.start_play();
            }
        }

        if self.pause_zone.in_clickable_zone(pos) {
            if _ctx.mouse.button_just_pressed(MouseButton::Left) {
                self.ai_player.pause_play();
            }
        }

        if self.game_zone.in_clickable_zone(pos) {

            let x_offset = cell_num_to_coord(self.max_nums_in_rows);
            let y_offset = cell_num_to_coord(self.max_nums_in_cols);

            let in_game_pos = mint::Point2::<f32>::from([pos.x - x_offset, pos.y - y_offset]);

            let mut col_number = in_game_pos.x.div_euclid(CELL_SIZE) as usize;
            let mut row_number = in_game_pos.y.div_euclid(CELL_SIZE) as usize;

            if row_number < self.game_state.height() && col_number < self.game_state.width() {
                if let None = self.click_state {
                    let new_state = if _ctx.mouse.button_pressed(MouseButton::Left) {
                        if self.game_state.get(col_number, row_number) == Filled { Some(Empty) } else { Some(Filled) }
                    } else if _ctx.mouse.button_pressed(MouseButton::Right) {
                        if self.game_state.get(col_number, row_number) == Crossed { Some(Empty) } else { Some(Crossed) }
                    } else {
                        None
                    };

                    if let Some(state) = new_state {
                        self.click_state = Some(GameClickState {
                            is_horizontal: true,
                            is_vertical: true,
                            col: col_number,
                            row: row_number,
                            state: state
                        })
                    }
                } else if !_ctx.mouse.button_pressed(MouseButton::Left) && !_ctx.mouse.button_pressed(MouseButton::Right) {
                    self.click_state = None;
                }

                if let Some(click_state) = &self.click_state {
                    if click_state.is_horizontal && click_state.is_vertical {
                        //Diagonal move not allowed
                        if (click_state.row != row_number && click_state.col != col_number) {
                            self.click_state = None;
                        } else if (click_state.row != row_number) {
                            self.click_state = Some(GameClickState {
                                is_horizontal: false,
                                is_vertical: true,
                                col: click_state.col,
                                row: click_state.row,
                                state: click_state.state
                            });
                        } else if (click_state.col != col_number) {
                            self.click_state = Some(GameClickState {
                                is_horizontal: true,
                                is_vertical: false,
                                col: click_state.col,
                                row: click_state.row,
                                state: click_state.state
                            });
                        }
                    } else if click_state.is_horizontal {
                        row_number = click_state.row;
                    } else if click_state.is_vertical {
                        col_number = click_state.col;
                    } else {
                        panic!("Impossible state");
                    }
                }

                if let Some(click_state) = &self.click_state {
                    if self.game_state.get(col_number, row_number) != click_state.state {
                        self.game_state.set(col_number, row_number, click_state.state);
                        self.ai_player.restart_clock();
                    }
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);


        for (i,row) in self.game_state.lvl_desc().rows.iter().enumerate() {
            for (j,cell) in row.parts.iter().rev().enumerate() {
                let dest_point = Vec2::new(
                    ((self.max_nums_in_rows - j - 1) as f32 + 0.5) * CELL_SIZE, 
                    ((self.max_nums_in_cols + i) as f32 + 0.5) * CELL_SIZE
                );
                let text = graphics::Text::new(format!("{}", cell.elements_count))
                    .set_font(MAIN_FONT)
                    .set_layout(TextLayout::center())
                    .set_scale(CELL_SIZE / 2.0)
                    .clone();
                canvas.draw( &text, graphics::DrawParam::from(dest_point).color(Color::BLACK));
            }
        }

        for (i,col) in self.game_state.lvl_desc().cols.iter().enumerate() {
            for (j,cell) in col.parts.iter().rev().enumerate() {
                let dest_point = Vec2::new(
                    ((self.max_nums_in_rows + i) as f32 + 0.5) * CELL_SIZE, 
                    ((self.max_nums_in_cols - j - 1) as f32 + 0.5) * CELL_SIZE,
                );
                let text = graphics::Text::new(format!("{}", cell.elements_count))
                    .set_font(MAIN_FONT)
                    .set_layout(TextLayout::center())
                    .set_scale(CELL_SIZE / 2.0)
                    .clone();
                canvas.draw( &text, graphics::DrawParam::from(dest_point).color(Color::BLACK));
            }
        }

        for (x,y,cell) in self.game_state.grid_to_iter() {
            use game_state::CellState::*;
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

        for (row_num, row_description) in self.game_state.lvl_desc().rows.iter().enumerate() {
            for (row_part_num, row_description_part) in row_description.parts.iter().enumerate() {
                if row_description_part.is_completed {
                    canvas.draw(
                        &self.transparent_cross_mesh,
                        graphics::DrawParam::new()
                            .dest_rect(self.row_description_cell(self.max_nums_in_rows - row_part_num - 1, row_num))
                    )
                }
            }
        }

        for (col_num, col_description) in self.game_state.lvl_desc().cols.iter().enumerate() {
            for (col_part_num, col_description_part) in col_description.parts.iter().enumerate() {
                if col_description_part.is_completed {
                    canvas.draw(
                        &self.transparent_cross_mesh,
                        graphics::DrawParam::new()
                            .dest_rect(self.col_description_cell(col_num, self.max_nums_in_cols - col_part_num - 1))
                    )
                }
            }
        }

        self.undo_zone.draw(ctx.mouse.position(), &mut canvas);
        self.play_once_zone.draw(ctx.mouse.position(), &mut canvas);
        self.play_many_zone.draw(ctx.mouse.position(), &mut canvas);
        self.pause_zone.draw(ctx.mouse.position(), &mut canvas);

        let ai_mesh = if (self.ai_player.is_active()) {
            if self.game_state.lvl_desc().is_done() {
                &self.done_mesh
            }
            else {
                &self.in_progress_mesh
            }
        }
        else {
            &self.stopped_mesh
        };

        canvas.draw(
            ai_mesh,
            graphics::DrawParam::new()
                .dest_rect(self.button_cell(6))
        );

        canvas.draw(&self.background_mesh, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
}
