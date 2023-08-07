#![allow(unused_parens)]

use egui::Slider;
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

    // fn draw_better_circle(x: f32, y: f32, radius: f32, color: Color) {
    //     let sides: u8 = (std::f32::consts::PI / (radius.powf(0.4)/radius).asin()) as u8;
    //     draw_poly(x, y, sides, radius, 0.0, color);
    // }

    let mut pin_amount: u8 = 6;
    let mut bitting: Vec<u8> = generate_bitting(pin_amount);

    loop {
        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Pin amount:");
                        if ui.add(Slider::new(&mut pin_amount, 1u8..=10u8)).changed() {
                            bitting = generate_bitting(pin_amount);
                        };
                    });
                    ui.horizontal(|ui| {
                        ui.label("Bitting:");
                        let string = bitting
                            .as_slice()
                            .into_iter()
                            .map(|number| number.to_string())
                            .collect::<Vec<String>>()
                            .join(", ");
                        ui.code(format!("[{string}]"));
                    });
                    if ui.button("Reroll bitting").clicked() {
                        bitting = generate_bitting(pin_amount);
                    }
                }
            );
        });

        // rendering
        clear_background(BLACK);

        // values useful for rendering
        let pin_width = screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0);
        let pin_height = screen_height()/6.0;

        let plug_width = ((pin_amount as f32 * 2.0) + 1.0) * pin_width;
        let plug_height = pin_height * 2.0;

        let body_width = ((pin_amount as f32 * 2.0) + 2.0) * pin_width;
        let body_height = ((pin_height*2.0) + pin_width) * 2.0;

        // body
        draw_rectangle(
            (screen_width()/2.0) - body_width,
            (screen_height()/2.0) - body_height / 2.0,
            body_width,
            body_height,
            WHITE,
        );

        // plug cavity
        draw_rectangle(
            (screen_width()/2.0) - plug_width - (pin_width/8.0),
            (screen_height()/2.0) - (pin_width/8.0),
            plug_width + (pin_width/8.0),
            plug_height + (pin_width/4.0),
            DARKGRAY,
        );

        // plug
        draw_rectangle(
            (screen_width()/2.0) - plug_width,
            (screen_height()/2.0),
            plug_width,
            plug_height,
            WHITE,
        );

        // keyway
        draw_rectangle(
            (screen_width()/2.0) - pin_amount as f32 * 2.0 * pin_width,
            (screen_height()/2.0) + pin_height,
            pin_amount as f32 * 2.0 * pin_width,
            pin_height,
            DARKGRAY,
        );

        // pins
        for i in 1..=pin_amount {
            // plug cavity
            draw_rectangle(
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width),
                (screen_height()/2.0),
                pin_width,
                pin_height,
                DARKGRAY,
            );

            // body cavity
            draw_rectangle(
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width),
                (screen_height()/2.0) - (pin_height*1.5),
                pin_width,
                pin_height + pin_height*1.5,
                DARKGRAY,
            );
        }

        egui_macroquad::draw();

        // next frame
        next_frame().await
    }
}

fn generate_bitting(pin_amount: u8) -> Vec<u8> {
    let mut bitting = Vec::with_capacity(pin_amount as usize);
    for _ in 0..pin_amount {
        bitting.push(
            rand::gen_range(1u8, 10u8)
        );
    }
    bitting
}
