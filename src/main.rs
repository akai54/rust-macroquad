// Utilisation des bibliotheques necessaires.
use macroquad::ui::root_ui;
use macroquad::{audio::*, prelude::*};
use macroquad_platformer::*;
use macroquad_tiled as tiled;

//Donc macroquad_platformer est une crate qui nous permet d'avoir un système physique,
//dans notre jeu, sans avoir à tout manipuler de manière manuelle, mais il faudra quand meme
//préciser certaines informations pour que cela fonctionne.
//Ce système est basé sur l'article suivant: https://maddythorson.medium.com/celeste-and-towerfall-physics-d24bd2ae0fc5
//écrit par Maddy thorson pour les jeux qu'il a devloppé.

//Structure pour le joueur, qui contient la vitesse ainsi que son type de collision.
struct Joueur {
    collider: Actor,
    vitesse: Vec2,
}

struct Obstacles {
    _collider: Solid,
}

struct Tirs {
    collider: bool,
    pos: Vec2,
}

//Les constants du jeu.
mod consts {
    pub const VITESSE_SAUT: f32 = -700.0;
    pub const GRAVITE: f32 = 2000.0;
    pub const VITESSE_MOUV: f32 = 300.0;
    pub const LIMITE_MONDE: f32 = 5000.0;
    pub const VITESSE_BOOST: f32 = 2.0;
}

fn anima(texture: Texture2D, x: f32, y: f32, flip: bool){
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(0.0, 0.0, texture.width(), texture.height())),
                    flip_x: flip,
                    ..Default::default()
                },
            );
}

async fn end() {
    loop {
        let end = load_texture("GFX/SeasonalTilesets/end.png").await.unwrap();
        end.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            end,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        if root_ui().button(
            Vec2::new(screen_width() / 2., screen_height() / 2.),
            "Quitter",
        ) {
            break;
        }
        next_frame().await;
    }
}

async fn start() {
    //nombre de vies de bunny.
    let mut nombre_vies = 6;

    //Choisir la caméra actif.
    let mut camera =
        Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));

    //Ajout tileset
    let tileset = load_texture("GFX/TileMap/Terrain.png").await.unwrap();

    //Sets the FilterMode of this texture.
    tileset.set_filter(FilterMode::Nearest);

    let autumn = load_texture("GFX/TileMap/Autumn_entities(16x16).png")
        .await
        .unwrap();
    autumn.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 51).
    let bunny_ready = load_texture("GFX/Players/resized/bunny1_ready.png")
        .await
        .unwrap();
    bunny_ready.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 54).
    let bunny_stand = load_texture("GFX/Players/resized/bunny1_stand.png")
        .await
        .unwrap();
    bunny_stand.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 37).
    let bunny_hurt = load_texture("GFX/Players/resized/bunny1_hurt.png")
        .await
        .unwrap();
    bunny_hurt.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 39).
    let bunny_jump = load_texture("GFX/Players/resized/bunny1_jump.png")
        .await
        .unwrap();
    bunny_jump.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 54).
    let bunny_marche1 = load_texture("GFX/Players/resized/bunny1_walk1.png")
        .await
        .unwrap();
    bunny_marche1.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 55).
    let bunny_marche2 = load_texture("GFX/Players/resized/bunny1_walk2.png")
        .await
        .unwrap();
    bunny_marche2.set_filter(FilterMode::Nearest);

    //
    let ennemi_draw = load_texture("GFX/Enemies/resized/spikeMan_stand.png")
        .await
        .unwrap();
    ennemi_draw.set_filter(FilterMode::Nearest);

    let all = load_texture("GFX/TileMap/all.png").await.unwrap();
    all.set_filter(FilterMode::Nearest);

    let bg = load_texture("GFX/TileMap/bg.png").await.unwrap();
    bg.set_filter(FilterMode::Nearest);

    let spring = load_texture("GFX/Items/resized/spring.png").await.unwrap();
    spring.set_filter(FilterMode::Nearest);

    let tirs_texture = load_texture("GFX/TileMap/tirs.png").await.unwrap();
    tirs_texture.set_filter(FilterMode::Nearest);

    let spring_in = load_texture("GFX/Items/resized/spring_in.png")
        .await
        .unwrap();
    spring_in.set_filter(FilterMode::Nearest);

    let spring_out = load_texture("GFX/Items/resized/spring_out.png")
        .await
        .unwrap();
    spring_out.set_filter(FilterMode::Nearest);

    let son_bg = load_sound("SFX/Chiptune_Adventures_1.ogg").await.unwrap();

    let son_saut = load_sound("SFX/jump1.ogg").await.unwrap();

    let son_blesse = load_sound("SFX/pain.ogg").await.unwrap();

    let obstacle = load_texture("GFX/Environment/spikes_top_resized.png")
        .await
        .unwrap();
    obstacle.set_filter(FilterMode::Nearest);

    let mut tirs = Vec::new();

    //Charger le fichier json de la map.
    let tiled_map_json = load_string("GFX/TileMap/map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[
        ("Terrain.png", tileset),
        ("all.png", all),
        ("bg.png", bg),
        ("Autumn_entities(16x16).png", autumn),
        ],
        &[],
    )
        .unwrap();

    //Les tuiles statiques, sont sauvegardé dans un vecteur.
    let mut collisions_statiques = vec![];

    //Dans cette boucle nous allons ajouter tout les tuiles, qui sont dans la tilemap déja dans le
    //vecteur collisions_statiques, donc soit c'est solide comme tuile soit il n'y a rien.
    for (_x, _y, tile) in tiled_map.tiles("main-layer", None) {
        collisions_statiques.push(
            //OK donc, ici cette boucle est censé donner des collisions aux tiles suivant la tilemap
            //crée, sauf qu'avec cette condition, les tiles venant de la tileset "all" n'auront pas
            //de collisions !! C'est donc merveillieux pour les décorations.
            //Update: filter plus simple que map.
            //Encore plus simple.
            if tile.is_some() {
                if tile.as_ref().filter(|tile| tile.tileset == "all").is_some() {
                    Tile::Empty
                } else if tile
                    .as_ref()
                        .filter(|tile| tile.tileset == "Autumn_entities(16x16)")
                        .is_some()
                {
                    Tile::Empty
                } else {
                    Tile::Solid
                }
            } else {
                Tile::Empty
            },
        );
    }

    let mut monde = World::new();

    //Ici on ajoute les tuiles qui sont statiques,
    //on leur connait grace à la taille des tuiles en pixel de la tilemap.
    //Donc par ordre: largeur de la tuile - longeur de la tuile - largeur et le label ou
    //l'étiquette.
    monde.add_static_tiled_layer(
        collisions_statiques,
        tiled_map.raw_tiled_map.tilewidth as f32,
        tiled_map.raw_tiled_map.tileheight as f32,
        tiled_map.raw_tiled_map.width as _,
        1,
    );

    //Ajout du variable joueur, qui utilise la struct Joueur.
    let mut joueur = Joueur {
        //En ce qui concerne les collision, le joueur est un acteur et non pas un objet statique.
        collider: monde.add_actor(vec2(200.0, 100.0), 32, 51),
        vitesse: vec2(0., 0.),
    };
    let mut _obs = Obstacles {
        _collider: monde.add_solid(vec2(901., 610.), 32, 18),
    };

    let largeur = tiled_map.raw_tiled_map.tilewidth as f32 * tiled_map.raw_tiled_map.width as f32;
    let longeur = tiled_map.raw_tiled_map.tileheight as f32 * tiled_map.raw_tiled_map.height as f32;

    play_sound(
        son_bg,
        PlaySoundParams {
            looped: true,
            volume: 0.6,
        },
    );
    loop {
        clear_background(WHITE);

        set_camera(&camera);

        //Contient la position de Bunny.
        let mut bunny_pos = monde.actor_pos(joueur.collider);

        //La caméra suit le joueur.
        camera = Camera2D::from_display_rect(Rect::new(
                bunny_pos.x / 3.5,
                bunny_pos.y / 3.5,
                screen_width(),
                screen_height(),
        ));

        //Afficher fond d'écran.
        draw_texture_ex(
            bg,
            bunny_pos.x / 3.5,
            bunny_pos.y / 3.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        //Afficher les tuiles.
        tiled_map.draw_tiles(
            // The name of the layer in assets/map.json
            "main-layer",
            Rect::new(0.0, 0.0, largeur, longeur),
            None,
        );
        //Afficher spring.
        anima(spring, 275., 500., false);

        //Afficher obstacle.
        anima(obstacle, 901., 624., false);

        //Affichage nombre de vies restants.
        draw_text(
            &nombre_vies.to_string(),
            bunny_pos.x + 10.,
            bunny_pos.y - 6.,
            15.,
            GREEN,
        );

        //Un bool qui indique si Bunny est sur le sol ou pas.
        let sur_le_sol = monde.collide_check(joueur.collider, bunny_pos + vec2(0., 1.));

        //Si bunny n'est pas sur le sol, alors sa vitesse en l'air va se diminuer.
        if sur_le_sol == false {
            joueur.vitesse.y += consts::GRAVITE * get_frame_time();
            anima(bunny_jump, bunny_pos.x, bunny_pos.y, false);

            //Si la position de bunny dépasse la limite du monde,
            //alors le jeu prend fin.
            if bunny_pos.y > consts::LIMITE_MONDE {
                end().await;
                break;
            }
        }

        //Condition pour tirer.
        if is_key_pressed(KeyCode::RightShift) {
            tirs.push(Tirs {
                collider: false,
                pos: vec2(bunny_pos.x + 10.0, bunny_pos.y + 10.0),
            } 
            ) 
        };
        // Si bunny est sur le spring, alors il va sauter en l'air.
        if bunny_pos.x > 242. && bunny_pos.x < 308. {
            anima(spring_out, 275., 492., false);
            if bunny_pos.y == 461. {
                joueur.vitesse.y = consts::VITESSE_SAUT * consts::VITESSE_BOOST;
            }
        }
        if nombre_vies < 1 {
            end().await;
            break;
        }

        //Si bunny est sur l'obstacle.
        if bunny_pos.y == 558.0 && bunny_pos.x > 868. && bunny_pos.x < 934. {
            bunny_pos.y = bunny_pos.y - 100.;
            joueur.vitesse.y = joueur.vitesse.y + consts::VITESSE_SAUT;
            anima(bunny_hurt, bunny_pos.x, bunny_pos.y, false);
            play_sound_once(son_blesse);
            nombre_vies = nombre_vies - 1;
        }

        //Condition de touche pour bouger bunny.
        if is_key_down(KeyCode::Right) {
            joueur.vitesse.x = consts::VITESSE_MOUV;
            anima(bunny_marche1,bunny_pos.x,bunny_pos.y, false);
        }

        else if is_key_down(KeyCode::Left) {
            joueur.vitesse.x = - consts::VITESSE_MOUV;
            anima(bunny_marche1,bunny_pos.x,bunny_pos.y, true);
        }
        else if is_key_pressed(KeyCode::Space) {
            if sur_le_sol{
                joueur.vitesse.y = consts::VITESSE_SAUT;
                play_sound(
                    son_saut,
                    PlaySoundParams {
                        looped: false,
                        volume: 0.6,
                    },
                );
            }
        } else {
            joueur.vitesse.x = 0.;
            if sur_le_sol == true {
                anima(bunny_stand, bunny_pos.x, bunny_pos.y, false);
            }
        }

        for tir in tirs.iter() {
            anima(tirs_texture, tir.pos.x, tir.pos.y, false);
        }

        for tir in tirs.iter_mut(){
            tir.pos.x += 10.;
        }

        //On affiche le joueur grace à sa position communiqué par macroquad_platformer.
        monde.move_h(joueur.collider, joueur.vitesse.x * get_frame_time());
        monde.move_v(joueur.collider, joueur.vitesse.y * get_frame_time());

        next_frame().await;
    }

}

#[macroquad::main("Macroquad")]
async fn main() {

    loop {
        //Image du menu avant le démarrage du jeu.
        let begin = load_texture("GFX/SeasonalTilesets/begin.png")
            .await
            .unwrap();
        begin.set_filter(FilterMode::Nearest);
        draw_texture_ex(
            begin,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        //Le bouton Jouer
        if root_ui().button(
            vec2(screen_width() / 2. - 50., screen_height() / 2.),
            "Jouer",
        ) {
            println!("Chargement\nMettez le jeu en plein écran pour une meilleur expérience.");
            start().await;
            break;
        }
        //Le bouton quitter
        if root_ui().button(
            Vec2::new(screen_width() / 4., screen_height() / 4.),
            "Quitter",
        ) {
            break;
        }
        next_frame().await;
    }
}
