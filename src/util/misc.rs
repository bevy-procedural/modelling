

/// Converts a color from RGB format (f32, \[0,1]) to HEX format (u8, \[0,FF]).
pub fn rgb2hex(r: f32, g: f32, b: f32) -> String {
    format!(
        "#{:02x}{:02x}{:02x}",
        (255.0 * r) as u8,
        (255.0 * g) as u8,
        (255.0 * b) as u8
    )
}
