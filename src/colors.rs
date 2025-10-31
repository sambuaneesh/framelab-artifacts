use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct ColorGenerator {
    tool_colors: HashMap<String, String>,
    base_hues: Vec<f64>,
}

impl ColorGenerator {
    pub fn new() -> Self {
        Self {
            tool_colors: HashMap::new(),
            base_hues: vec![
                210.0, // Blue
                270.0, // Purple
                150.0, // Green
                30.0,  // Orange
                0.0,   // Red
                330.0, // Pink
                180.0, // Cyan
                120.0, // Lime
                240.0, // Indigo
                60.0,  // Yellow
                300.0, // Magenta
                90.0,  // Chartreuse
            ],
        }
    }

    pub fn generate_tool_colors(&mut self, tools: &[String]) {
        let unique_tools: Vec<String> = tools
            .iter()
            .map(|tv| tv.split(" (").next().unwrap_or("").to_string())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        let hue_step = 360.0 / unique_tools.len().max(1) as f64;
        
        for (i, tool) in unique_tools.iter().enumerate() {
            let hue = if i < self.base_hues.len() {
                self.base_hues[i]
            } else {
                (i as f64 * hue_step) % 360.0
            };
            
            let color = Self::hsl_to_hex(hue, 70.0, 60.0);
            self.tool_colors.insert(tool.clone(), color);
        }
    }

    pub fn get_color(&self, tool_variant: &str) -> String {
        let parts: Vec<&str> = tool_variant.split(" (").collect();
        let tool = parts.get(0).unwrap_or(&"").to_string();
        let variant = parts.get(1).map(|v| v.trim_end_matches(')')).unwrap_or("");

        if let Some(base_color) = self.tool_colors.get(&tool) {
            if variant.is_empty() {
                return base_color.clone();
            }

            let variant_hash = Self::hash_string(variant);
            let lightness_offset = (variant_hash % 35) as f64 - 17.5;
            let saturation_offset = ((variant_hash / 35) % 25) as f64 - 12.5;
            let hue_offset = ((variant_hash / 70) % 15) as f64 - 7.5;

            let (h, s, l) = Self::hex_to_hsl(base_color);
            let new_h = (h + hue_offset + 360.0) % 360.0;
            let new_l = (l + lightness_offset).clamp(35.0, 80.0);
            let new_s = (s + saturation_offset).clamp(50.0, 90.0);
            
            Self::hsl_to_hex(new_h, new_s, new_l)
        } else {
            "#64748b".to_string()
        }
    }

    fn hash_string(s: &str) -> u32 {
        s.bytes().fold(0u32, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u32))
    }

    fn hsl_to_hex(h: f64, s: f64, l: f64) -> String {
        let h = h / 360.0;
        let s = s / 100.0;
        let l = l / 100.0;

        let (r, g, b) = if s == 0.0 {
            let val = (l * 255.0) as u8;
            (val, val, val)
        } else {
            let q = if l < 0.5 {
                l * (1.0 + s)
            } else {
                l + s - l * s
            };
            let p = 2.0 * l - q;

            let r = Self::hue_to_rgb(p, q, h + 1.0 / 3.0);
            let g = Self::hue_to_rgb(p, q, h);
            let b = Self::hue_to_rgb(p, q, h - 1.0 / 3.0);

            ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
        };

        format!("#{:02x}{:02x}{:02x}", r, g, b)
    }

    fn hex_to_hsl(hex: &str) -> (f64, f64, f64) {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0) as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0) as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0) as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max == min {
            return (0.0, 0.0, l * 100.0);
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if max == r {
            ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
        } else if max == g {
            ((b - r) / d + 2.0) / 6.0
        } else {
            ((r - g) / d + 4.0) / 6.0
        };

        (h * 360.0, s * 100.0, l * 100.0)
    }

    fn hue_to_rgb(p: f64, q: f64, t: f64) -> f64 {
        let mut t = t;
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        p
    }
}
