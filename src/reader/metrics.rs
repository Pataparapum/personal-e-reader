use ttf_parser::Face;

/// Mide el ancho de texto usando los anchos de letra de la fuente (sin kerning).
pub struct Metrics {
    face: Face<'static>,
    units: f32,
}

impl Metrics {
    pub fn new(bytes: &'static [u8]) -> Self {
        let face = Face::parse(bytes, 0).expect("fuente inválida");
        let units = f32::from(face.units_per_em());
        Self { face, units }
    }

    pub fn advance(&self, c: char, size: f32) -> f32 {
        let units = self
            .face
            .glyph_index(c)
            .and_then(|g| self.face.glyph_hor_advance(g))
            .map(f32::from)
            .unwrap_or(self.units * 0.5);
        units / self.units * size
    }

    pub fn width(&self, s: &str, size: f32) -> f32 {
        s.chars().map(|c| self.advance(c, size)).sum()
    }
}