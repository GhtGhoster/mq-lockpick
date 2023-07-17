
use macroquad::prelude::*;

#[macroquad::main("mq-lockpick")]
async fn main() {
    // debugging
    // std::env::set_var("RUST_BACKTRACE", "1");

    // problems with mq in general (later add to template)
    //  circles are 20 sided polygons (at least upscale the sides with how many pixels they take up)
    //  no support for arbitrary polygon
    //  on the fly sound synthesis
    //  over the top - rendering vector art
    //  unrelated - add self-removal to the rename.ps1 script

    let pixel_side_ratios = vec![
        (0.0, 0.0),
        (50.0, 30.0),
        (100.0, 40.0),
        (200.0, 57.0),
        (300.0, 64.0),
        (400.0, 70.0),
        (500.0, 90.0),
    ];

    let mut show_circle: bool = false;
    let mut radius_power: f32 = 0.4;
    let mut radius: f32 = screen_height().min(screen_width())/2.0 - 20.0;
    let mut sides: u8 = (std::f32::consts::PI / (radius.powf(radius_power)/radius).asin()) as u8;
    let mut sides_min: u8 = 10;

    loop {
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .show(egui_ctx, |ui| {
                    ui.spacing_mut().slider_width = screen_width() - 100.0;
                    ui.checkbox(&mut show_circle, "Show default circle");
                    if ui.add(egui::Slider::new(&mut radius, 0.0..=screen_width().min(screen_height())/2.0)).changed() {
                        sides = (std::f32::consts::PI / (radius.powf(radius_power)/radius).asin()) as u8;
                    }
                    if ui.add(egui::Slider::new(&mut radius_power, 0.1..=1.0)).changed() {
                        sides = (std::f32::consts::PI / (radius.powf(radius_power)/radius).asin()) as u8;
                    }
                    ui.add(egui::Slider::new(&mut sides, u8::MIN..=u8::MAX));
                    if ui.add(egui::Slider::new(&mut sides_min, 0..=20)).changed() {
                        sides = (std::f32::consts::PI / (radius.powf(radius_power)/radius).asin()) as u8;
                    };
                }
            );
        });

        // rendering
        clear_background(BLACK);

        // graph
        let factor_x = screen_width() / pixel_side_ratios[pixel_side_ratios.len()-1].0;
        let factor_y = screen_height() / pixel_side_ratios[pixel_side_ratios.len()-1].1;
        for i in 0..pixel_side_ratios.len()-1 {
            let x = pixel_side_ratios[i].0 * factor_x;
            let y = screen_height() - (pixel_side_ratios[i].1 * factor_y);
            draw_text(
                format!("{}, {}", pixel_side_ratios[i].0, pixel_side_ratios[i].1).as_str(),
                x,
                y,
                20.0,
                RED,
            );
            draw_circle(x, y, 5.0, RED);
            draw_line(
                x,
                y,
                pixel_side_ratios[i+1].0 * factor_x,
                screen_height() - (pixel_side_ratios[i+1].1 * factor_y),
                1.0,
                RED,
            );
        }

        // cirlce
        if radius > 0.0 {
            draw_poly(screen_width()/2.0, screen_height()/2.0, sides.max(sides_min), radius, 0.0, Color::from_rgba(255, 255, 0, 127));
            if show_circle {
                draw_circle(screen_width()/2.0, screen_height()/2.0, radius, Color::from_rgba(255, 0, 255, 127));
            }
        }

        egui_macroquad::draw();

        // next frame
        next_frame().await
    }
}
