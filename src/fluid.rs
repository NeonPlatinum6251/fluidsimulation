pub struct FluidCube {
    pub size: usize,
    pub dt: f32,
    pub diff: f32,
    pub visc: f32,

    pub s: Vec<f32>,
    pub density: Vec<f32>,

    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
    pub vz: Vec<f32>,

    pub vx0: Vec<f32>,
    pub vy0: Vec<f32>,
    pub vz0: Vec<f32>,
}

impl FluidCube {
    pub fn new(size: usize, dt: f32, diff: f32, visc: f32) -> Self {
        let total_cells = size * size * size;

        Self {
            size,
            dt,
            diff,
            visc,

            s: vec![0.0; total_cells],
            density: vec![0.0; total_cells],

            vx: vec![0.0; total_cells],
            vy: vec![0.0; total_cells],
            vz: vec![0.0; total_cells],

            vx0: vec![0.0; total_cells],
            vy0: vec![0.0; total_cells],
            vz0: vec![0.0; total_cells],
        }
    }

    pub fn ix(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.size + z * self.size * self.size
    }
    pub fn AddDensity(&mut self, x:usize, y:iusize, z:usize, amount:f32) {
        let index = self.ix(x,y,z);
        self.density[index] += amount;
    }
    pub fn AddVelocity(&mut self, x:usize, y:usize, z:usize, amountY:f32, amountX:f32, amountZ:f32) {
        let index = self.ix(x,y,z);
        cube.Vx[index] += amountX;
        cube.Vy[index] += amountY;
        cube.Vz[index] += amountZ;
    }
}