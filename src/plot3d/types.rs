#[derive(Debug)]
pub struct Plot3DBlock {
    pub imax: i32, 
    pub jmax: i32, 
    pub kmax: i32, 
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub z: Vec<f64>,
}