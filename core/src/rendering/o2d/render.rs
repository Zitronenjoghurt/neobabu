use crate::error::CoreResult;
use crate::rendering::o2d::atlas::cache::AtlasCache;
use crate::rendering::o2d::object;
use crate::rendering::o2d::object::{Object, ObjectGrid};
use crate::rendering::o2d::sprite::Sprite;
use image::{Pixel, RgbaImage};
use std::sync::Arc;

pub trait O2DRenderable {
    fn to_objects(&self) -> Vec<Object>;
}

impl O2DRenderable for Object {
    fn to_objects(&self) -> Vec<Object> {
        vec![*self]
    }
}

pub struct O2DRenderer {
    atlas_cache: Arc<AtlasCache>,
}

impl O2DRenderer {
    pub fn initialize() -> CoreResult<Arc<Self>> {
        Ok(Arc::new(Self {
            atlas_cache: AtlasCache::initialize()?,
        }))
    }

    pub fn render<R: O2DRenderable>(
        &self,
        renderables: &[R],
        tile_height: u8,
        tile_width: u8,
        tile_size: u8,
    ) -> CoreResult<RgbaImage> {
        let mut grid = object::ObjectGrid::from_objects(
            renderables.iter().flat_map(|r| r.to_objects()).collect(),
            tile_height,
            tile_width,
        );
        grid.y_sort();

        let width = tile_width as u32 * tile_size as u32;
        let height = tile_height as u32 * tile_size as u32;
        let mut canvas = RgbaImage::new(width, height);

        let mut object_refs = grid.iter_objects().collect::<Vec<_>>();
        object_refs.sort_by_key(|o| o.position.tile_y);

        for object in object_refs {
            self.render_object(&mut canvas, object, &grid, tile_size)?;
        }

        Ok(canvas)
    }

    fn render_object(
        &self,
        canvas: &mut RgbaImage,
        object: &Object,
        grid: &ObjectGrid,
        tile_size: u8,
    ) -> CoreResult<()> {
        match &object.visual {
            object::Visual::Sprite(sprite_id) => {
                let sprite = sprite_id.get_sprite();
                let pos = object.position.pixel_position(tile_size);
                self.draw_sprite(canvas, &sprite, pos)?;
            }
            object::Visual::Tile(tileset_id) => {
                let mask = grid.determine_tile_mask(
                    object.position.tile_x,
                    object.position.tile_y,
                    *tileset_id,
                );
                let sprite = tileset_id.tile_set().get_sprite(mask);
                let pos = object.position.pixel_position(tile_size);
                self.draw_sprite(canvas, &sprite, pos)?;
            }
        }
        Ok(())
    }

    fn draw_sprite(
        &self,
        canvas: &mut RgbaImage,
        sprite: &Sprite,
        pos: (u32, u32),
    ) -> CoreResult<()> {
        let atlas = self.atlas_cache.get(sprite.atlas_id)?;

        for y in 0..sprite.rect.height {
            for x in 0..sprite.rect.width {
                let src_x = sprite.rect.x + x;
                let src_y = sprite.rect.y + y;
                let dst_x = pos.0 + x;
                let dst_y = pos.1 + y;

                if dst_x < canvas.width() && dst_y < canvas.height() {
                    if let Some(src_pixel) = atlas.get_pixel(src_x, src_y) {
                        let dst_pixel = canvas.get_pixel_mut(dst_x, dst_y);
                        dst_pixel.blend(src_pixel);
                    }
                }
            }
        }

        Ok(())
    }
}
