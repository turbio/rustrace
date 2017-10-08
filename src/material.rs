use color::Color;

#[derive(Clone)]
pub struct Material {
    pub ambient: Color,
    pub diffuse: Color,
    pub specular: Color,
    pub shininess: f64,
    pub reflectivity: f64,
}
