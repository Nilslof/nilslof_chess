use std::{env, path};

use nilslof_chess_engine;
use nilslof_chess_engine::{PieceType, Game, Colour, MoveType};
use ggez::{Context, event, GameResult, graphics};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::conf::NumSamples;
use ggez::event::MouseButton;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::nalgebra::Point2;


const size: f32 = 400.0;

struct MainState {
    pos_x: f32,
    game: Game,
    pressed_pos: Option<GridPosition>

}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut g = Game::new();
        let s = MainState {
            pos_x: 0.0,
            game: g,
            pressed_pos: Option::None,
        };

        Ok(s)

    }

    fn draw_board(&mut self, ctx: &mut Context) -> GameResult{
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for y in 0..8 {
            for x in 0..8 {
                let mut color = graphics::WHITE;

                if (x + y) % 2 == 0 {
                    color = graphics::Color::from_rgb(200, 100, 100);
                }

                let rectangle = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        (x as f32) * size / 8.0,
                        (y as f32) * size / 8.0,
                        size / 8.0,
                        size / 8.0,
                    ),
                    color,
                )?;

                graphics::draw(ctx, &rectangle, (na::Point2::new(self.pos_x, 0.0), ))?;

                let mut p = ggez::graphics::Image::new(ctx, "/white_king.png")?;
                if self.game.board[x][y].is_some() {
                    match self.game.board[x][y].unwrap().get_colour() {
                        Colour::White => {
                            match self.game.board[x][y].unwrap().piece_type {
                                PieceType::King => {
                                    p = ggez::graphics::Image::new(ctx, "/white_king.png")?;
                                }
                                PieceType::Queen => {
                                    p = ggez::graphics::Image::new(ctx, "/white_queen.png")?;
                                }
                                PieceType::Bishop => {
                                    p = ggez::graphics::Image::new(ctx, "/white_bishop.png")?;
                                }
                                PieceType::Knight => {
                                    p = ggez::graphics::Image::new(ctx, "/white_knight.png")?;
                                }
                                PieceType::Rook => {
                                    p = ggez::graphics::Image::new(ctx, "/white_rook.png")?;
                                }
                                PieceType::Pawn => {
                                    p = ggez::graphics::Image::new(ctx, "/white_pawn.png")?;
                                }
                            }
                        }
                        Colour::Black => {
                            match self.game.board[x][y].unwrap().piece_type {
                                PieceType::King => {
                                    p = ggez::graphics::Image::new(ctx, "/black_king.png")?;
                                }
                                PieceType::Queen => {
                                    p = ggez::graphics::Image::new(ctx, "/black_queen.png")?;
                                }
                                PieceType::Bishop => {
                                    p = ggez::graphics::Image::new(ctx, "/black_bishop.png")?;
                                }
                                PieceType::Knight => {
                                    p = ggez::graphics::Image::new(ctx, "/black_knight.png")?;
                                }
                                PieceType::Rook => {
                                    p = ggez::graphics::Image::new(ctx, "/black_rook.png")?;
                                }
                                PieceType::Pawn => {
                                    p = ggez::graphics::Image::new(ctx, "/black_pawn.png")?;
                                }
                            }
                        }
                    }
                    graphics::draw(ctx, &p, DrawParam::default()
                        .dest(GridPosition::new(x, y))
                        .scale(ggez::mint::Vector2 { x: size / 360.0, y: size / 360.0 }));
                }
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn draw_possible_moves(&mut self, ctx: &mut Context) -> GameResult {
        let pos = self.pressed_pos.unwrap();
        let possible_moves = self.game.get_allowed_moves(pos.x as usize, pos.y as usize);

        let mut color = graphics::BLACK;

        for possible_move in possible_moves {

            let rectangle = graphics::Mesh::new_rectangle(
                ctx,
                graphics::DrawMode::fill(),
                graphics::Rect::new(
                    ((possible_move.1 as f32) * size / 8.0) + (size/24.0),
                    ((possible_move.2 as f32) * size / 8.0) + (size/24.0),
                    size / 32.0,
                    size / 32.0,
                ),
                color,
            )?;

            graphics::draw(ctx, &rectangle, (na::Point2::new(self.pos_x, 0.0), ))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.draw_board(ctx);
        if self.pressed_pos.is_some(){
            self.draw_possible_moves(ctx);
        }
        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32,) {
        let pos = GridPosition::new_from_pixel(_x, _y);

        if self.pressed_pos.is_none(){
            self.pressed_pos = Option::Some(pos);
        }
        else{
            let possible_moves = self.game.get_allowed_moves(self.pressed_pos.unwrap().x as usize, self.pressed_pos.unwrap().y as usize);

            for pos_move in possible_moves{
                if pos_move.1 == pos.x as usize && pos_move.2 == pos.y as usize{
                    self.game.move_piece(
                        pos_move.0,
                        self.pressed_pos.unwrap().x as usize,
                        self.pressed_pos.unwrap().y as usize,
                        pos.x as usize,
                        pos.y as usize
                    );

                }
            }
            self.pressed_pos = Option::None;
        }



    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32,) {
        let pos = GridPosition::new_from_pixel(_x, _y);
        //println!("{:?} Button released at {} {}", _button, pos.x, pos.y );
    }




}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    fn new(x: usize, y: usize) -> GridPosition {
        GridPosition {
            x: x as i32,
            y: y as i32,
        }
    }

    fn new_from_pixel(_x: f32, _y: f32) -> GridPosition {
        GridPosition {
            x: (_x as i32*8)/size as i32,
            y: (_y as i32*8)/size as i32,
        }
    }
}

impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x * size as i32/8,
            pos.y * size as i32/8,
             size as i32/8 as i32,
             size as i32/8 as i32,
        )
    }

}

impl From<GridPosition> for ggez::mint::Point2<f32> {
    fn from(pos: GridPosition) -> Self {
        ggez::mint::Point2 { x: (pos.x * size as i32/8) as f32, y: (pos.y * size as i32/8) as f32 }
    }
}


pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("Basta-Chack-GUI/ChessGUI/resources")
    };

    let mut window = ggez::ContextBuilder::new("ChessGUI", "ggez")
        .window_setup(ggez::conf::WindowSetup::default().title("Nilslof Chess"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(size, size))
        .add_resource_path(resource_dir);


    let (ctx, event_loop) = &mut window.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
