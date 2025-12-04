use crate::games::farming::hemisphere::Hemisphere;
use crate::games::farming::procedural::tile::ProceduralTile;
use crate::games::farming::tile::FarmTile;
use crate::games::farming::world::FarmWorld;
use crate::types::grid::Grid;
use chrono_tz::Tz;
use noise::{NoiseFn, Perlin};

mod tile;

pub struct ProceduralWorld {
    pub height: u8,
    pub width: u8,
    pub hemisphere: Hemisphere,
    pub tz: Tz,
    pub seed: u32,
    pub scale: f64,
    pub threshold: f64,
}

impl ProceduralWorld {
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn generate(&self) -> FarmWorld {
        let tiles = vec![ProceduralTile::default(); self.width as usize * self.height as usize];
        let mut grid = Grid::new(tiles, self.width, self.height);

        self.perlin_stage(&mut grid);

        self.build_world(grid)
    }

    fn build_tile(&self, tile: ProceduralTile) -> FarmTile {
        if tile.contains(ProceduralTile::GROUND) {
            FarmTile::new_land()
        } else {
            FarmTile::default()
        }
    }

    fn build_world(&self, tiles: Grid<ProceduralTile>) -> FarmWorld {
        let farm_tiles = tiles
            .into_iter()
            .map(|tile| self.build_tile(tile))
            .collect();

        FarmWorld {
            grid: Grid::new(farm_tiles, self.width, self.height),
            hemisphere: self.hemisphere,
            tz: self.tz,
        }
    }

    fn perlin_stage(&self, tiles: &mut Grid<ProceduralTile>) {
        let perlin = Perlin::new(self.seed);

        for (tile, x, y) in tiles.iter_tiles_coords_mut() {
            let nx = x as f64 / self.width as f64;
            let ny = y as f64 / self.height as f64;

            let dx = nx - 0.5;
            let dy = ny - 0.5;
            let dist = (dx * dx + dy * dy).sqrt();

            let raw_noise = perlin.get([nx * self.scale, ny * self.scale]);
            let normalized_noise = (raw_noise + 1.0) / 2.0;

            let island_mask = dist * 2.0;
            let height = normalized_noise - island_mask;

            if height > self.threshold {
                tile.insert(ProceduralTile::GROUND);
            }
        }
    }
}
