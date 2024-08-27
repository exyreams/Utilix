use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

pub struct ColorConverter {
    pub input: String,
    pub cmyk: String,
    pub rgb: String,
    pub hex: String,
    pub hsl: String,
    pub tools_export_message: Option<String>,
}

impl ColorConverter {
    pub fn new() -> Self {
        ColorConverter {
            input: String::new(),
            cmyk: String::new(),
            rgb: String::new(),
            hex: String::new(),
            hsl: String::new(),
            tools_export_message: None,
        }
    }

    pub fn convert_to_cmyk(&mut self) {
        if let Some(color) = parse_color(&self.input) {
            self.cmyk = convert_to_cmyk(&color);
        } else {
            self.cmyk = "Invalid color format".to_string();
        }
    }

    pub fn convert_to_rgb(&mut self) {
        if let Some(color) = parse_color(&self.input) {
            self.rgb = convert_to_rgb(&color);
        } else {
            self.rgb = "Invalid color format".to_string();
        }
    }

    pub fn convert_to_hex(&mut self) {
        if let Some(color) = parse_color(&self.input) {
            self.hex = convert_to_hex(&color);
        } else {
            self.hex = "Invalid color format".to_string();
        }
    }

    pub fn convert_to_hsl(&mut self) {
        if let Some(color) = parse_color(&self.input) {
            self.hsl = convert_to_hsl(&color);
        } else {
            self.hsl = "Invalid color format".to_string();
        }
    }

    pub fn convert_all(&mut self) {
        self.convert_to_cmyk();
        self.convert_to_rgb();
        self.convert_to_hex();
        self.convert_to_hsl();
    }

    pub fn export_color_codes(&self) -> std::io::Result<()> {
        let file_path = Path::new("export/color_codes.txt");
        if let Some(parent) = file_path.parent() {
            create_dir_all(parent)?;
        }
        let mut file = File::create(file_path)?;

        writeln!(file, "Entered Color Code: {}", self.input)?;
        writeln!(file, "CMYK: {}", self.cmyk)?;
        writeln!(file, "RGB: {}", self.rgb)?;
        writeln!(file, "HEX: {}", self.hex)?;
        writeln!(file, "HSL: {}", self.hsl)?;
        Ok(())
    }
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn convert_to_cmyk(color: &Color) -> String {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;

    let k = 1.0 - r.max(g).max(b);
    let c = (1.0 - r - k) / (1.0 - k);
    let m = (1.0 - g - k) / (1.0 - k);
    let y = (1.0 - b - k) / (1.0 - k);

    format!("{:.0}%, {:.0}%, {:.0}%, {:.0}%", c * 100.0, m * 100.0, y * 100.0, k * 100.0)
}

fn convert_to_rgb(color: &Color) -> String {
    format!("{}, {}, {}", color.r, color.g, color.b)
}

fn convert_to_hex(color: &Color) -> String {
    format!("#{:02X}{:02X}{:02X}", color.r, color.g, color.b)
}

fn convert_to_hsl(color: &Color) -> String {
    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let diff = max - min;

    let h = if max == min {
        0.0
    } else if max == r {
        (60.0 * ((g - b) / diff) + 360.0) % 360.0
    } else if max == g {
        60.0 * ((b - r) / diff) + 120.0
    } else {
        60.0 * ((r - g) / diff) + 240.0
    };

    let l = (max + min) / 2.0;

    let s = if l == 0.0 || max == min {
        0.0
    } else if l <= 0.5 {
        diff / (max + min)
    } else {
        diff / (2.0 - max - min)
    };

    format!("{:.0}°, {:.0}%, {:.0}%", h, s * 100.0, l * 100.0)
}

fn parse_color(input: &str) -> Option<Color> {
    parse_hex(input)
        .or_else(|| parse_rgb(input))
        .or_else(|| parse_cmyk(input))
        .or_else(|| parse_hsl(input))
}

fn parse_hex(input: &str) -> Option<Color> {
    let input = input.trim_start_matches('#');
    if input.len() != 6 {
        return None;
    }
    u32::from_str_radix(input, 16).ok().map(|rgb| Color {
        r: ((rgb >> 16) & 0xFF) as u8,
        g: ((rgb >> 8) & 0xFF) as u8,
        b: (rgb & 0xFF) as u8,
    })
}

fn parse_rgb(input: &str) -> Option<Color> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let r = parts[0].trim().parse().ok()?;
    let g = parts[1].trim().parse().ok()?;
    let b = parts[2].trim().parse().ok()?;
    Some(Color { r, g, b })
}

fn parse_cmyk(input: &str) -> Option<Color> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 4 {
        return None;
    }
    let c: f32 = parts[0].trim().trim_end_matches('%').parse().ok()?;
    let m: f32 = parts[1].trim().trim_end_matches('%').parse().ok()?;
    let y: f32 = parts[2].trim().trim_end_matches('%').parse().ok()?;
    let k: f32 = parts[3].trim().trim_end_matches('%').parse().ok()?;

    let r = (255.0 * (1.0 - c / 100.0) * (1.0 - k / 100.0)) as u8;
    let g = (255.0 * (1.0 - m / 100.0) * (1.0 - k / 100.0)) as u8;
    let b = (255.0 * (1.0 - y / 100.0) * (1.0 - k / 100.0)) as u8;

    Some(Color { r, g, b })
}

fn parse_hsl(input: &str) -> Option<Color> {
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() != 3 {
        return None;
    }
    let h: f32 = parts[0].trim().trim_end_matches('°').parse().ok()?;
    let s: f32 = parts[1].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;
    let l: f32 = parts[2].trim().trim_end_matches('%').parse::<f32>().ok()? / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    Some(Color {
        r: ((r + m) * 255.0) as u8,
        g: ((g + m) * 255.0) as u8,
        b: ((b + m) * 255.0) as u8,
    })
}

