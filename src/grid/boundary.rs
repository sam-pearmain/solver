#[derive(Debug, Clone, Copy)]
pub enum Boundary {
    IMin,
    IMax,
    JMin, 
    JMax,
    KMin,
    KMax,
}

#[derive(Debug, Clone, Copy)]
pub enum BoundaryType {
    Wall,
    VelocityInlet,
    PressureOutlet,
}