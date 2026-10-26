pub struct FluidCube {
    pub size: usize,
    pub dt: f32,
    pub diff: f32,
    pub visc: f32,

    pub s: Vec<f32>, //previous density 
    pub density: Vec<f32>,

    pub vx: Vec<f32>,
    pub vy: Vec<f32>,
    pub vz: Vec<f32>,

    pub vx0: Vec<f32>, //previous velocities
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
    //index function takes three coordinates, and gives the corresponding array index in a one-dimensional array 
    //its kind of a hash table basically kind of not really 
    pub fn ix(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.size + z * self.size * self.size
    }
    pub fn adddensity(&mut self, x:usize, y:usize, z:usize, amount:f32) {
        let index = self.ix(x,y,z);
        self.density[index] += amount;
    }
    pub fn addvelocity(&mut self, x:usize, y:usize, z:usize, amounty:f32, amountx:f32, amountz:f32) {
        let index = self.ix(x,y,z);
        self.vx[index] += amountx;
        self.vy[index] += amounty;
        self.vz[index] += amountz;
    }

    //this is what it does everyframe
    pub fn step(&mut self) {
        let n = self.size;
        let dt = self.dt;
        let visc = self.visc;
        let diff = self.diff;

        diffuse(1, &mut self.vx0, &self.vx, visc, dt, 4, n);
        diffuse(2, &mut self.vy0, &self.vy, visc, dt, 4, n);
        diffuse(3, &mut self.vz0, &self.vz, visc, dt, 4, n);

        project(
            &mut self.vx0,
            &mut self.vy0,
            &mut self.vz0,
            &mut self.vx,
            &mut self.vy,
            4,
            n,
        );

        advect(1, &mut self.vx, &self.vx0, &self.vx0, &self.vy0, &self.vz0, dt, n);
        advect(2, &mut self.vy, &self.vy0, &self.vx0, &self.vy0, &self.vz0, dt, n);
        advect(3, &mut self.vz, &self.vz0, &self.vx0, &self.vy0, &self.vz0, dt, n);

        project(
            &mut self.vx,
            &mut self.vy,
            &mut self.vz,
            &mut self.vx0,
            &mut self.vy0,
            4,
            n,
        );

        diffuse(0, &mut self.s, &self.density, diff, dt, 4, n);

        advect(
            0,
            &mut self.density,
            &self.s,
            &self.vx,
            &self.vy,
            &self.vz,
            dt,
            n,
        );
    }
}


fn ix(x: usize, y: usize, z: usize, n: usize) -> usize {
    x + y * n + z * n * n
}
//this is gonna do some funky wall stuff, walls are essentially more cells that just have normal reactions.
//the b variable just tells which orientation the wall is so floor ceiling wall etc... (not that theres any more ways a wall can be in 3D)
fn set_bnd(b:i32,x:&mut [f32],n:usize) {
    //these loops sets the sides/faces
    for j in 1..n -1 {
        for i in 1..n -1 {  
            let nearfront = ix(i,j,1,n);
            let front = ix(i,j,0,n);

            x[front] = if b == 3 {
                -x[nearfront]
            } else {
                x[nearfront]
            };
            let near_back = ix(i, j, n-2, n);
            let back = ix(i,j,n-1,n);

            x[back] = if b == 3 {
                -x[near_back]
            } else {
                x[near_back]
            }
        }
    }
    for k in 1..n - 1 {
        for i in 1..n - 1 {
            let near_bottom = ix(i, 1, k, n);
            let bottom = ix(i, 0, k, n);

            x[bottom] = if b == 2 {
                -x[near_bottom]
            } else {
                x[near_bottom]
            };

            let near_top = ix(i, n - 2, k, n);
            let top = ix(i, n - 1, k, n);

            x[top] = if b == 2 {
                -x[near_top]
            } else {
                x[near_top]
            };
        }
    }
    for k in 1..n - 1 {
        for j in 1..n - 1 {
            let near_left = ix(1, j, k, n);
            let left = ix(0, j, k, n);

            x[left] = if b == 1 {
                -x[near_left]
            } else {
                x[near_left]
            };

            let near_right = ix(n - 2, j, k, n);
            let right = ix(n - 1, j, k, n);

            x[right] = if b == 1 {
                -x[near_right]
            } else {
                x[near_right]
            };
        }
    }
    //these are for the cornerssss
    x[ix(0, 0, 0, n)] = 0.33 * (
        x[ix(1, 0, 0, n)]
        + x[ix(0, 1, 0, n)]
        + x[ix(0, 0, 1, n)]
    );

    x[ix(0, n - 1, 0, n)] = 0.33 * (
        x[ix(1, n - 1, 0, n)]
        + x[ix(0, n - 2, 0, n)]
        + x[ix(0, n - 1, 1, n)]
    );

    x[ix(0, 0, n - 1, n)] = 0.33 * (
        x[ix(1, 0, n - 1, n)]
        + x[ix(0, 1, n - 1, n)]
        + x[ix(0, 0, n - 2, n)]
    );

    x[ix(0, n - 1, n - 1, n)] = 0.33 * (
        x[ix(1, n - 1, n - 1, n)]
        + x[ix(0, n - 2, n - 1, n)]
        + x[ix(0, n - 1, n - 2, n)]
    );

    x[ix(n - 1, 0, 0, n)] = 0.33 * (
        x[ix(n - 2, 0, 0, n)]
        + x[ix(n - 1, 1, 0, n)]
        + x[ix(n - 1, 0, 1, n)]
    );

    x[ix(n - 1, n - 1, 0, n)] = 0.33 * (
        x[ix(n - 2, n - 1, 0, n)]
        + x[ix(n - 1, n - 2, 0, n)]
        + x[ix(n - 1, n - 1, 1, n)]
    );

    x[ix(n - 1, 0, n - 1, n)] = 0.33 * (
        x[ix(n - 2, 0, n - 1, n)]
        + x[ix(n - 1, 1, n - 1, n)]
        + x[ix(n - 1, 0, n - 2, n)]
    );

    x[ix(n - 1, n - 1, n - 1, n)] = 0.33 * (
        x[ix(n - 2, n - 1, n - 1, n)]
        + x[ix(n - 1, n - 2, n - 1, n)]
        + x[ix(n - 1, n - 1, n - 2, n)]
    );

}

//black magic maths 
/*most of the maths I do not understand below here its taken from this paper by Jon Stam
https://www.dgp.toronto.edu/public_user/stam/reality/Research/pdf/GDC03.pdf
heres another explanation on how it works
Since the matrix of this system is sparse (i.e contain a lot of zeros), this system can be solved efficiently (note that there exist more accurate algorithms) 
using a Gauss-Seidel algorithm. at do we solve exactly ? For the diffusion, we try to find tthe densities which, when diffused backward in time, gives the density we 
started with. In the case of the projection, it is a little more complicated. We are solving a linear system called a poisson equation. In mathematics, Poisson's equation 
is a partial differential equation. To make sure that the fluid is incompressible, we need to satisfy the non-divergence condition. The divergence of a vector field 
(the velicity of the fluid)at a particular location is a scalar value that essentially represents how much the field is ‘expanding’ at the point. A positive value 
represents a divergence and a negative value represents a convergence or compression.If we use the assumption that a fluid is incompressible, then that implies that 
the divergence of the velocity must be zero everywhere. A useful splitting method for solving the incompressible flow equations is the so-called ‘projection method’. In 
this technique, one first computes a divergent vector field that results from the advection & viscosity terms, and then projects that entire vector field onto the space of 
non-divergent vector fields This can be thought of as creating an instantaneous pressure field whose gradient compensates for any divergence caused by the advection & 
viscosity. Getting back to the poisson equation, we create these beast to solve the unknown pressure field. The gradient of this pressure field is an additional force on 
the fluid and the divergence of this gradient compensates for the divergence resulting from the convection-viscosity equation. The result is a final velocity field that 
has no divergence.
*/
//this function solves linear equations but who knows how
fn lin_solve(b: i32, x: &mut [f32], x0: &[f32], a: f32, c: f32, iter: usize, n: usize,) {
    let c_recip = 1.0 / c;
    for _ in 0..iter {
        for m in 1..n - 1 {
            for j in 1..n - 1 {
                for i in 1..n - 1 {
                    let index = ix(i, j, m, n);
                    let neighbours =
                        x[ix(i + 1, j, m, n)]
                        + x[ix(i - 1, j, m, n)]
                        + x[ix(i, j + 1, m, n)]
                        + x[ix(i, j - 1, m, n)]
                        + x[ix(i, j, m + 1, n)]
                        + x[ix(i, j, m - 1, n)];
                    x[index] = (x0[index] + a * neighbours) * c_recip;
                }
            }
        }

        set_bnd(b, x, n);
    }
}

fn diffuse(b:i32, x:&mut [f32], x0: &[f32], diff:f32, dt:f32, iter:usize, n:usize) {
    let n_float =(n-2) as f32;
    let a = dt * diff * n_float * n_float;
    lin_solve(b, x, x0, a, 1.0 + 6.0 * a, iter, n);
}   
//this is for like letting stuff out the grid 
//p is a temp grid for pressure 
//div is a temp grid for divergence 
//so because its an incrompessable fluid the amount of fluid leaving must be as the same going in, it basically cleans up messy numbers
fn project(veloc_x: &mut [f32], veloc_y: &mut [f32], veloc_z: &mut [f32], p: &mut [f32], div: &mut [f32], iter: usize, n: usize,) {
    for k in 1..n - 1 {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let index = ix(i, j, k, n);
                div[index] = -0.5 * (
                    veloc_x[ix(i + 1, j, k, n)]
                    - veloc_x[ix(i - 1, j, k, n)]
                    + veloc_y[ix(i, j + 1, k, n)]
                    - veloc_y[ix(i, j - 1, k, n)]
                    + veloc_z[ix(i, j, k + 1, n)]
                    - veloc_z[ix(i, j, k - 1, n)]
                ) / n as f32;
                p[index] = 0.0;
            }
        }
    }
    //applies boundaries to the temp grids
    set_bnd(0, div, n);
    set_bnd(0, p, n);
    lin_solve(0, p, div, 1.0, 6.0, iter, n);

    for k in 1..n - 1 {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let index = ix(i, j, k, n);

                veloc_x[index] -= 0.5 * (
                    p[ix(i + 1, j, k, n)]
                    - p[ix(i - 1, j, k, n)]
                ) * n as f32;

                veloc_y[index] -= 0.5 * (
                    p[ix(i, j + 1, k, n)]
                    - p[ix(i, j - 1, k, n)]
                ) * n as f32;

                veloc_z[index] -= 0.5 * (
                    p[ix(i, j, k + 1, n)]
                    - p[ix(i, j, k - 1, n)]
                ) * n as f32;
            }
        }
    }
    set_bnd(1, veloc_x, n);
    set_bnd(2, veloc_y, n);
    set_bnd(3, veloc_z, n);
}

//make stuff move but not like diffusion 
fn advect(b: i32, d: &mut [f32], d0: &[f32], veloc_x: &[f32], veloc_y: &[f32], veloc_z: &[f32], dt: f32, n: usize,) {
    let n_float = n as f32;
    let dtx = dt * (n - 2) as f32;
    let dty = dt * (n - 2) as f32;
    let dtz = dt * (n - 2) as f32;
    for k in 1..n - 1 {
        for j in 1..n - 1 {
            for i in 1..n - 1 {
                let index = ix(i, j, k, n);
                let mut x = i as f32
                    - dtx * veloc_x[index];
                let mut y = j as f32
                    - dty * veloc_y[index];
                let mut z = k as f32
                    - dtz * veloc_z[index];

                x = x.clamp(0.5, n_float - 1.5);
                y = y.clamp(0.5, n_float - 1.5);
                z = z.clamp(0.5, n_float - 1.5);

                let i0 = x.floor() as usize;
                let i1 = i0 + 1;

                let j0 = y.floor() as usize;
                let j1 = j0 + 1;

                let k0 = z.floor() as usize;
                let k1 = k0 + 1;

                let s1 = x - i0 as f32;
                let s0 = 1.0 - s1;

                let t1 = y - j0 as f32;
                let t0 = 1.0 - t1;

                let u1 = z - k0 as f32;
                let u0 = 1.0 - u1;

                let c000 = d0[ix(i0, j0, k0, n)];
                let c001 = d0[ix(i0, j0, k1, n)];
                let c010 = d0[ix(i0, j1, k0, n)];
                let c011 = d0[ix(i0, j1, k1, n)];

                let c100 = d0[ix(i1, j0, k0, n)];
                let c101 = d0[ix(i1, j0, k1, n)];
                let c110 = d0[ix(i1, j1, k0, n)];
                let c111 = d0[ix(i1, j1, k1, n)];

                d[index] =
                    s0 * (
                        t0 * (u0 * c000 + u1 * c001)
                        + t1 * (u0 * c010 + u1 * c011)
                    )
                    + s1 * (
                        t0 * (u0 * c100 + u1 * c101)
                        + t1 * (u0 * c110 + u1 * c111)
                    );
            }
        }
    }

    set_bnd(b, d, n);
}

