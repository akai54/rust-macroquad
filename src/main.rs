// Utilisation des bibliotheques necessaires.
use macroquad::prelude::*;

use macroquad_tiled as tiled;

use macroquad_platformer::*;

use macroquad::experimental::{
    collections::storage,
    scene::{Node, RefMut},
};

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
//Une structure qui contient tout les ressources utilisé dans le jeu.
struct Ressources {
    bunny: Texture2D,
    physique: World,
}

impl Joueur {
    //Les constants du jeu.
    pub const VITESSE_SAUT: f32 = -700.0;
    pub const GRAVITE: f32 = 2000.0;
    pub const VITESSE_MOUV: f32 = 300.0;

    fn new() -> Joueur {
        let mut ressources = storage::get_mut::<Ressources>();

        Joueur {
            collider: ressources.physique.add_actor(vec2(200.0, 100.0), 36, 66),
            vitesse: vec2(0., 0.),
        }
    }
}

impl Node for Joueur {
    fn draw(node: RefMut<Self>) {
        let camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, screen_width(), screen_height()));
        //Choisir la caméra actif.
        set_camera(&camera);

        let ressources = storage::get_mut::<Ressources>();

        let mut bunny_pos = ressources.physique.actor_pos(node.collider);
        //À cause d'un probleme de collision, j'ai du ajouter la valeur 15 à la position Y de
        //bunny.
        bunny_pos.y += 15.0;

        draw_texture_ex(
            ressources.bunny,
            bunny_pos.x,
            bunny_pos.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0.0, 0.0, 32., 51.)),
                ..Default::default()
            },
        );
    }

    //Cette fonction a pour but de mettre à jour les parametres du jeu, comme la position de bunny
    //et de vérifier des conditions tels que les le mouvement de bunny.
    fn update(mut node: RefMut<Self>){
        //Donc ici on créer notre monde, avec la structure World, déjà implemnté dans
        //macroquad_platformer.
        let monde = &mut storage::get_mut::<Ressources>().physique;

        //Contient la position de Bunny.
        let bunny_pos = monde.actor_pos(node.collider);

        //Un bool qui indique si Bunny est sur le sol ou pas.
        let sur_le_sol = monde.collide_check(node.collider, bunny_pos + vec2(0., 1.));

        //Si bunny n'est pas sur le sol, alors sa vitesse sera de:
        if sur_le_sol == false{
            node.vitesse.y += Self::GRAVITE * get_frame_time();
        }

        println!("{}", bunny_pos);
        
        //Condition de touche pour bouger bunny.
        if is_key_down(KeyCode::Right) {
            node.vitesse.x = Self::VITESSE_MOUV;
        }

        else if is_key_down(KeyCode::Left) {
            node.vitesse.x = - Self::VITESSE_MOUV;
        }

        else if is_key_down(KeyCode::Down) {
            node.vitesse.y = - Self::VITESSE_SAUT;
        }

        else if is_key_pressed(KeyCode::Space) {
            if sur_le_sol{
                node.vitesse.y = Self::VITESSE_SAUT;
            } 
        }

        else {
            node.vitesse.x = 0.;
        }

        //On affiche le joueur grace à sa position communiqué par macroquad_platformer.
        monde.move_h(node.collider, node.vitesse.x * get_frame_time());
        monde.move_v(node.collider, node.vitesse.y * get_frame_time());
    }
}

#[macroquad::main("Platformer")]
async fn main() {

    //Ajout tileset
    let tileset = load_texture("GFX/fishgame_assets/tileset.png").await.unwrap();

    //Sets the FilterMode of this texture.
    tileset.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 51).
    let bunny = load_texture("GFX/Players/resized/bunny1_ready.png").await.unwrap();
    bunny.set_filter(FilterMode::Nearest);

    let decorations = load_texture("GFX/fishgame_assets/decorations1.png").await.unwrap();
    decorations.set_filter(FilterMode::Nearest);

    //Charger le fichier json de la map.
    let tiled_map_json = load_string("GFX/fishgame_assets/map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset), ("decorations1.png", decorations)],
        &[],
    )
        .unwrap();

    //Les tuiles statiques, sont sauvegardé dans un vecteur.
    let mut collisions_statiques = vec![];

    //Dans cette boucle nous allons ajouter tout les tuiles, qui sont dans la tilemap déja dans le
    //vecteur collisions_statiques, donc soit c'est solide comme tuile soit il n'y a rien.
    for (_x, _y, tile) in tiled_map.tiles("main layer", None) {
        collisions_statiques.push(if tile.is_some() {
            Tile::Solid
        } else {
            Tile::Empty
        });
    }

    let mut physique = World::new();

    //Ici on ajoute les tuiles qui sont statiques,
    //on leur connait grace à la taille des tuiles en pixel de la tilemap.
    //Donc par ordre: largeur de la tuile - longeur de la tuile - largeur et le label ou
    //l'étiquette.
    physique.add_static_tiled_layer( 
        collisions_statiques, 
        tiled_map.raw_tiled_map.tilewidth as f32, 
        tiled_map.raw_tiled_map.tileheight as f32, 
        tiled_map.raw_tiled_map.width as _, 
        1,
    );

    let res = Ressources{bunny,physique};
    storage::store(res);
    
    //Ajout du variable joueur, qui utilise la struct Joueur.
    let joueur = Joueur::new();

    scene::add_node(joueur);

    let largeur = tiled_map.raw_tiled_map.tilewidth as f32 * tiled_map.raw_tiled_map.width as f32;
    let longeur = tiled_map.raw_tiled_map.tileheight as f32* tiled_map.raw_tiled_map.height as f32;

    loop {
        clear_background(BLACK);

        tiled_map.draw_tiles(
            // The name of the layer in assets/map.json
            "main layer",
            Rect::new(0.0, 0.0, largeur, longeur),
            None,
        );

        next_frame().await;
    }
}
