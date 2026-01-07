use macroquad::{prelude::*, rand::gen_range};
use std::f32::consts::PI;
use macroquad::ui;

#[derive(Clone)]
struct TreeParams {
    iteration : i32,
    total_iterations : i32,
    angle : f32,
    ratio : f32,
    offset : f32,
    pos : Vec2,
    scale : f32,
    color : Color,
    disco_mode : bool
}

fn draw_tree_helper(mut params : TreeParams){
    if params.iteration == params.total_iterations{
        return;
    }
    let scale_factor = params.ratio.powf(params.iteration as f32) * params.scale;
    
    let x2= params.offset.cos() * scale_factor + params.pos.x;
    let y2 = -params.offset.sin() * scale_factor + params.pos.y;

    draw_line(params.pos.x, params.pos.y,  x2, y2, 2.5*params.ratio.powf(params.iteration as f32) + 0.75, params.color);
    params.iteration += 1;
    params.pos = Vec2 { x: x2, y: y2 };
    params.offset += params.angle;

    if params.disco_mode{
        let random_color : Color = Color{r : gen_range(0.0, 1.0),g : gen_range(0.0, 1.0), b: gen_range(0.0, 1.0), a: 1.0};
        params.color = random_color;
    }

    draw_tree_helper(params.clone());
    params.offset -= 2.0 * params.angle;
    draw_tree_helper(params.clone());
}

pub fn draw_tree(total_iterations : i32, angle : f32, ratio : f32, pos : Vec2, scale : f32, diso_mode : bool){
    // draw_line(0.0, screen_height()/2.0, screen_width(), screen_height()/2.0, 3.0, BLUE);
    // draw_line(screen_width() / 2.0, 0.0, screen_width() / 2.0, screen_height(), 3.0, RED);
    let params = TreeParams {
        iteration : 0,
        total_iterations : total_iterations,
        angle : angle,
        ratio : ratio,
        offset : PI / 2.0,
        pos : pos,
        scale : scale,
        color : GREEN,
        disco_mode : diso_mode
    };

    draw_tree_helper(params.clone());

}

pub fn mouse_controls(pos : &mut Vec2, start : &mut Vec2, old_pos : &mut Vec2, scale: &mut f32){
        if is_mouse_button_pressed(MouseButton::Left){
            let m = mouse_position();
            start.x = m.0;
            start.y = m.1;

            *old_pos = *pos;
        }
        else if is_mouse_button_down(MouseButton::Left) && !ui::root_ui().is_mouse_captured(){ 
            let m = mouse_position();
            let delta_x = m.0 - start.x;
            let delta_y = m.1 - start.y;
            
            pos.x = old_pos.x + delta_x;
            pos.y = old_pos.y + delta_y;
        }

        let wheel = mouse_wheel().1;
        if wheel != 0.0{
            *scale += wheel;
        }
        

}