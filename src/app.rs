pub mod board;

use std::fmt::Display;

use board::{Board, GridPosition, piece::PieceColor, sprites::SpritesMap};
use macroquad::{
    color::*,
    input::{MouseButton, is_mouse_button_pressed},
    math::Vec2,
    shapes::draw_circle,
    texture::Texture2D,
};
use tracing::{info, warn};

pub struct Game {
    board: Board,
    player_color: PieceColor,

    pub ctx: Context,
}

impl Game {
    pub fn new(
        white_sprites: SpritesMap,
        black_sprites: SpritesMap,
        move_sprite: Texture2D,
    ) -> Self {
        Self {
            board: Board::new(white_sprites, black_sprites, move_sprite),
            player_color: PieceColor::White,
            ctx: Default::default(),
        }
    }

    pub fn draw(&self) {
        self.board.render();
        draw_circle(
            self.ctx.mouse_position.x,
            self.ctx.mouse_position.y,
            15.0,
            RED,
        );
    }
    pub fn update(&mut self) {
        if let Some(p) = self.board.grid_from_world(self.ctx.mouse_position) {
            if is_mouse_button_pressed(MouseButton::Left) {
                let action = match (self.board.selected_piece_pos(), self.board.piece_at(p)) {
                    (None, None) => ClickAction::Nothing,
                    (None, Some(p)) => ClickAction::SelectNew(p.position),
                    (Some(selected), None) => {
                        let color = self.board.selected_piece().unwrap().color;
                        if color == self.player_color {
                            ClickAction::TryMove {
                                from: selected,
                                to: p,
                            }
                        } else {
                            ClickAction::Nothing
                        }
                    }
                    (Some(from), Some(to)) => {
                        let color = self.board.selected_piece().unwrap().color;
                        if from == to.position {
                            ClickAction::Nothing
                        } else if color == self.player_color {
                            if color == to.color {
                                ClickAction::ChangeSelection {
                                    from,
                                    to: to.position,
                                }
                            } else {
                                ClickAction::TryCapture {
                                    from,
                                    to: to.position,
                                }
                            }
                        } else {
                            ClickAction::SelectNew(to.position)
                        }
                    }
                };
                info!("{}", action);

                // TODO: Would it be worth it to have Rc<RefCell> instead of loose references?
                // Consider that the runtime costs of handling the references could be close
                // to RefCell's assertions.
                match action {
                    ClickAction::SelectNew(piece) => self.board.select_piece_at(piece),
                    ClickAction::TryMove { from, to } | ClickAction::TryCapture { from, to } => {
                        if let Err(err) = self.board.try_move_piece(from, to) {
                            warn!("Invalid move: {:?}", err);
                        };
                    }
                    ClickAction::ChangeSelection { from: _, to } => self.board.select_piece_at(to),
                    ClickAction::Nothing => (),
                }
            }
        }
    }
}
#[derive(Debug)]
pub enum ClickAction {
    SelectNew(GridPosition),
    TryMove {
        from: GridPosition,
        to: GridPosition,
    },
    ChangeSelection {
        from: GridPosition,
        to: GridPosition,
    },
    TryCapture {
        from: GridPosition,
        to: GridPosition,
    },
    Nothing,
}
impl Display for ClickAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClickAction::SelectNew(grid_position) => {
                write!(f, "Select {grid_position}")
            }
            ClickAction::TryMove { from, to } => {
                write!(f, "Attempt {from}->{to}")
            }
            ClickAction::ChangeSelection { from, to } => {
                write!(f, "Select {from}->{to}")
            }
            ClickAction::TryCapture { from, to } => {
                write!(f, "Capture {from}->{to}")
            }
            ClickAction::Nothing => write!(f, "Nothing"),
        }
    }
}

#[derive(Default)]
pub struct Context {
    pub mouse_position: Vec2,
}
