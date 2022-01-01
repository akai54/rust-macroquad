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
    let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

    let width = 700.;
    let height = 500.;

    pub enum Keycode {
        Right,
        Left,
        Up,
        Down,
    }

    //Ajout tileset
    let tileset = Texture2D::from_file_with_format(
        include_bytes!("../GFX/fishgame_assets/tileset.png"),
        None,
    );

    //Sets the FilterMode of this texture.
    tileset.set_filter(FilterMode::Nearest);

    let decorations = Texture2D::from_file_with_format(
        include_bytes!("../GFX/fishgame_assets/decorations1.png"),
        None,
    );
    decorations.set_filter(FilterMode::Nearest);

    //Charger le fichier json de la map.
    let tiled_map_json = load_string("GFX/fishgame_assets/map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset), ("decorations1.png", decorations)],
        &[],
    )
    .unwrap();

    //Ajout texture Personnage (32 x 51).
    let bunny = Texture2D::from_file_with_format(
        include_bytes!("../GFX/Players/resized/bunny1_ready.png"),
        None,
    );
    bunny.set_filter(FilterMode::Nearest);

    //Ajout de la position de bunny.
    let mut bunny_pos = vec2(200., 100.);

    loop {
        clear_background(BLACK);

        //Choisir la caméra actif.
        set_camera(&camera);

        tiled_map.draw_tiles(
            // The name of the layer in assets/map.json
            "main layer",
            Rect::new(0.0, 0.0, width, height),
            None,
        );

        draw_texture_ex(
            bunny,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2:: bunny_pos),
                source: Some(Rect::new(0.0, 0.0, 32., 51.)),
                ..Default::default()
            },
        );

        //Condition de touche pour bouger bunny.
        /* if is_key_pressed(key_code: KeyCode) {
            True
        } */
        if is_key_down(KeyCode::Right) {
            bunny_pos.x += 1.0;
        }
        if is_key_down(KeyCode::Left) {
            bunny_pos.x -= 1.0;
        }

        let bunny_bottom_point = vec2(bunny_pos.x + 76. / 2., bunny_pos.y + 66.);

        let bunny_tile = vec2(
            bunny_bottom_point.x / width * tiled_map.raw_tiled_map.width as f32,
            bunny_bottom_point.y / height * tiled_map.raw_tiled_map.height as f32,
        );

        if tiled_map
            .get_tile("main layer", bunny_tile.x as u32, bunny_tile.y as u32)
            .is_none()
        {
            bunny_pos.y += 20.0;
        }
        next_frame().await
    }
}
