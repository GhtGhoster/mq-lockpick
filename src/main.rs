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
    //  point in shape, shape overlaps shape, ... (yea I realize I could've used a crate, shut up)
    //  distance from (SDFs)

    // fn draw_better_circle(x: f32, y: f32, radius: f32, color: Color) {
    //     let sides: u8 = (std::f32::consts::PI / (radius.powf(0.4)/radius).asin()) as u8;
    //     draw_poly(x, y, sides, radius, 0.0, color);
    // }

    // TODO:
    //  add key pin triangle hitboxes to the anti-rake
    //  add timer
    //  add rotating tension indicator
    //  add SFX

    // WODO:
    //  add closest available position extrapolation for smooth operation
    //      this would require major physics engine things and rubberbanding of
    //      the actual object position to the supposed position. nuh-uh.

    let mut debug: bool = false;
    let mut rake: bool = false;

    let mut pin_amount: u8 = 6;
    let mut bitting: Vec<u8> = generate_bitting(pin_amount);
    let mut tension_values: Vec<u8> = generate_tension_values(pin_amount);
    let mut pin_boxes: Vec<((f32, f32), (f32, f32))>;
    let mut current_tension: u8 = 0;

    let mut mouse_box: ((f32, f32), (f32, f32));
    let mut keyway_box: ((f32, f32), (f32, f32));


    // do not be deceived, weary reader, for despite the nomenclature, the variables that follow denominate current state.
    let mut last_mouse_x: f32 = screen_width() - (screen_height()/6.0);
    //yea idk, not worth my time to optimize
    let mut last_mouse_y: f32 =
        (screen_height()/2.0) + (screen_height()/6.0) -
        (((((screen_height()/6.0)*2.0) + (screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0))) * 2.0)/2.0) +
        ((screen_height()/6.0)*2.0) + ((screen_height()/6.0)/2.0);

    let mut last_lockpick_x: f32 = (screen_width()/2.0) - (screen_height()/6.0);
    let mut last_lockpick_y: f32 =
        (screen_height()/2.0) + (screen_height()/6.0) -
        (((((screen_height()/6.0)*2.0) + (screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0))) * 2.0)/2.0) +
        ((screen_height()/6.0)*2.0) + ((screen_height()/6.0)/2.0);

    let mut last_lockpick_tip_x: f32 = (screen_width()/2.0) - (screen_height()/6.0);
    let mut last_lockpick_tip_y: f32 =
        (screen_height()/2.0) + (screen_height()/6.0) -
        (((((screen_height()/6.0)*2.0) + (screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0))) * 2.0)/2.0) +
        ((screen_height()/6.0)*2.0) + ((screen_height()/6.0)/2.0) - (screen_width()/32.0);

    let mut lockpick_length: f32;// = screen_width()/2.0;
    let mut lockpick_width: f32;

    loop {
        // values useful for rendering and logic
        let pin_width = screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0);
        let pin_height = screen_height()/6.0;

        let plug_width = ((pin_amount as f32 * 2.0) + 1.0) * pin_width;
        let plug_height = pin_height * 2.0;

        let body_width = ((pin_amount as f32 * 2.0) + 2.0) * pin_width;
        let body_height = ((pin_height*2.0) + pin_width) * 2.0;

        let keyway_x: f32 = (screen_width()/2.0) + ((mouse_position().0 - (screen_width()/2.0)) / (screen_width()/2.0)) * pin_height;
        let keyway_y: f32 = (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0);//(screen_height()/2.0) + (pin_height*1.5);

        pin_boxes = generate_pin_boxes(pin_amount, pin_width, pin_height, body_height);
        lockpick_width = pin_width*0.75;

        // update screen-relative global values
        lockpick_length = screen_width()/2.0;
        mouse_box = (
            (
                screen_width()/2.0,
                0.0,
            ), (
                screen_width(),
                screen_height(),
            ),
        );
        keyway_box = (
            (
                (screen_width()/2.0) - (pin_amount as f32 * 2.0 * pin_width),// - (pin_width / 8.0),
                (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0),
            ), (
                (screen_width()/2.0) - (pin_amount as f32 * 2.0 * pin_width) - (pin_width / 8.0) + ((pin_amount as f32 * 2.0 * pin_width) + (pin_width / 8.0)),
                (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + pin_height,
            ),
        );

        // game logic
        let (mouse_x, mouse_y) = mouse_position();
        if point_in_box(&mouse_box, (mouse_x, mouse_y)) {
            let ((lockpick_x, lockpick_y), (lockpick_tip_x, lockpick_tip_y)) = calculate_lockpick_points(
                lockpick_length,
                keyway_x,
                keyway_y,
                mouse_x,
                mouse_y,
            );
            let mut lockpick_in_pin_box: bool = false;
            let mut lockpick_tip_in_pin_box: bool = false;
            for pin_box in pin_boxes.as_slice() {
                if point_in_box(pin_box, (lockpick_x, lockpick_y)) {
                    lockpick_in_pin_box = true;
                }
                if point_in_box(pin_box, (lockpick_tip_x, lockpick_tip_y)) {
                    lockpick_tip_in_pin_box = true;
                }
            }
            let mut key_pin_collides: bool = false;
            if !rake {
                for key_pin_box in generate_key_boxes(
                    pin_amount,
                    current_tension,
                    &tension_values,
                    &bitting,
                    last_mouse_x,
                    last_mouse_y,
                    last_lockpick_x,
                    last_lockpick_y,
                    last_lockpick_tip_x,
                    last_lockpick_tip_y,
                    lockpick_width
                ) {
                    if point_in_box(&key_pin_box, (lockpick_x, lockpick_y)) {
                        key_pin_collides = true;
                        break;
                    }
                    if point_in_box(&key_pin_box, (lockpick_tip_x, lockpick_tip_y)) {
                        key_pin_collides = true;
                        break;
                    }
                }    
            }
            if 
                (
                    point_in_box(&keyway_box, (lockpick_x, lockpick_y)) ||
                    point_in_box(&mouse_box, (lockpick_x, lockpick_y)) ||
                    lockpick_in_pin_box
                ) && (
                    point_in_box(&keyway_box, (lockpick_tip_x, lockpick_tip_y)) ||
                    point_in_box(&mouse_box, (lockpick_tip_x, lockpick_tip_y)) ||
                    lockpick_tip_in_pin_box
                ) && !lines_intersect(
                    &((lockpick_x, lockpick_y), (mouse_x, mouse_y)),
                    &(
                        ((lockpick_x + pin_width).min((screen_width()/2.0) - pin_width), (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0)),
                        ((screen_width()/2.0), (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0)),
                    ),
                ) && !lines_intersect(
                    &((lockpick_x, lockpick_y), (mouse_x, mouse_y)),
                    &(
                        ((screen_width()/2.0), pin_height),
                        ((screen_width()/2.0), (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0)),
                    ),
                ) && !key_pin_collides
            {
                last_mouse_x = mouse_x;
                last_mouse_y = mouse_y;
                last_lockpick_x = lockpick_x;
                last_lockpick_y = lockpick_y;
                last_lockpick_tip_x = lockpick_tip_x;
                last_lockpick_tip_y = lockpick_tip_y;
            }
        }

        // keyboard input
        if is_key_pressed(KeyCode::R) {
            bitting = generate_bitting(pin_amount);
            tension_values = generate_tension_values(pin_amount);
            current_tension = 0;
        }

        // ui
        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controls")
                .default_open(false)
                .anchor(egui::Align2::RIGHT_TOP, egui::Vec2::new(0.0, 0.0))
                .show(egui_ctx, |ui| {
                    ui.label("Mouse to play, R to reset.");
                    ui.checkbox(&mut rake, "Enable raking");
                    ui.checkbox(&mut debug, "Debug mode");
                    ui.horizontal(|ui| {
                        ui.label("Pin amount:");
                        if ui.add(Slider::new(&mut pin_amount, 1u8..=10u8)).changed() {
                            bitting = generate_bitting(pin_amount);
                            tension_values = generate_tension_values(pin_amount);
                            current_tension = 0;
                        };
                    });
                    ui.horizontal(|ui| {
                        ui.label("Current tension:");
                        if ui.add(Slider::new(&mut current_tension, 0u8..=pin_amount)).changed() {
                            //bitting = generate_bitting(pin_amount);
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
                        current_tension = 0;
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
            (screen_height()/2.0) - (pin_width/8.0) - (body_height/2.0) + (pin_height*2.0),
            plug_width + (pin_width/8.0),
            plug_height + (pin_width/4.0),
            DARKGRAY,
        );

        // plug
        draw_rectangle(
            (screen_width()/2.0) - plug_width,
            (screen_height()/2.0) - (body_height/2.0) + (pin_height*2.0),
            plug_width,
            plug_height,
            WHITE,
        );

        // keyway
        draw_rectangle(
            (screen_width()/2.0) - (pin_amount as f32 * 2.0 * pin_width) - (pin_width / 8.0),
            (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0),
            (pin_amount as f32 * 2.0 * pin_width) + (pin_width / 8.0),
            pin_height,
            DARKGRAY,
        );

        // pins
        for i in 1..=pin_amount {

            // body and plug pin cavity
            draw_rectangle(
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width) - (pin_width / 8.0),
                (screen_height()/2.0) - (body_height/2.0) + pin_width - (pin_width/8.0),
                pin_width + (pin_width / 4.0),
                pin_height + (pin_height*2.0) + pin_width + (pin_width/8.0),
                        //added 0.5 to this ^ multiplication since these are cavities, shouldn't matter
                DARKGRAY,
            );

            // key + driver pin
            let x_pos = (screen_width()/2.0) - (i as f32 * 2.0 * pin_width);
            let pickbar_height = if last_lockpick_x < x_pos {
                last_mouse_y -
                (((last_mouse_x - (x_pos + (pin_width/2.0))) / (last_mouse_x - last_lockpick_x)) * (last_mouse_y - last_lockpick_y)) -
                (lockpick_width * 0.4)
            } else {
                (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0)
            };
            let should_set = draw_key_driver_pins(
                x_pos,
                if last_lockpick_tip_x - (x_pos  - (pin_width / 8.0)) > 0.0 && last_lockpick_tip_x - (x_pos - (pin_width / 8.0)) < pin_width * 1.25 {
                    ((screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0)).min(
                        last_lockpick_tip_y - (lockpick_width * 0.4) + ((pin_width/2.0) - (last_lockpick_tip_x - x_pos)).abs()
                    )

                } else {
                    //(screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0)
                    pickbar_height.min((screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0))
                },
                (screen_height()/2.0) - (body_height/2.0) + pin_width - (pin_width/8.0),
                pin_width,
                pin_height,
                bitting[i as usize - 1],
                current_tension > tension_values[i as usize - 1]
            );

            if should_set && tension_values[i as usize - 1] == current_tension {
                current_tension += 1;
            }
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
            //pin_width/4.0,
            lockpick_width,
            LIGHTGRAY,
        );
        draw_circle(
            last_lockpick_x,
            last_lockpick_y,
            //pin_width/8.0,
            lockpick_width/2.0,
            LIGHTGRAY,
        );
        draw_line(
            last_lockpick_x,
            last_lockpick_y,
            last_lockpick_tip_x,
            last_lockpick_tip_y,
            //pin_width/4.0,
            lockpick_width,
            LIGHTGRAY,
        );
        draw_circle(
            last_lockpick_tip_x,
            last_lockpick_tip_y,
            //pin_width/8.0,
            lockpick_width/2.0,
            LIGHTGRAY,
        );

        // debug
        if debug {
            draw_text(format!("{}", get_fps()).as_str(), 0.0, 20.0, 20.0, WHITE);

            // lockpick pivot
            draw_circle(keyway_x, keyway_y, 5.0, BLUE);

            // lockpick coord
            draw_circle(last_lockpick_x, last_lockpick_y, 5.0, RED);

            // lockpick tip coord
            draw_circle(last_lockpick_tip_x, last_lockpick_tip_y, 5.0, RED);

            // mouse box
            draw_rectangle_lines(mouse_box.0.0, mouse_box.0.1, mouse_box.1.0 - mouse_box.0.0, mouse_box.1.1 - mouse_box.0.1, 2.0, RED);

            // keyway box
            draw_rectangle_lines(keyway_box.0.0, keyway_box.0.1, keyway_box.1.0 - keyway_box.0.0, keyway_box.1.1 - keyway_box.0.1, 2.0, GREEN);

            // driver pin boxes
            for pin_box in pin_boxes.as_slice() {
                draw_rectangle_lines(pin_box.0.0, pin_box.0.1, pin_box.1.0 - pin_box.0.0, pin_box.1.1 - pin_box.0.1, 2.0, BLUE);
            }

            // key pin boxes
            for key_pin_box in generate_key_boxes(
                pin_amount,
                current_tension,
                &tension_values,
                &bitting,
                last_mouse_x,
                last_mouse_y,
                last_lockpick_x,
                last_lockpick_y,
                last_lockpick_tip_x,
                last_lockpick_tip_y,
                lockpick_width
            ){
                draw_rectangle_lines(key_pin_box.0.0, key_pin_box.0.1, key_pin_box.1.0 - key_pin_box.0.0, key_pin_box.1.1 - key_pin_box.0.1, 3.0, RED);
            }

            let magenta = Color::from_rgba(255, 0, 255, 255);

            // keyway top
            let line = (
                ((last_lockpick_x + pin_width).min((screen_width()/2.0) - pin_width), (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0)),
                ((screen_width()/2.0), (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0)),
            );
            draw_line(line.0.0, line.0.1, line.1.0, line.1.1, 2.0, magenta);
            draw_text(format!("{}", lines_intersect(&line, &((last_lockpick_x, last_lockpick_y), (last_mouse_x, last_mouse_y)))).as_str(), line.0.0, line.0.1, 20.0, magenta);
        
            // front guard
            let line = (
                ((screen_width()/2.0), pin_height),
                ((screen_width()/2.0), (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0)),
            );
            draw_line(line.0.0, line.0.1, line.1.0, line.1.1, 2.0, magenta);
            draw_text(format!("{}", lines_intersect(&line, &((last_lockpick_x, last_lockpick_y), (last_mouse_x, last_mouse_y)))).as_str(), line.0.0, line.0.1, 20.0, magenta);

            // screen diagonal
            let line = ((0.0, 0.0), (screen_width(), screen_height()));
            draw_line(line.0.0, line.0.1, line.1.0, line.1.1, 2.0, magenta);
            draw_text(format!("{}", lines_intersect(&line, &((last_lockpick_x, last_lockpick_y), (last_mouse_x, last_mouse_y)))).as_str(), line.0.0, line.0.1, 20.0, magenta);
        }


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

fn point_in_box(hitbox: &((f32, f32), (f32, f32)), point: (f32, f32)) -> bool {
    point.0 > hitbox.0.0 && point.1 > hitbox.0.1 && point.0 < hitbox.1.0 && point.1 < hitbox.1.1
}

// stolen/reworked from here: https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
// Given three collinear points p, q, r, the function checks if
// point q lies on line segment 'pr'
fn on_segment(p: &(f32, f32), q: &(f32, f32), r: &(f32, f32)) -> bool {
    (q.0 <= f32::max(p.0, r.0) && q.0 >= f32::min(p.0, r.0) && q.1 <= f32::max(p.1, r.1) && q.1 >= f32::min(p.1, r.1))
}

// stolen/reworked from here: https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
// To find orientation of ordered triplet (p, q, r).
// The function returns following values
// 0 --> p, q and r are collinear
// 1 --> Clockwise
// 2 --> Counterclockwise
fn orientation(p: &(f32, f32), q: &(f32, f32), r: &(f32, f32)) -> i32 {
    // See https://www.geeksforgeeks.org/orientation-3-ordered-points/
    // for details of below formula.
    let val = ((q.1 - p.1) * (r.0 - q.0) - (q.0 - p.0) * (r.1 - q.1)) as i32;

    // collinear
    if val == 0 {
        return 0;
    }

    // clock or counterclock wise
    if (val > 0) {
        1
    } else {
        2
    }
}

// stolen/reworked from here: https://www.geeksforgeeks.org/check-if-two-given-line-segments-intersect/
// The main function that returns true if line segment 'p1q1'
// and 'p2q2' intersect.
fn lines_intersect(line1: &((f32, f32),(f32, f32)), line2: &((f32, f32),(f32, f32))) -> bool {
    let (p1, q1) = line1;
    let (p2, q2) = line2;

    // Find the four orientations needed for general and
    // special cases
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if (o1 != o2 && o3 != o4) {
        return true;
    }

    // Special Cases
    // p1, q1 and p2 are collinear and p2 lies on segment p1q1
    if (o1 == 0 && on_segment(p1, p2, q1)) {return true;}

    // p1, q1 and q2 are collinear and q2 lies on segment p1q1
    if (o2 == 0 && on_segment(p1, q2, q1)) {return true;}

    // p2, q2 and p1 are collinear and p1 lies on segment p2q2
    if (o3 == 0 && on_segment(p2, p1, q2)) {return true;}

     // p2, q2 and q1 are collinear and q1 lies on segment p2q2
    if (o4 == 0 && on_segment(p2, q1, q2)) {return true;}

    return false; // Doesn't fall in any of the above cases
}

// fn line_in_box(hitbox: &((f32, f32), (f32, f32)), start_point: (f32, f32), end_point: (f32, f32)) -> bool {
//     lines_intersect(
//         &(hitbox.0, (hitbox.0.0, hitbox.1.1)),
//         &(start_point, end_point)
//     ) ||
//     lines_intersect(
//         &(hitbox.0, (hitbox.1.0, hitbox.0.1)),
//         &(start_point, end_point)
//     ) ||
//     lines_intersect(
//         &(hitbox.1, (hitbox.0.0, hitbox.1.1)),
//         &(start_point, end_point)
//     ) ||
//     lines_intersect(
//         &(hitbox.1, (hitbox.1.0, hitbox.0.1)),
//         &(start_point, end_point)
//     )
// }

fn generate_bitting(pin_amount: u8) -> Vec<u8> {
    let mut bitting = Vec::with_capacity(pin_amount as usize);
    for _ in 0..pin_amount {
        bitting.push(
            rand::gen_range(2u8, 10u8)
        );
    }
    bitting
}

fn generate_tension_values(pin_amount: u8) -> Vec<u8> {
    let mut tension_values: Vec<u8> = (0..pin_amount).collect();
    tension_values.shuffle(&mut thread_rng());
    tension_values
}

fn generate_pin_boxes(pin_amount: u8, pin_width: f32, pin_height: f32, body_height: f32) -> Vec<((f32, f32), (f32, f32))> {
    let mut pin_boxes: Vec<((f32, f32), (f32, f32))> = Vec::with_capacity(pin_amount as usize);
    for i in 1..=pin_amount {
        pin_boxes.push((
            (
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width) - (pin_width / 8.0),
                (screen_height()/2.0) - (body_height/2.0) + pin_width - (pin_width/8.0),
            ), (
                (screen_width()/2.0) - (i as f32 * 2.0 * pin_width) - (pin_width / 8.0) + pin_width + (pin_width / 4.0),
                (screen_height()/2.0) - (body_height/2.0) + pin_width - (pin_width/8.0) + (pin_height*1.5) + (pin_height*1.5) + pin_width + (pin_width/8.0),
                                                                                        // multiplied ^ this by 1.5 since these are allow-boxes, shouldn't matter
            )
        ));
    }
    pin_boxes
}

fn generate_key_boxes(
    pin_amount: u8,
    current_tension: u8,
    tension_values: &Vec<u8>,
    bitting: &Vec<u8>,
    last_mouse_x: f32,
    last_mouse_y: f32,
    last_lockpick_x: f32,
    last_lockpick_y: f32,
    last_lockpick_tip_x: f32,
    last_lockpick_tip_y: f32,
    lockpick_width: f32,
) -> Vec<((f32, f32), (f32, f32))> {
    let mut pin_boxes: Vec<((f32, f32), (f32, f32))> = Vec::with_capacity(pin_amount as usize);
    
    let pin_width = screen_width() / 32f32.max(((pin_amount as f32 * 2.0) + 2.0) * 2.0);
    let pin_height = screen_height()/6.0;
    let body_height = ((pin_height*2.0) + pin_width) * 2.0;

    for i in 1..=pin_amount {
        let x_pos = (screen_width()/2.0) - (i as f32 * 2.0 * pin_width);
        let pickbar_height = if last_lockpick_x < x_pos {
            last_mouse_y -
            (((last_mouse_x - (x_pos + (pin_width/2.0))) / (last_mouse_x - last_lockpick_x)) * (last_mouse_y - last_lockpick_y)) -
            (lockpick_width * 0.4)
        } else {
            (screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0)
        };
        let bottom_y = if last_lockpick_tip_x - (x_pos  - (pin_width / 8.0)) > 0.0 && last_lockpick_tip_x - (x_pos - (pin_width / 8.0)) < pin_width * 1.25 {
            ((screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0)).min(
                last_lockpick_tip_y - (lockpick_width * 0.4) + ((pin_width/2.0) - (last_lockpick_tip_x - x_pos)).abs()
            )
        } else {
            //(screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0)
            pickbar_height.min((screen_height()/2.0) + pin_height - (body_height/2.0) + (pin_height*2.0) + (pin_height/2.0))
        };
        let pin_bitting = bitting[i as usize - 1];
        let key_pin_height = (pin_height * 0.5) + (pin_height * 0.05 * pin_bitting as f32);
        let pin_is_set = current_tension > tension_values[i as usize - 1];
        let y = if pin_is_set {
            (bottom_y - (pin_width/2.0) - key_pin_height).max(
                (screen_height()/2.0) -
                ((((pin_height*2.0) + pin_width) * 2.0)/2.0) +
                (pin_height*2.0)
            )
        } else {
            bottom_y - (pin_width/2.0) - key_pin_height
        };
        pin_boxes.push((
            (
                x_pos - (pin_width / 8.0),
                y,
            ), (
                x_pos + pin_width + (pin_width / 8.0) ,
                y+key_pin_height,
            )
        ));
    }

    pin_boxes
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

fn draw_key_driver_pins(x: f32, bottom_y: f32, top_y: f32, w: f32, driver_h: f32, pin_bitting: u8, pin_is_set: bool) -> bool {
    let key_pin_height = (driver_h * 0.5) + (driver_h * 0.05 * pin_bitting as f32);

    let should_set: bool = !pin_is_set && (
        ((screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + driver_h) -
        (bottom_y - (w/2.0) - key_pin_height - (w/8.0) - driver_h)
    ).abs() < 2.0;
    
    // spring
    draw_spring(
        x,
        top_y,//(screen_height()/2.0) - (driver_h*1.5),
        w,
        //(bottom_y - (w/2.0) - key_pin_height - (w/8.0) - driver_h) - ((screen_height()/2.0) - (driver_h*1.5)),
        if pin_is_set {
            ((screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + driver_h) - top_y
        } else {
            bottom_y - top_y - (w/2.0) - key_pin_height - (w/8.0) - driver_h
        },
        9,
        w/8.0,
        LIGHTGRAY,
    );

    // driver pin
    draw_rectangle(
        x,
        if pin_is_set {
            (screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + driver_h
        } else {
            bottom_y - (w/2.0) - key_pin_height - (w/8.0) - driver_h
        },
        w,
        driver_h,
        if pin_is_set {
            Color::new(0.64, 0.64, 0.64, 1.0)
        } else {
            LIGHTGRAY
        },
    );

    // key pin
    draw_rectangle(
        x,
        if pin_is_set {
            (bottom_y - (w/2.0) - key_pin_height).max((screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + (driver_h*2.0) + (w/8.0))
        } else {
            bottom_y - (w/2.0) - key_pin_height
        },
        w,
        key_pin_height,
        LIGHTGRAY,
    );
    draw_triangle(
        Vec2{
            x: x,
            y: if pin_is_set {
                (bottom_y - (w/2.0)).max((screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + (driver_h*2.0) + (w/8.0) + key_pin_height)
            } else {
                bottom_y - (w/2.0)
            },
        },
        Vec2{
            x: x + w,
            y: if pin_is_set {
                (bottom_y - (w/2.0)).max((screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + (driver_h*2.0) + (w/8.0) + key_pin_height)
            } else {
                bottom_y - (w/2.0)
            },
        },
        Vec2{
            x: x + (w/2.0),
            y: if pin_is_set {
                (bottom_y).max((screen_height()/2.0) - (w/8.0) - ((((driver_h*2.0) + w) * 2.0)/2.0) + (driver_h*2.0) + (w/8.0) + key_pin_height + (w/2.0))
            } else {
                bottom_y
            },
        },
        LIGHTGRAY,
    );

    should_set
}
