use crate::rendering::o2d::sprite::SpriteId;
use crate::rendering::o2d::tileset::TilesetId;

#[derive(Debug, Clone, Copy)]
pub enum VisualO2D {
    Sprite(SpriteId),
    Tile(TilesetId),
}
