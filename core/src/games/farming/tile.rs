use crate::games::farming::layer::FarmLayer;
use crate::games::farming::soil::SoilFlags;
use crate::rendering::o2d::prelude::*;

#[derive(Debug, Default)]
pub struct FarmTile {
    pub soil: SoilFlags,
}

impl FarmTile {
    pub fn render_objects(&self, x: u8, y: u8) -> Vec<Object2D> {
        let mut objects = Vec::new();
        objects.extend(self.render_soil(x, y));
        objects
    }

    pub fn render_soil(&self, x: u8, y: u8) -> Vec<Object2D> {
        let position = PositionO2D::from_tile_xy_layer(x, y, FarmLayer::Ground);

        if !self.soil.is_tilled() {
            let visual = VisualO2D::Sprite(SpriteId::UntilledSoil);
            return vec![Object2D { visual, position }];
        };

        let dry = Object2D {
            position,
            visual: VisualO2D::Tile(TilesetId::Farmland),
        };

        if !self.soil.is_watered() {
            return vec![dry];
        };

        let wet = Object2D {
            position: position.with_layer(FarmLayer::OnGround),
            visual: VisualO2D::Tile(TilesetId::FarmlandWet),
        };

        vec![wet, dry]
    }
}
