use crate::games::farming::object::FarmObject;

pub struct FarmWorld {
    pub objects: Vec<FarmObject>,
}

impl FarmWorld {
    pub fn new_empty(width: u8, height: u8) -> Self {
        let mut objects = Vec::with_capacity(width as usize * height as usize);
        for x in 0..width {
            for y in 0..height {
                objects.push(FarmObject::untilled_farmland(x, y))
            }
        }
        Self { objects }
    }

    pub fn apply_at<F>(&mut self, x: u8, y: u8, mut f: F)
    where
        F: FnMut(&mut FarmObject),
    {
        self.objects
            .iter_mut()
            .filter(|object| {
                let (tile_x, tile_y) = object.get_tile_coords();
                tile_x == x && tile_y == y
            })
            .for_each(|object| f(object));
    }

    pub fn till_at(&mut self, x: u8, y: u8) {
        self.apply_at(x, y, |object| object.till());
    }

    pub fn water_at(&mut self, x: u8, y: u8) {
        self.apply_at(x, y, |object| object.water());
    }
}
