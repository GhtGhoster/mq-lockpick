#![allow(unused_parens)]

use ::rand::thread_rng;
use ::rand::seq::SliceRandom;
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
    // more robust rendering and collision-checking infrastructure in general required
    //  easier on the math: length, angle, etc. of a vector
    //  point in shape, shape overlaps shape, ...

    // fn draw_better_circle(x: f32, y: f32, radius: f32, color: Color) {
    //     let sides: u8 = (std::f32::consts::PI / (radius.powf(0.4)/radius).asin()) as u8;
    //     draw_poly(x, y, sides, radius, 0.0, color);
    // }

    // TODO:
    //  move lock innards and relevant hitboxes higher up
    //  add allow-boxes for pin holes/cavities
    //  make global lockpick thickness
    //  make tensioning tool as thick as lockpick
    //  disable starting position or prebake a better one

    let mut pin_amount: u8 = 6;
    let mut bitting: Vec<u8> = generate_bitting(pin_amount);
    let mut tension_values: Vec<u8> = generate_tension_values(pin_amount); // TODO


    // let mut mouse_box_min: (f32, f32);// = (screen_width()/2.0, screen_height()/2.0);
    // let mut mouse_box_max: (f32, f32);// = (screen_width(), screen_height());
    let mut mouse_box: ((f32, f32), (f32, f32));
    let mut lockpick_box: ((f32, f32), (f32, f32));

    let mut last_mouse_x: f32 = screen_width();
    let mut last_mouse_y: f32 = (screen_height()/2.0) + ((screen_height()/6.0)*1.5);

    let mut last_lockpick_x: f32 = (screen_height()/2.0) + ((screen_height()/6.0)*1.5);
    let mut last_lockpick_y: f32 = (screen_width()/2.0);

    let mut last_lockpick_tip_x: f32 = 0.0;
    let mut last_lockpick_tip_y: f32 = 0.0;

    let mut lockpick_length: f32;// = screen_width()/2.0;

    loop {
        // update screen-relative global values
        // mouse_box_min = (screen_width()/2.0, screen_height()/2.0);
        // mouse_box_max = (screen_width(), screen_height());
        mouse_box = (
            (
                screen_width()/2.0,
                screen_height()/2.0
            ), (
                screen_width(),
                screen_height()
            ),
        );
        lockpick_length = screen_width()/2.0;

        // values useful for rendering and logic
        let pin_width = screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0);
        let pin_height = screen_height()/6.0;

        let plug_width = ((pin_amount as f32 * 2.0) + 1.0) * pin_width;
        let plug_height = pin_height * 2.0;

        let body_width = ((pin_amount as f32 * 2.0) + 2.0) * pin_width;
        let body_height = ((pin_height*2.0) + pin_width) * 2.0;

        let keyway_y: f32 = (screen_height()/2.0) + (pin_height*1.5);
        let keyway_x: f32 = (screen_width()/2.0);

        lockpick_box = (
            (
                (screen_width()/2.0) - (pin_amount as f32 * 2.0 * pin_width) - (pin_width / 8.0),
                (screen_height()/2.0) + pin_height,
            ), (
                (screen_width()/2.0) - (pin_amount as f32 * 2.0 * pin_width) - (pin_width / 8.0) + ((pin_amount as f32 * 2.0 * pin_width) + (pin_width / 8.0)),
                (screen_height()/2.0) + pin_height + pin_height,
            ),
        );

        // game logic
        let (mouse_x, mouse_y) = mouse_position();
        if point_in_box(mouse_box, (mouse_x, mouse_y)) {
            //mouse_keyway_vector = (keyway_x - mouse_x, keyway_y - mouse_y);
            let ((lockpick_x, lockpick_y), (lockpick_tip_x, lockpick_tip_y)) = calculate_lockpick_points(
                lockpick_length,
                keyway_x,
                keyway_y,
                mouse_x,
                mouse_y,
            );
            if 
                (
                    point_in_box(lockpick_box, (lockpick_x, lockpick_y)) ||
                    point_in_box(mouse_box, (lockpick_x, lockpick_y))
                ) && (
                    point_in_box(lockpick_box, (lockpick_tip_x, lockpick_tip_y)) ||
                    point_in_box(mouse_box, (lockpick_tip_x, lockpick_tip_y))
                )
            {
                last_mouse_x = mouse_x;
                last_mouse_y = mouse_y;
                last_lockpick_x = lockpick_x;
                last_lockpick_y = lockpick_y;
                last_lockpick_tip_x = lockpick_tip_x;
                last_lockpick_tip_y = lockpick_tip_y;
            }
        }


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
                        ui.code(vec_to_string(&bitting));
                    });
                    ui.horizontal(|ui| {
                        ui.label("Tension values:");
                        ui.code(vec_to_string(&tension_values));
                    });
                    if ui.button("Reroll bitting").clicked() {
                        bitting = generate_bitting(pin_amount);
                        tension_values = generate_tension_values(pin_amount);
                    }
                }
            );
        });

        // rendering
        clear_background(BLACK);

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
            (screen_width()/2.0) - (pin_amount as f32 * 2.0 * pin_width) - (pin_width / 8.0),
            (screen_height()/2.0) + pin_height,
            (pin_amount as f32 * 2.0 * pin_width) + (pin_width / 8.0),
            pin_height,
            DARKGRAY,
        );

        // pins
        for i in 1..=pin_amount {
            // plug pin cavity
            draw_rectangle(
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width) - (pin_width / 8.0),
                (screen_height()/2.0),
                pin_width + (pin_width / 4.0),
                pin_height,
                DARKGRAY,
            );

            // body pin cavity
            draw_rectangle(
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width) - (pin_width / 8.0),
                (screen_height()/2.0) - (pin_height*1.5),
                pin_width + (pin_width / 4.0),
                pin_height + pin_height*1.5,
                DARKGRAY,
            );

            // key + driver pin
            draw_key_driver_pins(
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width),
                (screen_height()/2.0) + pin_height * 1.5,
                pin_width,
                pin_height,
                bitting[i as usize - 1],
            );
        }

        // tension tool
        draw_rectangle(
            (screen_width()/2.0) - pin_width * 2.0,
            (screen_height()/2.0) + plug_height - pin_width,
            pin_width * 3.0,
            pin_width,
            LIGHTGRAY,
        );
        draw_rectangle(
            (screen_width()/2.0) + pin_width / 8.0,
            (screen_height()/2.0) + plug_height - pin_width,
            pin_width,
            screen_height(), // infinite
            LIGHTGRAY,
        );

        // lockpick
        draw_line(
            last_mouse_x,
            last_mouse_y,
            last_lockpick_x,
            last_lockpick_y,
            pin_width/4.0,
            LIGHTGRAY,
        );
        draw_circle(
            last_lockpick_x,
            last_lockpick_y,
            pin_width/8.0,
            LIGHTGRAY,
        );
        draw_line(
            last_lockpick_x,
            last_lockpick_y,
            last_lockpick_tip_x,
            last_lockpick_tip_y,
            pin_width/4.0,
            LIGHTGRAY,
        );

        // debug TODO
        // draw_circle(keyway_x, keyway_y, 5.0, BLUE);
        // draw_circle(last_lockpick_x, last_lockpick_y, 5.0, RED);
        // draw_rectangle_lines(mouse_box.0.0, mouse_box.0.1, mouse_box.1.0 - mouse_box.0.0, mouse_box.1.1 - mouse_box.0.1, 1.0, BLUE);
        // draw_rectangle_lines(lockpick_box.0.0, lockpick_box.0.1, lockpick_box.1.0 - lockpick_box.0.0, lockpick_box.1.1 - lockpick_box.0.1, 1.0, RED);

        egui_macroquad::draw();

        // next frame
        next_frame().await
    }
}

fn vec_to_string<T: std::fmt::Display>(vector: &Vec<T>) -> String {
    let string = vector
        .as_slice()
        .into_iter()
        .map(|number| number.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{string}]")
}

fn point_in_box(hitbox: ((f32, f32), (f32, f32)), point: (f32, f32)) -> bool {
    point.0 > hitbox.0.0 && point.1 > hitbox.0.1 && point.0 < hitbox.1.0 && point.1 < hitbox.1.1
}

fn generate_bitting(pin_amount: u8) -> Vec<u8> {
    let mut bitting = Vec::with_capacity(pin_amount as usize);
    for _ in 0..pin_amount {
        bitting.push(
            rand::gen_range(0u8, 10u8)
        );
    }
    bitting
}

fn generate_tension_values(pin_amount: u8) -> Vec<u8> {
    let mut tension_values: Vec<u8> = (0..pin_amount).collect();
    tension_values.shuffle(&mut thread_rng());
    tension_values
}

fn calculate_lockpick_points(lockpick_length: f32, keyway_x: f32, keyway_y: f32, mouse_x: f32, mouse_y: f32) -> ((f32, f32), (f32, f32)) {
    let mouse_keyway_vector_x = keyway_x - mouse_x;
    let mouse_keyway_vector_y = keyway_y - mouse_y;
    let vector_len = (mouse_keyway_vector_x.powi(2) + mouse_keyway_vector_y.powi(2)).sqrt();
    let unit_vector_x = mouse_keyway_vector_x / vector_len;
    let unit_vector_y = mouse_keyway_vector_y / vector_len;
    let lockpick_x = mouse_x + (unit_vector_x * lockpick_length);
    let lockpick_y = mouse_y + (unit_vector_y * lockpick_length);
    let lockpick_tip_x = lockpick_x - (unit_vector_y * (lockpick_length / 16.0));
    let lockpick_tip_y = lockpick_y + (unit_vector_x * (lockpick_length / 16.0));
    ((lockpick_x, lockpick_y), (lockpick_tip_x, lockpick_tip_y))
}

fn draw_spring(x: f32, y: f32, w: f32, h:f32, coils: u8, thickness: f32, color: Color) {
    let coil_height: f32 = h / coils as f32;
    for i in (0..coils).step_by(2) {
        draw_line(
            x,
            y + (i as f32 * coil_height),
            x + w,
            y + ((i + 1) as f32 * coil_height),
            thickness,
            color,
        );
    }
    for i in (1..coils).step_by(2) {
        draw_line(
            x,
            y + ((i + 1) as f32 * coil_height),
            x + w,
            y + (i as f32 * coil_height),
            thickness,
            color,
        );
    }
}

fn draw_key_driver_pins(x: f32, low_y: f32, w: f32, driver_h:f32, pin_bitting: u8) {
    let key_pin_height = (driver_h * 0.5) + (driver_h * 0.05 * pin_bitting as f32);
    
    // spring
    draw_spring(
        x,
        (screen_height()/2.0) - (driver_h*1.5),
        w,
        (low_y - (w/2.0) - key_pin_height - (w/8.0) - driver_h) - ((screen_height()/2.0) - (driver_h*1.5)),
        9,
        w/8.0,
        LIGHTGRAY,
    );

    // driver pin
    draw_rectangle(
        x,
        low_y - (w/2.0) - key_pin_height - (w/8.0) - driver_h,
        w,
        driver_h,
        LIGHTGRAY,
    );

    // key pin
    draw_rectangle(
        x,
        low_y - (w/2.0) - key_pin_height,
        w,
        key_pin_height,
        LIGHTGRAY,
    );
    draw_triangle(
        Vec2{x: x           , y: low_y - (w/2.0)},
        Vec2{x: x + w       , y: low_y - (w/2.0)},
        Vec2{x: x + (w/2.0) , y: low_y},
        LIGHTGRAY,
    );
}
