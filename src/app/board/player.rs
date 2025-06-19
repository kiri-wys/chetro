use std::collections::HashMap;

use macroquad::{
    color::{RED, WHITE},
    math::{Rect, Vec2},
    texture::{DrawTextureParams, draw_texture_ex},
};

use super::{
    BoardState, GridPosition,
    piece::{MoveError, Piece, PieceColor, PieceKind},
    sprites::SpritesMap,
};

pub struct Players {
    black: Player,
    white: Player,
}
impl Players {
    pub fn new(black_sprites: SpritesMap, white_sprites: SpritesMap) -> Self {
        let mut pieces = vec![];
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
            pieces.push(Piece {
                kind,
                color: PieceColor::White,
                position,
            });
        }
        let mut white = Player::new(
            GridPosition { x: 0, y: 0 },
            PieceColor::White,
            white_sprites,
        );
        white.append_pieces(pieces);
        let mut black = Player::new(
            GridPosition { x: 7, y: 7 },
            PieceColor::Black,
            black_sprites,
        );
        black.append_pieces(vec![Piece {
            kind: PieceKind::Rook,
            color: PieceColor::Black,
            position: GridPosition { x: 3, y: 3 },
        }]);
        Self { black, white }
    }
    pub fn king_position(&self, color: PieceColor) -> GridPosition {
        match color {
            PieceColor::Black => self.black.king_position,
            PieceColor::White => self.white.king_position,
        }
    }
    pub fn piece_at(&self, position: GridPosition) -> Option<&Piece> {
        self.black
            .pieces
            .iter()
            .chain(self.white.pieces.iter())
            .find(|p| p.position == position)
    }
    fn piece_at_mut(&mut self, position: GridPosition) -> Option<&mut Piece> {
        self.black
            .pieces
            .iter_mut()
            .chain(self.white.pieces.iter_mut())
            .find(|p| p.position == position)
    }
    pub fn remove_piece_at(&mut self, position: GridPosition) -> Option<Piece> {
        if let Some(idx) = self
            .black
            .pieces
            .iter()
            .position(|p| p.position == position)
        {
            return Some(self.black.pieces.swap_remove(idx));
        };
        if let Some(idx) = self
            .white
            .pieces
            .iter()
            .position(|p| p.position == position)
        {
            return Some(self.white.pieces.swap_remove(idx));
        };
        None
    }

    pub fn pieces(&self) -> HashMap<GridPosition, Piece> {
        self.black
            .pieces
            .iter()
            .chain(self.white.pieces.iter())
            .map(|p| (p.position, p.clone()))
            .collect()
    }

    pub fn move_piece(
        &mut self,
        mut snapshot: BoardState,
        from: GridPosition,
        to: GridPosition,
    ) -> Result<(), MoveError> {
        let bk = self.black.king_position;
        let wk = self.white.king_position;

        if self.piece_at(to).is_some() {
            self.remove_piece_at(to);
        }
        let piece = self.piece_at_mut(from).ok_or(MoveError::InvalidOrigin)?;

        let king_position = match piece.color {
            PieceColor::Black => bk,
            PieceColor::White => wk,
        };
        piece.move_to(&mut snapshot, to, king_position)?;

        if piece.kind == PieceKind::King {
            match piece.color {
                PieceColor::Black => self.black.king_position = to,
                PieceColor::White => self.white.king_position = to,
            }
        }

        Ok(())
    }

    pub fn draw(&self, y_columns: u16, cell_size: Vec2, highlight_piece: Option<GridPosition>) {
        for player in [&self.black, &self.white] {
            player.draw(y_columns, cell_size, highlight_piece);
        }
    }
}

pub struct Player {
    king_position: GridPosition,
    pieces: Vec<Piece>,
    sprites: SpritesMap,
}
impl Player {
    pub fn new(king_position: GridPosition, color: PieceColor, sprites: SpritesMap) -> Self {
        Self {
            king_position,
            pieces: vec![Piece {
                kind: PieceKind::King,
                color,
                position: king_position,
            }],
            sprites,
        }
    }
    pub fn append_pieces(&mut self, pieces: Vec<Piece>) {
        let color = pieces.first().map(|p| p.color);
        if color.is_none() {
            return;
        }
        let color = color.unwrap();
        self.pieces.reserve_exact(pieces.len());
        for mut p in pieces {
            if p.kind == PieceKind::King {
                self.remove_piece_at(self.king_position);
                self.king_position = p.position;
            }
            p.color = color;
            self.pieces.push(p);
        }
    }
    pub fn remove_piece_at(&mut self, position: GridPosition) -> Option<Piece> {
        if let Some(idx) = self.pieces.iter().position(|p| p.position == position) {
            return Some(self.pieces.swap_remove(idx));
        };
        None
    }

    pub fn draw(&self, y_columns: u16, cell_size: Vec2, highlight_piece: Option<GridPosition>) {
        for piece in self.pieces.iter() {
            let GridPosition { x, y } = piece.position;
            let y = y_columns - y - 1;
            let GridPosition { x: ax, y: ay } = piece.kind.atlas_offset(&self.sprites.mappings);
            let modulate = if highlight_piece.is_some_and(|p| p == piece.position) {
                RED
            } else {
                WHITE
            };
            draw_texture_ex(
                &self.sprites.atlas,
                x as f32 * cell_size.x,
                y as f32 * cell_size.y,
                modulate,
                DrawTextureParams {
                    source: Some(Rect {
                        x: ax as f32 * cell_size.x,
                        y: ay as f32 * cell_size.y,
                        w: cell_size.x,
                        h: cell_size.y,
                    }),
                    ..Default::default()
                },
            );
        }
    }
}
