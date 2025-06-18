use super::{BoardState, GridPosition, SquareQueryFlags, sprites::PieceMappings};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceColor {
    Black,
    White,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PieceKind {
    Pawn,
    Rook,
    Bishop,
    Knight,
    Queen,
    King,
}
#[derive(Clone, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
    pub position: GridPosition,
}
impl Piece {
    /// Generates pseduolegal moves.
    /// A pseudolegal move is defined as a reachable possible move the piece can take.
    /// The difference with legal moves is that pseudolegal moves dont take into account
    /// some of the game's state and rules such as if the move would lead to a check(mate).
    pub fn pseudo_moveset(&self, board: &BoardState) -> Vec<GridPosition> {
        let front = match self.color {
            PieceColor::Black => (0, -1),
            PieceColor::White => (0, 1),
        };
        let mut res = vec![];
        let mut helper = MoveConstructor {
            start: self.position,
            piece_color: self.color,
            front,
            square_flags: SquareQueryFlags::IN_BOUNDS,
            board,
            result: &mut res,
        };
        match self.kind {
            PieceKind::Pawn => {
                helper.build_straight_line(Some(1), InclusionPolicy::EMPTY);
                helper.front.0 = 1;
                helper.build_straight_line(Some(1), InclusionPolicy::DIFFERENT);
                helper.front.0 = -1;
                helper.build_straight_line(Some(1), InclusionPolicy::DIFFERENT);
            }
            PieceKind::Rook => {
                helper.build_cross(None, InclusionPolicy::EMPTY | InclusionPolicy::DIFFERENT);
            }
            PieceKind::Bishop => {
                helper.build_diag_cross(None, InclusionPolicy::EMPTY | InclusionPolicy::DIFFERENT);
            }
            PieceKind::Knight => {
                helper.build_square_corners((2, 1));
                helper.build_square_corners((1, 2));
            }
            PieceKind::Queen => {
                helper.build_cross(None, InclusionPolicy::EMPTY | InclusionPolicy::DIFFERENT);
                helper.build_diag_cross(None, InclusionPolicy::EMPTY | InclusionPolicy::DIFFERENT);
            }
            PieceKind::King => {
                helper.build_cross(Some(1), InclusionPolicy::EMPTY | InclusionPolicy::DIFFERENT);
                helper
                    .build_diag_cross(Some(1), InclusionPolicy::EMPTY | InclusionPolicy::DIFFERENT);
            }
        };
        res
    }
}

struct MoveConstructor<'a> {
    start: GridPosition,
    piece_color: PieceColor,
    front: (i8, i8),
    square_flags: SquareQueryFlags,
    board: &'a BoardState,
    result: &'a mut Vec<GridPosition>,
}
impl MoveConstructor<'_> {
    fn build_cross(&mut self, max: Option<u16>, include: InclusionPolicy) {
        let old = self.front;
        self.front = (1, 0);
        self.build_straight_line(max, include);
        self.front = (0, 1);
        self.build_straight_line(max, include);
        self.front = (0, -1);
        self.build_straight_line(max, include);
        self.front = (-1, 0);
        self.build_straight_line(max, include);
        self.front = old;
    }
    fn build_diag_cross(&mut self, max: Option<u16>, include: InclusionPolicy) {
        let old = self.front;
        for dy in [-1, 1] {
            for dx in [-1, 1] {
                self.front = (dx, dy);
                self.build_straight_line(max, include);
            }
        }
        self.front = old;
    }
    fn build_straight_line(&mut self, max: Option<u16>, include: InclusionPolicy) {
        let max = max.unwrap_or(512);
        let squares = (1..=max).map_while(|i| {
            self.start.try_add((
                self.front.0 as i32 * i as i32,
                self.front.1 as i32 * i as i32,
            ))
        });

        for candidate in squares {
            if self.board.query_square(candidate, self.square_flags) {
                match self.board.state.get(&candidate) {
                    Some(p) => {
                        let same = p.color == self.piece_color;
                        if same && include.contains(InclusionPolicy::SAME) {
                            self.result.push(candidate);
                        }
                        if !same && include.contains(InclusionPolicy::DIFFERENT) {
                            self.result.push(candidate);
                        }
                        break;
                    }
                    None => {
                        if include.contains(InclusionPolicy::EMPTY) {
                            self.result.push(candidate);
                        }
                    }
                }
            }
        }
    }

    fn build_square_corners(&mut self, delta: (i8, i8)) {
        for m1 in [-1, 1] {
            for m2 in [-1, 1] {
                let delta = (delta.0 * m1, delta.1 * m2);
                if let Some(candidate) = self.start.try_add(delta) {
                    if self.board.query_square(candidate, self.square_flags)
                        && self
                            .board
                            .state
                            .get(&candidate)
                            .is_none_or(|p| p.color != self.piece_color)
                    {
                        self.result.push(candidate);
                    }
                }
            }
        }
    }
}
bitflags::bitflags! {
    #[derive(Clone, Copy)]
    pub struct InclusionPolicy: u8 {
        const EMPTY         = 1 << 1;
        const SAME          = 1 << 2;
        const DIFFERENT     = 1 << 3;

    }
}

impl PieceKind {
    pub fn atlas_offset(&self, map: &PieceMappings) -> GridPosition {
        let PieceMappings {
            pawn,
            rook,
            bishop,
            knight,
            king,
            queen,
            move_gizmo: _,
        } = *map;
        match self {
            PieceKind::Pawn => pawn,
            PieceKind::Rook => rook,
            PieceKind::Bishop => bishop,
            PieceKind::Knight => knight,
            PieceKind::Queen => queen,
            PieceKind::King => king,
        }
    }
}
