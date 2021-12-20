// Utilisation des bibliotheques necessaires.
use macroquad::prelude::*;

// use macroquad_tiled as tiled;

#[macroquad::main("Platformer")]
async fn main() {

    /* Explications Camera2D. (Par Ordre).
       Rotation in degrees

       zoom: Vec2

       target: Vec2
       Rotation and zoom origin

       offset: Vec2
       Displacement from target

       render_target: Option<RenderTarget>
       If “render_target” is set - camera will render to texture otherwise to the screen

       viewport: Option<(i32, i32, i32, i32)>
       */
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, 320.0, 152.0));
    
    //Ajout tileset
    let tileset = load_texture("../Tilemap/spritesheet_jumper.png").await;

    //Ajout texture Personnage (120 x 201).
    let bunny = load_texture("../GFX/Players/bunny1_stand.png").await;

    let width = 1000;
    let height = 800;
    loop {
        clear_background(WHITE);
        
        //Choisir la caméra actif.
        set_camera(&camera);
        
        next_frame().await
    }
}
