use crate::error::CoreResult;
use crate::rendering::o2d::atlas::cache::AtlasCache;
use crate::rendering::o2d::bitmap::Bitmap;
use crate::rendering::o2d::object::grid::O2DGrid;
use crate::rendering::o2d::object::visual::{TextVisual, VisualO2D};
use crate::rendering::o2d::object::Object2D;
use crate::rendering::o2d::sprite::Sprite;
use crate::rendering::o2d::text::char_bitmap;
use image::{Pixel, Rgba, RgbaImage};
use std::sync::Arc;

pub trait O2DRenderable {
    fn to_objects(&self) -> Vec<Object2D>;
}

impl O2DRenderable for Object2D {
    fn to_objects(&self) -> Vec<Object2D> {
        vec![self.clone()]
    }
}

#[derive(Clone)]
pub struct O2DRenderer {
    atlas_cache: Arc<AtlasCache>,
}

impl O2DRenderer {
    pub fn initialize() -> CoreResult<Self> {
        Ok(Self {
            atlas_cache: AtlasCache::initialize()?,
        })
    }

    #[tracing::instrument(level = "trace", skip_all)]
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

        let grid = O2DGrid::from_objects(geometry, tile_height, tile_width);

        let width = tile_width as u32 * tile_size as u32;
        let height = tile_height as u32 * tile_size as u32;
        let mut canvas = RgbaImage::new(width, height);

        let mut object_refs = grid.iter_objects().collect::<Vec<_>>();
        object_refs.sort_by_key(|o| (o.position.z_index, o.position.tile_y, o.position.y_offset));

        for object in object_refs {
            self.render_object(&mut canvas, object, &grid, tile_size)?;
        }

        let ambience_color = effects.iter().find_map(|o| match o.visual {
            VisualO2D::Ambience(color) => Some(color),
            _ => None,
        });

        let point_lights: Vec<&Object2D> = effects
            .iter()
            .filter(|obj| matches!(obj.visual, VisualO2D::PointLight { .. }))
            .collect();

        if let Some(ambience_color) = ambience_color {
            self.apply_lighting(&mut canvas, &ambience_color, &point_lights, tile_size);
        }

        Ok(canvas)
    }

    #[tracing::instrument(level = "trace", skip_all)]
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
            VisualO2D::Text(text) => {
                self.draw_text(canvas, text, object.position.pixel_position(tile_size))
            }
            _ => {}
        }
        Ok(())
    }

    fn draw_color(&self, canvas: &mut RgbaImage, color: &Rgba<u8>, pos: (u32, u32), tile_size: u8) {
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

    fn draw_bitmap(
        &self,
        canvas: &mut RgbaImage,
        bitmap: &Bitmap<'_>,
        color: Rgba<u8>,
        pos: (u32, u32),
    ) {
        for (x_rel, y_rel, is_set) in bitmap.iter_pixels() {
            let x = ((pos.0 as isize).saturating_add(x_rel)) as u32;
            let y = ((pos.1 as isize).saturating_add(y_rel)) as u32;
            if x < canvas.width() && y < canvas.height() && is_set {
                let pixel = canvas.get_pixel_mut(x, y);
                *pixel = color;
            }
        }
    }

    fn draw_text(&self, canvas: &mut RgbaImage, visual: &TextVisual, pos: (u32, u32)) {
        let mut x_offset: usize = 0;
        let mut y_offset: usize = 0;
        for char in visual.text.chars() {
            let bitmap = char_bitmap(char);
            let max_width = match visual.max_width {
                Some(max_width) => max_width,
                None => canvas.width().saturating_sub(pos.0),
            };

            if visual.wrapping && x_offset.saturating_add(bitmap.width) >= max_width as usize {
                x_offset = 0;
                y_offset = y_offset.saturating_add(bitmap.height() + 1);
            };

            self.draw_bitmap(
                canvas,
                &bitmap,
                visual.color,
                (
                    pos.0.wrapping_add(x_offset as u32),
                    pos.1.wrapping_add(y_offset as u32),
                ),
            );
            x_offset = x_offset.wrapping_add(bitmap.width).wrapping_add(1)
        }
    }

    fn apply_lighting(
        &self,
        canvas: &mut RgbaImage,
        ambience_color: &image::Rgba<u8>,
        lights: &[&Object2D],
        tile_size: u8,
    ) {
        let width = canvas.width();
        let height = canvas.height();

        let mut lightmap = RgbaImage::from_pixel(width, height, *ambience_color);

        for light_object in lights {
            let VisualO2D::PointLight(light) = &light_object.visual else {
                continue;
            };

            let (center_x, center_y) = light_object.position.pixel_center(tile_size);

            let radius_int = light.radius.ceil() as u32;
            let min_x = center_x.saturating_sub(radius_int);
            let max_x = (center_x + radius_int).min(width);
            let min_y = center_y.saturating_sub(radius_int);
            let max_y = (center_y + radius_int).min(height);

            for y in min_y..max_y {
                for x in min_x..max_x {
                    let dx = x as f32 - center_x as f32;
                    let dy = y as f32 - center_y as f32;

                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq > light.radius * light.radius {
                        continue;
                    }

                    let distance = dist_sq.sqrt();

                    let normalized_dist = distance / light.radius;
                    let falloff = (1.0 - normalized_dist).powi(2);

                    let brightness = falloff * light.intensity;

                    let pixel = lightmap.get_pixel_mut(x, y);

                    let r = lerp(pixel[0], light.color[0], brightness);
                    let g = lerp(pixel[1], light.color[1], brightness);
                    let b = lerp(pixel[2], light.color[2], brightness);
                    let a = lerp(pixel[3], light.color[3], brightness);

                    *pixel = image::Rgba([r, g, b, a]);
                }
            }
        }

        for (x, y, pixel) in lightmap.enumerate_pixels() {
            canvas.get_pixel_mut(x, y).blend(pixel);
        }
    }
}

fn lerp(start: u8, end: u8, t: f32) -> u8 {
    let start_f = start as f32;
    let end_f = end as f32;
    let t_clamped = t.clamp(0.0, 1.0);
    (start_f + (end_f - start_f) * t_clamped) as u8
}
