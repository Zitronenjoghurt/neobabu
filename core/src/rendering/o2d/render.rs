use crate::error::CoreResult;
use crate::rendering::o2d::atlas::cache::AtlasCache;
use crate::rendering::o2d::object::grid::O2DGrid;
use crate::rendering::o2d::object::visual::VisualO2D;
use crate::rendering::o2d::object::Object2D;
use crate::rendering::o2d::sprite::Sprite;
use image::{Pixel, RgbaImage};
use std::sync::Arc;

pub trait O2DRenderable {
    fn to_objects(&self) -> Vec<Object2D>;
}

impl O2DRenderable for Object2D {
    fn to_objects(&self) -> Vec<Object2D> {
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
        let all_objects: Vec<Object2D> = renderables.iter().flat_map(|r| r.to_objects()).collect();

        let (effects, geometry): (Vec<_>, Vec<_>) = all_objects
            .into_iter()
            .partition(|obj| obj.visual.is_effect());

        let mut grid = O2DGrid::from_objects(geometry, tile_height, tile_width);
        grid.sort();

        let width = tile_width as u32 * tile_size as u32;
        let height = tile_height as u32 * tile_size as u32;
        let mut canvas = RgbaImage::new(width, height);

        let mut object_refs = grid.iter_objects().collect::<Vec<_>>();
        object_refs.sort_by_key(|o| o.position.tile_y);

        for object in object_refs {
            self.render_object(&mut canvas, object, &grid, tile_size)?;
        }

        for effect in effects {
            self.render_object(&mut canvas, &effect, &grid, tile_size)?;
        }

        Ok(canvas)
    }

    pub fn render_debug<R: O2DRenderable>(
        &self,
        renderables: &[R],
        tile_height: u8,
        tile_width: u8,
        tile_size: u8,
    ) -> CoreResult<RgbaImage> {
        let mut canvas = self.render(renderables, tile_height, tile_width, tile_size)?;

        let grid_color = image::Rgba([255, 255, 255, 50]);

        for tile_x in 0..tile_width {
            for tile_y in 0..tile_height {
                let base_x = tile_x as u32 * tile_size as u32;
                let base_y = tile_y as u32 * tile_size as u32;

                for x in 0..tile_size as u32 {
                    let pixel_x = base_x + x;
                    if pixel_x < canvas.width() {
                        if base_y < canvas.height() {
                            canvas.get_pixel_mut(pixel_x, base_y).blend(&grid_color);
                        }
                        let bottom_y = base_y + tile_size as u32 - 1;
                        if bottom_y < canvas.height() {
                            canvas.get_pixel_mut(pixel_x, bottom_y).blend(&grid_color);
                        }
                    }
                }

                for y in 0..tile_size as u32 {
                    let pixel_y = base_y + y;
                    if pixel_y < canvas.height() {
                        if base_x < canvas.width() {
                            canvas.get_pixel_mut(base_x, pixel_y).blend(&grid_color);
                        }
                        let right_x = base_x + tile_size as u32 - 1;
                        if right_x < canvas.width() {
                            canvas.get_pixel_mut(right_x, pixel_y).blend(&grid_color);
                        }
                    }
                }
            }
        }

        Ok(canvas)
    }

    fn render_object(
        &self,
        canvas: &mut RgbaImage,
        object: &Object2D,
        grid: &O2DGrid,
        tile_size: u8,
    ) -> CoreResult<()> {
        match &object.visual {
            VisualO2D::Color(color) => {
                let pos = object.position.pixel_position(tile_size);
                self.draw_color(canvas, color, pos, tile_size);
            }
            VisualO2D::Filter(color) => {
                self.blend_all(canvas, color);
            }
            VisualO2D::Sprite(sprite_id) => {
                let sprite = sprite_id.get_sprite();
                let pos = object.position.pixel_position(tile_size);
                self.draw_sprite(canvas, &sprite, pos, tile_size)?;
            }
            VisualO2D::Tile(tileset_id) => {
                let mask = grid.determine_tile_mask(
                    object.position.tile_x,
                    object.position.tile_y,
                    *tileset_id,
                );
                let sprite = tileset_id.get_sprite(mask);
                let pos = object.position.pixel_position(tile_size);
                self.draw_sprite(canvas, &sprite, pos, tile_size)?;
            }
        }
        Ok(())
    }

    fn draw_color(
        &self,
        canvas: &mut RgbaImage,
        color: &image::Rgba<u8>,
        pos: (u32, u32),
        tile_size: u8,
    ) {
        for y in 0..tile_size as u32 {
            for x in 0..tile_size as u32 {
                let dst_x = pos.0 + x;
                let dst_y = pos.1 + y;
                if dst_x < canvas.width() && dst_y < canvas.height() {
                    let dst_pixel = canvas.get_pixel_mut(dst_x, dst_y);
                    dst_pixel.blend(color);
                }
            }
        }
    }

    fn blend_all(&self, canvas: &mut RgbaImage, color: &image::Rgba<u8>) {
        for pixel in canvas.pixels_mut() {
            pixel.blend(color);
        }
    }

    fn draw_sprite(
        &self,
        canvas: &mut RgbaImage,
        sprite: &Sprite,
        pos: (u32, u32),
        tile_size: u8,
    ) -> CoreResult<()> {
        let atlas = self.atlas_cache.get(sprite.atlas_id)?;

        // Accounting for sprites higher than tile size
        // This will ensure that the sprite is drawn at the expected position
        // The bottom-left tile of the sprite will be its origin
        let y_adjust = sprite.rect.height.saturating_sub(tile_size as u32);

        for y in 0..sprite.rect.height {
            for x in 0..sprite.rect.width {
                let src_x = sprite.rect.x + x;
                let src_y = sprite.rect.y + y;
                let dst_x = pos.0 + x;
                let dst_y = pos.1 + y;

                let Some(adjusted_y) = dst_y.checked_sub(y_adjust) else {
                    continue;
                };

                let Some(src_pixel) = atlas.get_pixel(src_x, src_y) else {
                    continue;
                };

                if dst_x < canvas.width() && adjusted_y < canvas.height() {
                    let dst_pixel = canvas.get_pixel_mut(dst_x, adjusted_y);
                    dst_pixel.blend(src_pixel);
                }
            }
        }

        Ok(())
    }
}
