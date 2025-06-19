use macroquad::texture::Texture2D;
use serde::Deserialize;

use super::GridPosition;

pub struct SpritesMap {
    pub atlas: Texture2D,
    pub mappings: PieceMappings,
}
#[derive(Deserialize, Clone, Copy)]
#[serde(deny_unknown_fields)]
pub struct PieceMappings {
    pub pawn: GridPosition,
    pub rook: GridPosition,
    pub bishop: GridPosition,
    pub knight: GridPosition,
    pub king: GridPosition,
    pub queen: GridPosition,
}
