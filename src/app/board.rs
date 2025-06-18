pub mod piece;
pub mod sprites;

use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use macroquad::{
    color::*,
    math::{Rect, U16Vec2},
    shapes::draw_rectangle,
    text::draw_text,
    texture::{DrawTextureParams, draw_texture_ex},
};
use piece::{Piece, PieceColor, PieceKind};
use sprites::SpritesMap;
use tracing::warn;

use super::Vec2;

pub struct Board {
    num_cells: U16Vec2,
    cell_size: Vec2,
    selected_piece_pos: Option<GridPosition>,
    black_pieces: Vec<Piece>,
    white_pieces: Vec<Piece>,
    black_king_pos: GridPosition,
    white_king_pos: GridPosition,
    white_illegal_moves: HashSet<GridPosition>,

    white_sprites: SpritesMap,
    black_sprites: SpritesMap,
}
impl Board {
    pub fn new(white_sprites: SpritesMap, black_sprites: SpritesMap) -> Self {
        let mut white_pieces = vec![];
        let mut king = None;
        for (idx, kind) in [
            PieceKind::Rook,
            PieceKind::Bishop,
            PieceKind::Pawn,
            PieceKind::Knight,
            PieceKind::King,
            PieceKind::Queen,
        ]
        .into_iter()
        .enumerate()
        {
            let position = GridPosition {
                x: idx as u16,
                y: 2,
            };
            if kind == PieceKind::King {
                king = Some(position)
            }
            white_pieces.push(Piece {
                kind,
                color: PieceColor::White,
                position,
            });
        }
        let bk = Piece {
            kind: PieceKind::King,
            color: PieceColor::Black,
            position: GridPosition { x: 7, y: 7 },
        };
        let black_king_pos = bk.position;
        Self {
            num_cells: U16Vec2 { x: 8, y: 8 },
            cell_size: Vec2 { x: 128.0, y: 128.0 },
            selected_piece_pos: None,
            black_pieces: vec![
                Piece {
                    kind: PieceKind::Rook,
                    color: PieceColor::Black,
                    position: GridPosition { x: 3, y: 3 },
                },
                bk,
            ],
            white_pieces,
            black_king_pos,
            white_king_pos: king.unwrap(),
            white_illegal_moves: HashSet::new(),

            white_sprites,
            black_sprites,
        }
    }

    fn draw_pieces(&self) {
        for piece in self.black_pieces.iter() {
            let GridPosition { x, y } = piece.position;
            let y = self.num_cells.y - y - 1;
            let GridPosition { x: ax, y: ay } =
                piece.kind.atlas_offset(&self.black_sprites.mappings);
            let modulate = if self.selected_piece_pos.is_some_and(|p| p == piece.position) {
                RED
            } else {
                WHITE
            };
            draw_texture_ex(
                &self.black_sprites.atlas,
                x as f32 * self.cell_size.x,
                y as f32 * self.cell_size.y,
                modulate,
                DrawTextureParams {
                    source: Some(Rect {
                        x: ax as f32 * self.cell_size.x,
                        y: ay as f32 * self.cell_size.y,
                        w: self.cell_size.x,
                        h: self.cell_size.y,
                    }),
                    ..Default::default()
                },
            );
        }
        for piece in self.white_pieces.iter() {
            let GridPosition { x, y } = piece.position;
            let y = self.num_cells.y - y - 1;
            let GridPosition { x: ax, y: ay } =
                piece.kind.atlas_offset(&self.white_sprites.mappings);
            let modulate = if self.selected_piece_pos.is_some_and(|p| p == piece.position) {
                RED
            } else {
                WHITE
            };
            draw_texture_ex(
                &self.white_sprites.atlas,
                x as f32 * self.cell_size.x,
                y as f32 * self.cell_size.y,
                modulate,
                DrawTextureParams {
                    source: Some(Rect {
                        x: ax as f32 * self.cell_size.x,
                        y: ay as f32 * self.cell_size.y,
                        w: self.cell_size.x,
                        h: self.cell_size.y,
                    }),
                    ..Default::default()
                },
            );
        }
    }
    fn draw_gizmos(&self) {
        if let Some(piece) = self.selected_piece_pos.and_then(|p| self.piece_at(p)) {
            let moves = piece.pseudo_moveset(&self.snapshot());
            let GridPosition { x: ax, y: ay } = self.white_sprites.mappings.move_gizmo;

            for mov in moves {
                let mut snapshot = self.snapshot();
                let opposite = match piece.color {
                    PieceColor::Black => PieceColor::White,
                    PieceColor::White => PieceColor::Black,
                };
                snapshot.move_piece(piece.position, mov);
                let atks = snapshot.attack_map(opposite);

                if atks.contains(&self.white_king_pos)
                    || (piece.kind == PieceKind::King && atks.contains(&mov))
                {
                    continue;
                }

                let GridPosition { x, y } = mov;
                let y = self.num_cells.y - y - 1;
                draw_texture_ex(
                    &self.white_sprites.atlas,
                    x as f32 * self.cell_size.x,
                    y as f32 * self.cell_size.y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect {
                            x: ax as f32 * self.cell_size.x,
                            y: ay as f32 * self.cell_size.y,
                            w: self.cell_size.x,
                            h: self.cell_size.y,
                        }),
                        ..Default::default()
                    },
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

    pub fn remove_piece_at(&mut self, pos: GridPosition) -> Option<Piece> {
        if let Some(idx) = self.black_pieces.iter().position(|p| p.position == pos) {
            return Some(self.black_pieces.swap_remove(idx));
        };
        if let Some(idx) = self.white_pieces.iter().position(|p| p.position == pos) {
            return Some(self.white_pieces.swap_remove(idx));
        }
        None
    }
    pub fn piece_at(&self, pos: GridPosition) -> Option<&Piece> {
        self.black_pieces
            .iter()
            .chain(self.white_pieces.iter())
            .find(|p| p.position == pos)
    }
    pub fn piece_at_mut(&mut self, pos: GridPosition) -> Option<&mut Piece> {
        self.black_pieces
            .iter_mut()
            .chain(self.white_pieces.iter_mut())
            .find(|p| p.position == pos)
    }

    pub fn selected_piece(&self) -> Option<&Piece> {
        self.selected_piece_pos.map(|p| {
            self.piece_at(p)
                .expect("Selected piece position doesn't correspont to an existing piece")
        })
    }

    #[inline]
    pub fn selected_piece_pos(&self) -> Option<GridPosition> {
        self.selected_piece_pos
    }
    pub fn selected_piece_mut(&mut self) -> Option<&mut Piece> {
        self.selected_piece_pos.map(|p| {
            self.piece_at_mut(p)
                .expect("Selected piece position doesn't correspont to an existing piece")
        })
    }

    #[inline]
    pub fn select_piece(&mut self, piece: &Piece) {
        self.select_piece_at(piece.position);
    }
    #[inline]
    pub fn select_piece_at(&mut self, position: GridPosition) {
        self.selected_piece_pos = Some(position);
    }

    fn test_movement(&self, from: GridPosition, to: GridPosition) -> bool {
        match self.piece_at(from) {
            Some(piece) => {
                let moves = piece.pseudo_moveset(&self.snapshot());
                moves.contains(&to)
            }
            None => false,
        }
    }
    pub fn try_move_piece(&mut self, from: GridPosition, to: GridPosition) {
        if self.selected_piece_pos.is_some_and(|p| p == from) {
            self.selected_piece_pos.take();
        }
        if self.test_movement(from, to) {
            let mut next = self.snapshot();
            next.move_piece(from, to);
            let king = self.white_king_pos;
            let piece = self.piece_at_mut(from).unwrap();
            let opposite = match piece.color {
                PieceColor::Black => PieceColor::White,
                PieceColor::White => PieceColor::Black,
            };
            let atks = next.attack_map(opposite);
            if atks.contains(&king) || (piece.kind == PieceKind::King && atks.contains(&to)) {
                warn!("{from}->{to} would lead to check");
                return;
            }
            piece.position = to;
            if piece.kind == PieceKind::King {
                self.white_king_pos = to;
            }
        }
    }

    /// Tries to move piece at `from` to `to`.
    /// Returns the captured piece, won't move if there's no piece at `to` so
    /// `Some(piece)` is guaranteed to have moved the piece while `None` is guaranteed
    /// to have left the capturing piece at `from`
    pub fn try_capture_piece(&mut self, from: GridPosition, to: GridPosition) -> Option<Piece> {
        if self.selected_piece_pos.is_some_and(|p| p == from) {
            self.selected_piece_pos.take();
        }
        if self.test_movement(from, to) {
            match self.remove_piece_at(to) {
                Some(s) => {
                    self.piece_at_mut(from).unwrap().position = to;
                    Some(s)
                }
                None => None,
            }
        } else {
            None
        }
    }

    fn snapshot(&self) -> BoardState {
        BoardState::new(self)
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
        let state = board
            .black_pieces
            .iter()
            .chain(board.white_pieces.iter())
            .map(|p| (p.position, p.clone()))
            .collect();
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
