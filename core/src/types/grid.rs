use crate::types::grid::cardinal::Cardinal;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

pub mod cardinal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Grid<T> {
    tiles: Vec<T>,
    width: u8,
    height: u8,
}

impl<T> Grid<T> {
    pub fn new(tiles: Vec<T>, width: u8, height: u8) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn width(&self) -> u8 {
        self.width
    }

    pub fn height(&self) -> u8 {
        self.height
    }

    pub fn get_tile(&self, x: u8, y: u8) -> Option<&T> {
        self.tiles
            .get(x as usize + y as usize * self.width as usize)
    }

    pub fn get_tile_mut(&mut self, x: u8, y: u8) -> Option<&mut T> {
        self.tiles
            .get_mut(x as usize + y as usize * self.width as usize)
    }

    pub fn apply_at<F>(&mut self, x: u8, y: u8, f: F)
    where
        F: Fn(&mut T),
    {
        if let Some(tile) = self.get_tile_mut(x, y) {
            f(tile);
        }
    }

    pub fn apply_at_range<F>(&mut self, min: (u8, u8), max: (u8, u8), mut f: F)
    where
        F: FnMut(&mut T),
    {
        if min.0 > max.0 || min.1 > max.1 {
            return;
        }

        for x in min.0..=max.0 {
            for y in min.1..=max.1 {
                if let Some(tile) = self.get_tile_mut(x, y) {
                    f(tile);
                }
            }
        }
    }

    pub fn get_neighbor_coordinates(&self, cardinal: Cardinal, x: u8, y: u8) -> Option<(u8, u8)> {
        match cardinal {
            Cardinal::North => {
                if y == 0 {
                    None
                } else {
                    Some((x, y - 1))
                }
            }
            Cardinal::East => {
                if x >= self.width - 1 {
                    None
                } else {
                    Some((x + 1, y))
                }
            }
            Cardinal::South => {
                if y >= self.height - 1 {
                    None
                } else {
                    Some((x, y + 1))
                }
            }
            Cardinal::West => {
                if x == 0 {
                    None
                } else {
                    Some((x - 1, y))
                }
            }
            Cardinal::NorthEast => {
                if x >= self.width - 1 || y == 0 {
                    None
                } else {
                    Some((x + 1, y - 1))
                }
            }
            Cardinal::SouthEast => {
                if x >= self.width - 1 || y >= self.height - 1 {
                    None
                } else {
                    Some((x + 1, y + 1))
                }
            }
            Cardinal::SouthWest => {
                if x == 0 || y >= self.height - 1 {
                    None
                } else {
                    Some((x - 1, y + 1))
                }
            }
            Cardinal::NorthWest => {
                if x == 0 || y == 0 {
                    None
                } else {
                    Some((x - 1, y - 1))
                }
            }
        }
    }

    pub fn get_neighbor_tile(&self, cardinal: Cardinal, x: u8, y: u8) -> Option<&T> {
        self.get_neighbor_coordinates(cardinal, x, y)
            .and_then(|coord| self.get_tile(coord.0, coord.1))
    }

    pub fn check_neighbor(&self, cardinal: Cardinal, x: u8, y: u8, f: impl Fn(&T) -> bool) -> bool {
        self.get_neighbor_tile(cardinal, x, y).map_or(false, f)
    }

    pub fn check_all_neighbors(&self, x: u8, y: u8, f: impl Fn(&T) -> bool) -> bool {
        for cardinal in Cardinal::iter() {
            if let Some(tile) = self.get_neighbor_tile(cardinal, x, y) {
                if !f(tile) {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn count_main_neighbors_with(&self, x: u8, y: u8, f: impl Fn(&T) -> bool) -> usize {
        let mut count = 0;
        for cardinal in Cardinal::iter_main() {
            if let Some(tile) = self.get_neighbor_tile(cardinal, x, y) {
                if f(tile) {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.tiles.into_iter()
    }

    pub fn iter_tiles(&self) -> impl Iterator<Item = &T> {
        self.tiles.iter()
    }

    pub fn iter_tiles_coords(&self) -> impl Iterator<Item = (&T, u8, u8)> {
        let width = self.width() as usize;
        (0..self.tiles.len()).map(move |i| {
            (
                self.tiles.get(i).unwrap(),
                (i % width) as u8,
                (i / width) as u8,
            )
        })
    }

    pub fn iter_tiles_coords_mut(&mut self) -> impl Iterator<Item = (&mut T, u8, u8)> {
        let width = self.width() as usize;
        self.tiles
            .iter_mut()
            .enumerate()
            .map(move |(i, tile)| (tile, (i % width) as u8, (i / width) as u8))
    }

    pub fn iter_tiles_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.tiles.iter_mut()
    }
}
