pub mod piece;
pub mod player;
pub mod sprites;

use std::{collections::HashMap, fmt::Display};

use macroquad::{
    color::*,
    math::U16Vec2,
    shapes::draw_rectangle,
    text::draw_text,
    texture::{Texture2D, draw_texture},
};
use piece::{Piece, PieceColor, PieceKind};
use player::Players;
use sprites::SpritesMap;

use super::Vec2;

pub struct Board {
    num_cells: U16Vec2,
    cell_size: Vec2,
    selected_piece_pos: Option<GridPosition>,
    players: Players,
    move_sprite: Texture2D,
}
impl Board {
    pub fn new(
        white_sprites: SpritesMap,
        black_sprites: SpritesMap,
        move_sprite: Texture2D,
    ) -> Self {
        Self {
            num_cells: U16Vec2 { x: 8, y: 8 },
            cell_size: Vec2 { x: 128.0, y: 128.0 },
            selected_piece_pos: None,
            players: Players::new(black_sprites, white_sprites),
            move_sprite,
        }
    }

    fn draw_pieces(&self) {
        self.players
            .draw(self.num_cells.y, self.cell_size, self.selected_piece_pos);
    }
    fn draw_gizmos(&self) {
        if let Some(piece) = self
            .selected_piece_pos
            .and_then(|p| self.players.piece_at(p))
        {
            let moves = piece.pseudo_moveset(&self.snapshot());

            for mov in moves {
                let mut snapshot = self.snapshot();
                let opposite = match piece.color {
                    PieceColor::Black => PieceColor::White,
                    PieceColor::White => PieceColor::Black,
                };
                snapshot.move_piece(piece.position, mov);
                let atks = snapshot.attack_map(opposite);

                if atks.contains(&self.players.king_position(piece.color))
                    || (piece.kind == PieceKind::King && atks.contains(&mov))
                {
                    continue;
                }

                let GridPosition { x, y } = mov;
                let y = self.num_cells.y - y - 1;
                draw_texture(
                    &self.move_sprite,
                    x as f32 * self.cell_size.x,
                    y as f32 * self.cell_size.y,
                    WHITE,
                );
            }
        };
    }
    fn draw_attacks(&self) {
        let mut moves = self.snapshot();
        let moves = moves.attack_map(PieceColor::Black);

        for mov in moves {
            let GridPosition { x, y } = mov;
            let y = self.num_cells.y - y - 1;
            draw_rectangle(
                *x as f32 * self.cell_size.x,
                y as f32 * self.cell_size.y,
                self.cell_size.x,
                self.cell_size.y,
                Color {
                    r: 1.0,
                    g: 0.5,
                    b: 0.3,
                    a: 0.8,
                },
            );
        }
    }

    pub fn render(&self) {
        let Vec2 { x: w, y: h } = self.cell_size;
        let U16Vec2 {
            x: rows,
            y: columns,
        } = self.num_cells;
        // As per https://github.com/not-fl3/macroquad/issues/876
        // Have both draw calls separated
        for y in 0..columns {
            for x in 0..rows {
                let inverted_y = (columns - 1) - y;
                let color = if (x + inverted_y) % 2 == 0 {
                    BLACK
                } else {
                    WHITE
                };
                let (mapped_x, mapped_y) = (x as f32 * w, y as f32 * h);
                draw_rectangle(mapped_x, mapped_y, w, h, color);
            }
        }
        for y in 0..columns {
            for x in 0..rows {
                let inverted_y = (columns - 1) - y;
                let color = if (x + inverted_y) % 2 == 0 {
                    WHITE
                } else {
                    BLACK
                };
                let (mapped_x, mapped_y) = (x as f32 * w, y as f32 * h);
                draw_text(
                    &format!("{}", GridPosition { x, y: inverted_y }),
                    mapped_x,
                    mapped_y + self.cell_size.y,
                    32.0,
                    color,
                );
            }
        }
        self.draw_pieces();
        self.draw_gizmos();
        self.draw_attacks();
    }

    pub fn grid_from_world(&self, pos: Vec2) -> Option<GridPosition> {
        let height = self.cell_size.y * self.num_cells.y as f32;
        let width = self.cell_size.y * self.num_cells.y as f32;

        if pos.min_element().signum() == -1.0 || pos.x > width || pos.y > height {
            return None;
        }
        let y = ((height - pos.y) / 128.).floor() as u16;
        let x = (pos.x / 128.0).floor() as u16;
        Some(GridPosition { x, y })
    }

    pub fn selected_piece(&self) -> Option<&Piece> {
        self.selected_piece_pos.map(|p| {
            self.players
                .piece_at(p)
                .expect("Selected piece position doesn't correspont to an existing piece")
        })
    }

    #[inline]
    pub fn selected_piece_pos(&self) -> Option<GridPosition> {
        self.selected_piece_pos
    }
    #[inline]
    pub fn select_piece_at(&mut self, position: GridPosition) {
        self.selected_piece_pos = Some(position);
    }

    pub fn try_move_piece(
        &mut self,
        from: GridPosition,
        to: GridPosition,
    ) -> Result<(), piece::MoveError> {
        if self.selected_piece_pos.is_some_and(|p| p == from) {
            self.selected_piece_pos.take();
        }
        self.players.move_piece(self.snapshot(), from, to)
    }

    fn snapshot(&self) -> BoardState {
        BoardState::new(self)
    }

    pub fn piece_at(&self, p: GridPosition) -> Option<&Piece> {
        self.players.piece_at(p)
    }
}
// TODO: Probably the 'simplest' way to implement an ahead of turn check
// is via something along the lines of a CheckValidator that would hold an
// (immutable) reference to the board + a simulated movement. It would then
// shadow the real position with the new position.
// This would allow to keep the drawing immutable and would very possibly greatly
// simplify the check simulation by being a proper simulation.

pub struct BoardState {
    state: HashMap<GridPosition, Piece>,
    num_cells: U16Vec2,
    attack_map: Option<Vec<GridPosition>>,
}
impl BoardState {
    pub fn new(board: &Board) -> Self {
        let state = board.players.pieces();
        Self {
            state,
            num_cells: board.num_cells,
            attack_map: None,
        }
    }
}
impl BoardState {
    pub fn query_square(&self, pos: GridPosition, flags: SquareQueryFlags) -> bool {
        let GridPosition { x, y } = pos;
        let mut res = !flags.is_empty();
        if flags.contains(SquareQueryFlags::IN_BOUNDS) {
            res &= (0..self.num_cells.x).contains(&x) && (0..self.num_cells.y).contains(&y);
        }
        res
    }
    pub fn attack_map(&mut self, color: PieceColor) -> &[GridPosition] {
        if self.attack_map.is_none() {
            let res = self
                .state
                .iter()
                .filter(|(_, v)| v.color == color)
                .flat_map(|(_, v)| v.pseudo_moveset(self))
                .collect();
            self.attack_map = Some(res);
        }
        self.attack_map.as_ref().unwrap()
    }
    /// Moves piece at `from` to `to`.
    /// Will assume any movement is valid and won't check if it would be a
    /// valid move. If a piece exists at `to` it gets "captured" and is returned.
    /// This only `from` contains a piece.
    pub fn move_piece(&mut self, from: GridPosition, to: GridPosition) -> Option<Piece> {
        let moved = self.state.remove(&from);
        if let Some(mut p) = moved {
            let taken = self.state.remove(&to);
            p.position = to;
            self.state.insert(to, p);
            taken
        } else {
            None
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct GridPosition {
    x: u16,
    y: u16,
}
impl GridPosition {
    pub fn try_add<T: Into<i32>>(&self, direction: (T, T)) -> Option<GridPosition> {
        let (dx, dy) = direction;
        let (dx, dy) = (dx.into(), dy.into());
        let x = (self.x as i32).saturating_add(dx);
        let y = (self.y as i32).saturating_add(dy);
        if (0..u16::MAX as i32).contains(&x) && (0..u16::MAX as i32).contains(&x) {
            Some(Self {
                x: x as u16,
                y: y as u16,
            })
        } else {
            None
        }
    }
}
impl<T> From<(T, T)> for GridPosition
where
    T: Into<u16>,
{
    #[inline]
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0.into(),
            y: value.1.into(),
        }
    }
}
impl Display for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        let mut n = self.x;
        loop {
            let rem = n % 26;
            s.push((b'A' + rem as u8) as char);
            n /= 26;
            if n == 0 {
                break;
            }
            n -= 1;
        }
        s = s.chars().rev().collect();
        write!(f, "{}{}", s, self.y + 1)
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct SquareQueryFlags: u8 {
        const IN_BOUNDS         = 1 << 0;
    }
}
