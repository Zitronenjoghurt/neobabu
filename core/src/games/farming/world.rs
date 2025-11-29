use crate::error::CoreResult;
use crate::games::farming::layer::FarmLayer;
use crate::games::farming::soil::SoilFlags;
use crate::games::farming::tile::FarmTile;
use crate::rendering::o2d::prelude::{O2DRenderer, Object2D, PositionO2D, SpriteId, VisualO2D};
use image::RgbaImage;

pub struct FarmWorld {
    pub tiles: Vec<FarmTile>,
    pub height: u8,
    pub width: u8,
}

impl FarmWorld {
    pub fn new_empty(width: u8, height: u8) -> Self {
        let mut tiles = Vec::with_capacity(width as usize * height as usize);
        for _ in 0..width {
            for _ in 0..height {
                tiles.push(FarmTile::default());
            }
        }
        Self {
            tiles,
            height,
            width,
        }
    }

    pub fn render(&self, o2d: &O2DRenderer, debug: bool) -> CoreResult<RgbaImage> {
        let mut objects = Vec::new();
        for x in 0..self.width {
            for y in 0..self.height {
                if let Some(tile) = self.get_tile(x, y) {
                    objects.extend(tile.render_objects(x, y));
                }
            }
        }
        objects.push(Object2D {
            position: PositionO2D::from_tile_xy(0, 0).with_layer(FarmLayer::OnGround),
            visual: VisualO2D::Sprite(SpriteId::Carrot5),
        });

        if debug {
            o2d.render_debug(&objects, self.height, self.width, 16)
        } else {
            o2d.render(&objects, self.height, self.width, 16)
        }
    }

    pub fn get_tile(&self, x: u8, y: u8) -> Option<&FarmTile> {
        self.tiles
            .get(x as usize + y as usize * self.width as usize)
    }

    pub fn get_tile_mut(&mut self, x: u8, y: u8) -> Option<&mut FarmTile> {
        self.tiles
            .get_mut(x as usize + y as usize * self.width as usize)
    }

    pub fn apply_at<F>(&mut self, x: u8, y: u8, mut f: F)
    where
        F: FnMut(&mut FarmTile),
    {
        if let Some(tile) = self.get_tile_mut(x, y) {
            f(tile);
        }
    }

    pub fn till_at(&mut self, x: u8, y: u8) {
        self.apply_at(x, y, |tile| tile.soil.insert(SoilFlags::TILLED));
    }

    pub fn water_at(&mut self, x: u8, y: u8) {
        self.apply_at(x, y, |tile| tile.soil.insert(SoilFlags::WATERED));
    }
}
