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

    let _width = 700;
    let _height = 500;

    //Ajout tileset
    let tileset = Texture2D::from_file_with_format(
        include_bytes!("../Tilemap/tileset.png"),
        None,
    );

    let portal = Texture2D::from_file_with_format(
        include_bytes!("../GFX/Items/portal_yellow.png"),
        None,
    );

    //Charger le fichier json de la map.
    let tiled_map_json = load_string("../Tilemap/map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[ 
        ("tileset.png", tileset), 
        ("portal_yellow.png", portal),],
        &[],
    );

    //Ajout texture Personnage (120 x 201).
    let _bunny = Texture2D::from_file_with_format(
        include_bytes!("../GFX/Players/bunny1_ready.png"), 
        None,
    );

    //Ajout texture ennemie (142 x 148).
    let ennemie = Texture2D::from_file_with_format(
        include_bytes!("../images/spikeMan.png"),
        None,
     );

    
    let ennemie2 = Texture2D::from_file_with_format(include_bytes!("../images/sun1.png"),None,);

    loop {
        clear_background(WHITE);

        //Choisir la caméra actif.
        set_camera(&camera);

        draw_texture_ex( 
            ennemie, 
            0.0, 
            0.0, 
            WHITE,
            DrawTextureParams {
            source: Some(Rect::new(0.0, 0.0, 140., 142.)),
            ..Default::default()
        },
        );
        draw_texture_ex(
            ennemie2,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(-40.0, 30.0, 140., 142.)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}