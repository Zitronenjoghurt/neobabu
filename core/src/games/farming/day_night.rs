use crate::types::color_gradient::{ColorGradient, ColorStop};
use chrono::Timelike;
use image::Rgba;

fn color_gradient() -> ColorGradient {
    ColorGradient::new(vec![
        ColorStop {
            value: 0.0,
            color: (0.0, 0.02, 0.08, 0.85),
        },
        ColorStop {
            value: 5.0,
            color: (0.08, 0.06, 0.12, 0.70),
        },
        ColorStop {
            value: 7.0,
            color: (1.0, 0.60, 0.37, 0.20),
        },
        ColorStop {
            value: 10.0,
            color: (1.0, 1.0, 1.0, 0.00),
        },
        ColorStop {
            value: 16.0,
            color: (1.0, 1.0, 1.0, 0.00),
        },
        ColorStop {
            value: 18.0,
            color: (1.0, 0.44, 0.12, 0.30),
        },
        ColorStop {
            value: 20.0,
            color: (0.06, 0.02, 0.14, 0.60),
        },
        ColorStop {
            value: 24.0,
            color: (0.0, 0.02, 0.08, 0.85),
        },
    ])
}

pub fn day_night_color(tz: chrono_tz::Tz) -> Rgba<u8> {
    let now = chrono::Utc::now().with_timezone(&tz);
    color_gradient().get_color(now.hour() as f32).unwrap()
}
