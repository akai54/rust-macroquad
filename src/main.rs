// Utilisation des bibliotheques necessaires.
use macroquad::prelude::*;

use macroquad_tiled as tiled;

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

    let width = screen_width();
    let height = screen_height();

    //Ajout tileset
    let tileset = Texture2D::from_file_with_format(
        include_bytes!("../GFX/fishgame_assets/tileset.png"),
        None,
    );

    let decorations = Texture2D::from_file_with_format(
        include_bytes!("../GFX/fishgame_assets/decorations1.png"),
        None,
    );

    //Charger le fichier json de la map.
    let tiled_map_json = load_string("GFX/fishgame_assets/map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[ 
        ("tileset.png", tileset), 
        ("decorations1.png", decorations),],
        &[],
    ).unwrap();

    //Ajout texture Personnage (120 x 201).
    let _bunny = Texture2D::from_file_with_format(
        include_bytes!("../GFX/Players/bunny1_ready.png"), 
        None,
    );

    //Ajout texture ennemie (90 x 155).
    let ennemie = Texture2D::from_file_with_format(
        include_bytes!("../GFX/Enemies/spikeMan_stand.png"),
        None,
    );

    let _ennemie2 = Texture2D::from_file_with_format(include_bytes!("../GFX/Enemies/sun1.png"),None,);

    loop {
        //Choisir la caméra actif.
        set_camera(&camera);

        tiled_map.draw_tiles(
            // The name of the layer in assets/map.json
            "main layer",
            Rect::new(0.0, 0.0, width, height),
            None,
        );

        draw_texture_ex( 
            ennemie, 
            0.0, 
            0.0, 
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 90., 155.)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
