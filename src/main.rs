
mod fluid;
use fluid::FluidCube;
use macroquad::prelude::*;

enum SimulationType {
    Rain,
    Draw,
    Pour,
}

#[macroquad::main("Fluid Simulation")]
async fn main() {
    let mut fluid = FluidCube::new(32,0.05 ,0.0001 ,0.0 );
    fluid.AddDensity(16, 16, 16, 1.0);
    fluid.AddVelocity(16, 16, 16, 0.0, 1.0, 0.0);

    loop {
        let n =fluid.size;
        let tempx = macroquad::rand::gen_range(1, n - 1);
        let tempy = macroquad::rand::gen_range(1, n - 1);

        let velocity_x = macroquad::rand::gen_range(-1.0, 1.0);
        let velocity_y = macroquad::rand::gen_range(-1.0, 1.0);
        let velocity_z = macroquad::rand::gen_range(-1.0, 1.0);

        fluid.AddDensity(tempx, tempy, 16, 1.0);
        fluid.AddVelocity(tempx, tempy, 16, velocity_y, velocity_x, velocity_z,);
        clear_background(BLACK);
        fluid.step();
        let cell_size = 10.0;
        let grid_width = n as f32 * cell_size;
        let grid_height = n as f32 * cell_size;

        let offset_x = (screen_width() - grid_width) / 2.0;
        let offset_y = (screen_height() - grid_height) / 2.0;

        for y in 0..n {
            for x in 0..n {
                let index = fluid.ix(x, y, 16);
                let density = fluid.density[index].clamp(0.0, 1.0);

                let colour = Color::new(
                    density * 0.9,
                    density * 0.2,
                    density,
                    1.0,
                );

                draw_rectangle(
                    offset_x + x as f32 * cell_size,
                    offset_y + y as f32 * cell_size,
                    cell_size - 1.0,
                    cell_size - 1.0,
                    colour,
                );
            }
        }
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await;
    }
}

/*TODO 
make macroquad display the cube somehow 
God only knows how ill show a 3d Cube <--- in the tune of the beach boys
make an enum for the different types of simulation e.g. rain, draw etc..
*/