/* 

#[macroquad::main("Fluid Simulation")]
async fn main() {
    let mut fluid = FluidCube::new(32,0.01 ,0.0001 ,0.0 );
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
*/
/*TODO 
make a readme with all my sources 
goated ----> https://www.youtube.com/watch?v=iKAVRgIrUOU
*/
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
    let mut fluid = FluidCube::new(32,0.01 ,0.0001 ,0.0 );
    loop {
        let n = fluid.size;

        let tempx = macroquad::rand::gen_range(1, n - 1);
        let tempy = macroquad::rand::gen_range(1, n - 1);
        let tempz = macroquad::rand::gen_range(1, n - 1);
        let velocity_x = macroquad::rand::gen_range(-1.0, 5.0);
        let velocity_y = macroquad::rand::gen_range(-1.0, 5.0);
        let velocity_z = macroquad::rand::gen_range(-1.0, 5.0);

        //fluid.AddDensity(tempx, tempy, tempz, 1.0);
        fluid.AddDensity(16, 16, 16, 1.0);
        fluid.AddVelocity(16, 16, 16, velocity_y, velocity_x, velocity_z,);
     
        clear_background(BLACK);
        fluid.step();

        set_camera(&Camera3D {
            position: vec3(40.0, 35.0, 40.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });
        for z in 0..n {
            for y in 0..n {
                for x in 0..n {
                    let offset = n as f32 / 2.0;
                    let index = fluid.ix(x, y, z);
                    let density = fluid.density[index];
                    let position = vec3(x as f32 -offset,y as f32-offset, z as f32-offset);
                    if density > 0.01 {
                        let brightness = density.clamp(0.0, 1.0);
                        let colour = Color::new(brightness * 0.9, brightness * 0.2, brightness, brightness,);
                        draw_cube(position,vec3(0.4,0.4,0.4) ,None , colour);
                    }
                }
            }
        }
        set_default_camera();
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await;
    }
}
