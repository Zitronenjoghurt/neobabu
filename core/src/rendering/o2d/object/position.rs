use super::layer::LayerO2D;

#[derive(Debug, Default, Clone, Copy)]
pub struct PositionO2D {
    pub tile_x: u8,
    pub tile_y: u8,
    pub x_offset: u8,
    pub y_offset: u8,
    pub z_index: u8,
}

impl PositionO2D {
    pub fn from_tile_xy(tile_x: u8, tile_y: u8) -> Self {
        Self {
            tile_x,
            tile_y,
            ..Default::default()
        }
    }

    pub fn from_tile_xy_layer(tile_x: u8, tile_y: u8, layer: impl LayerO2D) -> Self {
        Self {
            tile_x,
            tile_y,
            z_index: layer.get_z_index(),
            ..Default::default()
        }
    }

    pub fn pixel_position(&self, tile_size: u8) -> (u32, u32) {
        let x = self.tile_x as u32 * tile_size as u32 + self.x_offset as u32;
        let y = self.tile_y as u32 * tile_size as u32 + self.y_offset as u32;
        (x, y)
    }

    pub fn with_layer(self, layer: impl LayerO2D) -> Self {
        Self {
            z_index: layer.get_z_index(),
            ..self
        }
    }

    pub fn with_offsets(self, x_offset: u8, y_offset: u8) -> Self {
        Self {
            x_offset,
            y_offset,
            ..self
        }
    }
}
