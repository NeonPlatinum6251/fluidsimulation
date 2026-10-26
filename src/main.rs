/* 
mod fluid;
use fluid::FluidCube;
use macroquad::prelude::*;
#[macroquad::main("Fluid Simulation")]
async fn main() {
    let mut fluid = FluidCube::new(32,0.01 ,0.0001 ,0.0 );
    fluid.adddensity(16, 16, 16, 1.0);
    fluid.addvelocity(16, 16, 16, 0.0, 1.0, 0.0);

    loop {
        let n =fluid.size;
        let tempx = macroquad::rand::gen_range(1, n - 1);
        let tempy = macroquad::rand::gen_range(1, n - 1);
        let velocity_x = macroquad::rand::gen_range(-1.0, 1.0);
        let velocity_y = macroquad::rand::gen_range(-1.0, 1.0);
        let velocity_z = macroquad::rand::gen_range(-1.0, 1.0);

        fluid.adddensity(tempx, tempy, 16, 1.0);
        fluid.addvelocity(tempx, tempy, 16, velocity_y, velocity_x, velocity_z,);
        clear_background(BLACK);
        fluid.step();
        let cell_size = 16.0;
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

fn update_camera( yaw:&mut f32, pitch:&mut f32) {
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_delta = mouse_delta_position();
        *yaw -= mouse_delta.x * 3.0;
        *pitch += mouse_delta.y * 3.0;
        let max_pitch = std::f32::consts::FRAC_PI_2 - 0.01;
        *pitch = pitch.clamp(-max_pitch, max_pitch);      
    }
}

fn to_camera(yaw:f32,pitch:f32) -> Camera3D {
    let x = 55.0 * &yaw.sin() * pitch.cos();
    let y = 55.0 * &pitch.sin();
    let z = 55.0 * yaw.cos() * pitch.cos();

    Camera3D {
        position:vec3(x,y,z),
        target: vec3(0.0,0.0,0.0),
        up: vec3(0.0, 1.0, 0.0), 
        fovy: 45.0,
        ..Default::default()
    }
}

#[macroquad::main("Fluid Simulation")]
async fn main() {
    let mut fluid = FluidCube::new(32,0.01 ,0.000001 ,0.0 );
    let mut yaw = 0.0;
    let mut pitch = 0.5;
    loop {
        let n = fluid.size;
        let tempx = macroquad::rand::gen_range(1, n - 1);
        let tempy = macroquad::rand::gen_range(1, n - 1);
        let tempz = macroquad::rand::gen_range(1, n - 1);
        let velocity_x = macroquad::rand::gen_range(-1.0, 5.0);
        let velocity_y = macroquad::rand::gen_range(-1.0, 5.0);
        let velocity_z = macroquad::rand::gen_range(-1.0, 5.0);

        //fluid.adddensity(tempx, tempy, tempz, 1.0);
        //fluid.addvelocity(tempx, tempy, tempz, velocity_x, velocity_y, velocity_z);
        fluid.adddensity(16, 16, 16, 1.0);
        fluid.addvelocity(16, 16, 16, velocity_y, velocity_x, velocity_z,);
     
        clear_background(BLACK);
        fluid.step();
        //orbital camera 
        update_camera(&mut yaw, &mut pitch);
        set_camera(&to_camera(yaw,pitch));
        /*  this ones a fixed camera
        set_camera(&Camera3D {
            position: vec3(40.0, 35.0, 40.0),
            target: vec3(0.0, 0.0, 0.0),
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        });
    */
        for z in 0..n {
            for y in 0..n {
                for x in 0..n {
                    let offset = n as f32 / 2.0;
                    let index = fluid.ix(x, y, z);
                    let density = fluid.density[index];
                    let position = vec3(x as f32 -offset,y as f32-offset, z as f32-offset);
                    if density > 0.05 {
                        let brightness = density.clamp(0.0, 1.0);
                        let colour = Color::new(brightness * 0.1, brightness * 0.9, brightness*0.2, brightness*255.0,);
                        draw_cube(position,vec3(1.0,1.0,1.0) ,None , colour);
                    }
                }
            }
        }
        draw_cube_wires(vec3(0.0,0.0,0.0), vec3(30.0,30.0,30.0), WHITE);
        set_default_camera();
        if is_key_pressed(KeyCode::Escape) {
            std::process::exit(0);
        }
        next_frame().await;
    }
}
