#![allow(dead_code)]

trait Tuple {
    fn len() -> usize;
}

impl Tuple for (f64,) {
    fn len() -> usize {
        1
    }
}

impl Tuple for (f64, f64) {
    fn len() -> usize {
        2
    }
}

impl Tuple for (f64, f64, f64) {
    fn len() -> usize {
        3
    }
}

pub trait Dimensioned {
    fn is_1d(&self) -> bool;
    fn is_2d(&self) -> bool;
    fn is_3d(&self) -> bool;
    fn dimensions(&self) -> usize;
}

impl<T> Dimensioned for T 
where 
    T: Node,
{
    fn dimensions(&self) -> usize {
        <T as Node>::Coordinates::len()
    }

    fn is_1d(&self) -> bool {
        self.dimensions() == 1
    }

    fn is_2d(&self) -> bool {
        self.dimensions() == 2
    }

    fn is_3d(&self) -> bool {
        self.dimensions() == 3
    }
}

pub trait Node: Dimensioned {
    type Coordinates: Tuple;

    fn new(id: usize, coords: Self::Coordinates) -> Self;
    fn coords(&self) -> Self::Coordinates;
    fn id(&self) -> usize;
}

pub struct Node1D {
    id: usize, 
    x: f64,
}

impl Node for Node1D {
    type Coordinates = (f64, );

    fn new(id: usize, coords: Self::Coordinates) -> Self {
        Self { id, x: coords.0 }
    }

    fn coords(&self) -> Self::Coordinates {
        (self.x, )
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl Node1D {
    pub fn new_from_x(id: usize, x: f64) -> Self {
        Self::new(id, (x, ))
    }
}

pub struct Node2D {
    id: usize, 
    x: f64, 
    y: f64,
}

impl Node for Node2D {
    type Coordinates = (f64, f64);

    fn new(id: usize, coords: Self::Coordinates) -> Self {
        Self { id, x: coords.0, y: coords.1 }
    }

    fn coords(&self) -> Self::Coordinates {
        (self.x, self.y)    
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl Node2D {
    fn new_from_xy(id: usize, x: f64, y: f64) -> Self {
        Self::new(id, (x, y))
    }
}

pub struct Node3D {
    id: usize, 
    x: f64,
    y: f64,
    z: f64,
}

impl Node for Node3D {
    type Coordinates = (f64, f64, f64);

    fn new(id: usize, coords: Self::Coordinates) -> Self {
        Self { id, x: coords.0, y: coords.1, z: coords.2 }
    }

    fn coords(&self) -> Self::Coordinates {
        (self.x, self.y, self.z)
    }

    fn id(&self) -> usize {
        self.id
    }
}

impl Node3D {
    pub fn new_from_xyz(id: usize, x: f64, y: f64, z: f64) -> Self {
        Self::new(id, (x, y, z))
    }
}