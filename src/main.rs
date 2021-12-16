// Utilisation des bibliotheques necessaires.
use macroquad::prelude::*;

use macroquad_tiled as tiled;

#[macroquad::main("Platformer")]
async fn main() {

    /*
       rotation: f32
       Rotation in degrees

       zoom: Vec2
       Scaling, should be (1.0, 1.0) by default

       target: Vec2
       Rotation and zoom origin

       offset: Vec2
       Displacement from target

       render_target: Option<RenderTarget>
       If “render_target” is set - camera will render to texture otherwise to the screen

       viewport: Option<(i32, i32, i32, i32)>

       Part of the screen to render to

       None means the whole screen

       Viewport do not affect camera space, just the render position on the screen

       Usefull for things like splitscreen
       */
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 320.0, 152.0));

    loop {
        clear_background(WHITE);
        
        //Choisir la caméra actif.
        set_camera(&camera);
        
        next_frame().await
    }
}
