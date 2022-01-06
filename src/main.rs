// Utilisation des bibliotheques necessaires.
use macroquad::prelude::*;

use macroquad_tiled as tiled;

use macroquad_platformer::*;

use macroquad::experimental::{
    collections::storage,
    scene::{Node, RefMut},
};

//Donc macroquad_platformer est une crate qui nous permet d'avoir un système physique_joueur,
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
    physique_joueur: World,
}

impl Joueur {
    //Les constants du jeu.
    pub const VITESSE_SAUT: f32 = -700.0;
    pub const GRAVITE: f32 = 2000.0;
    pub const VITESSE_MOUV: f32 = 300.0;

    fn new() -> Joueur {
        let mut ressources = storage::get_mut::<Ressources>();

        Joueur {
            collider: ressources.physique_joueur.add_actor(vec2(250.0, 100.0), 32, 51),
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

        let bunny_pos = ressources.physique_joueur.actor_pos(node.collider);

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
        let monde = &mut storage::get_mut::<Ressources>().physique_joueur;

        //Contient la position de Bunny.
        let bunny_pos = monde.actor_pos(node.collider);

        //Un bool qui indique si Bunny est sur le sol ou pas.
        let sur_le_sol = monde.collide_check(node.collider, bunny_pos + vec2(0., 1.));

        println!("{}", bunny_pos);

        //Si bunny n'est pas sur le sol, alors sa vitesse sera de:
        if sur_le_sol == false{
            node.vitesse.y += Self::GRAVITE * get_frame_time();
        }

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
    let tileset = load_texture("GFX/TileMap/Terrain.png").await.unwrap();

    //Sets the FilterMode of this texture.
    tileset.set_filter(FilterMode::Nearest);

    //Ajout texture Personnage (32 x 51).
    let bunny = load_texture("GFX/Players/resized/bunny1_ready.png").await.unwrap();
    bunny.set_filter(FilterMode::Nearest);

    let ennemi = load_texture("GFX/Enemies/spikeMan_stand.png").await.unwrap();
    ennemi.set_filter(FilterMode::Nearest);

    let all = load_texture("GFX/TileMap/all.png").await.unwrap();
    all.set_filter(FilterMode::Nearest);

    //Charger le fichier json de la map.
    let tiled_map_json = load_string("GFX/TileMap/map.json").await.unwrap();

    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("Terrain.png", tileset), ("all.png", all)],
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

    let mut physique_joueur = World::new();

    //Ici on ajoute les tuiles qui sont statiques,
    //on leur connait grace à la taille des tuiles en pixel de la tilemap.
    //Donc par ordre: largeur de la tuile - longeur de la tuile - largeur et le label ou
    //l'étiquette.
    physique_joueur.add_static_tiled_layer( 
        collisions_statiques, 
        tiled_map.raw_tiled_map.tilewidth as f32, 
        tiled_map.raw_tiled_map.tileheight as f32, 
        tiled_map.raw_tiled_map.width as _, 
        1,
    );
     
    let ressource_joueur = Ressources{bunny,physique_joueur};
    storage::store(ressource_joueur);
    
    //Ajout du variable joueur, qui utilise la struct Joueur.
    let joueur = Joueur::new();
 
    scene::add_node(joueur);

    let largeur = tiled_map.raw_tiled_map.tilewidth as f32 * tiled_map.raw_tiled_map.width as f32;
    let longeur = tiled_map.raw_tiled_map.tileheight as f32* tiled_map.raw_tiled_map.height as f32;

    loop {
        clear_background(WHITE);

        tiled_map.draw_tiles(
            // The name of the layer in assets/map.json
            "main layer",
            Rect::new(0.0, 0.0, largeur, longeur),
            None,
        );

        next_frame().await;
    }
}
