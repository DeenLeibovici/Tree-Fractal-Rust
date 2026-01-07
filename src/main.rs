use macroquad::prelude::*;
use macroquad::ui;
use macroquad::ui::Skin;
use std::f32::consts::PI;
mod fractal;

fn window_conf() -> Conf {
    Conf {
        window_title: "Fractal".to_string(),
        window_width: 1000,
        window_height: 1000,
        sample_count: 16,
        ..Default::default()
    }
}
fn ui_skin() -> Skin {
    let label_style_built  = ui::root_ui()
                                    .style_builder()
                                    .text_color(WHITE)
                                    .build();

    Skin {label_style: label_style_built, 
        ..ui::root_ui().default_skin() }
}

fn immediate_ui(skin: &Skin, angle : &mut f32, ratio : &mut f32, iterations : &mut f32, disco_toggle : &mut bool){
        ui::root_ui().push_skin(&skin);
                ui::root_ui().label(None, "Angle(rad)");
                ui::root_ui().slider(1, "[-10 .. 10]", -10f32..10f32, angle);

                ui::root_ui().label(None, "Ratio");
                ui::root_ui().slider(2, "[-10 .. 10]", -10f32..10f32, ratio);

                ui::root_ui().label(None, "Iterations");
                ui::root_ui().slider(3, "[0 .. 20]", 0f32..20f32, iterations);

                if ui::root_ui().button(None, if *disco_toggle { "[x] Disable Disco Mode" } else { "[ ] Enable Disco Mode" }) {
                    *disco_toggle = !*disco_toggle;
                }
                ui::root_ui().pop_skin();
                *iterations = iterations.round();
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut angle : f32 = (2.0*PI)/11.0;
    let mut ratio : f32 = 0.75;
    let mut scale : f32 = 100.0;
    let mut iterations : f32 = 10.0;

    let mut pos : Vec2 = Vec2 { x: screen_width() / 2.0, y: screen_height() / 2.0 + 100.0};
    let mut start : Vec2 =  Vec2::ZERO;
    let mut old_pos : Vec2 =  Vec2::ZERO;
    let mut disco_toggle : bool = false;
    let skin = ui_skin();
    loop {
        fractal::mouse_controls(&mut pos, &mut start, &mut old_pos, &mut scale);
        
        immediate_ui(&skin, &mut angle, &mut ratio, &mut iterations, &mut disco_toggle);

        fractal::draw_tree(iterations as i32, angle, ratio, pos, scale, disco_toggle);
    
        // draw_fps();
        next_frame().await;
    }
}
