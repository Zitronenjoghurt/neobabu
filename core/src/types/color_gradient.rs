use image::Rgba;

pub struct ColorStop {
    pub value: f32,
    pub color: (f32, f32, f32, f32),
}

pub struct ColorGradient {
    stops: Vec<ColorStop>,
}

impl ColorGradient {
    pub fn new(stops: Vec<ColorStop>) -> Self {
        Self { stops }
    }

    pub fn get_color(&self, value: f32) -> Option<Rgba<u8>> {
        if self.stops.is_empty() {
            return None;
        }

        let next_index = self
            .stops
            .iter()
            .position(|stop| stop.value > value)
            .unwrap_or(0);

        let prev_index = if next_index == 0 {
            self.stops.len() - 1
        } else {
            next_index - 1
        };

        let prev = &self.stops[prev_index];
        let next = &self.stops[next_index];

        let difference = next.value - prev.value;
        let progress = value - prev.value;
        let ratio = progress / difference;

        Some(Rgba([
            (lerp(prev.color.0, next.color.0, ratio) * 255.0) as u8,
            (lerp(prev.color.1, next.color.1, ratio) * 255.0) as u8,
            (lerp(prev.color.2, next.color.2, ratio) * 255.0) as u8,
            (lerp(prev.color.3, next.color.3, ratio) * 255.0) as u8,
        ]))
    }
}

fn lerp(start: f32, end: f32, ratio: f32) -> f32 {
    start + (end - start) * ratio
}
