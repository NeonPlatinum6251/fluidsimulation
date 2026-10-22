
use macroquad::prelude::*;

#[macroquad::main("Fluid Simulation")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await;
    }
}
/*TODO 
make macroquad display the cube somehow 
God only knows how ill show a 3d Cube <--- in the tune of the beach boys
make the main simulation loop
a diffuse function
advenction function
project function 
set bounds function
steal someone elses code to do all the wacky maths for linear differential equtions
*/