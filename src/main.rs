
mod fluid;

use fluid::FluidCube;
use macroquad::prelude::*;

#[macroquad::main("Fluid Simulation")]
async fn main() {
    loop {
        fluid.step();
        clear_background(BLACK);
        next_frame().await;
    }
}

/*TODO 
make macroquad display the cube somehow 
God only knows how ill show a 3d Cube <--- in the tune of the beach boys
make the main simulation loop
*/