use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    loop {
        clear_background(RED);

        //        x1    y1    x2     y2     thick color.
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);

        //[x, y] with size [w, h].
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

        //[x, y] with a given radius r.
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        //[x1, y1] and [x2, y2] thickness color.
        draw_text("IT WORKS!", 29.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
