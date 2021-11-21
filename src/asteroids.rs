// is there a way to only do this once?
#[cfg(feature = "gui")]
use std::path::Path;
use sdl2;
#[cfg(feature = "gui")]
use sdl2::pixels::Color;
#[cfg(feature = "gui")]
use sdl2::rect::Rect;
#[cfg(feature = "gui")]
use sdl2::render::{Canvas, TextureCreator};
#[cfg(feature = "gui")]
use sdl2::surface::Surface;
#[cfg(feature = "gui")]
use sdl2::video::Window;
use sdl2::image::LoadTexture;

use crate::collision;

use crate::circles;

use rand::Rng;

const SMALL_ASTEROID_INDEX: usize = 0;
const BIG_ASTEROID_INDEX: usize = 1;


/// contains a list of resources used for rendering. 
pub struct ImageResources<'a> {
    // one of these can likely be deleted.
    // asteroids: [sdl2::surface::Surface<'a>; 2],

    asteroids_texture: sdl2::render::Texture<'a>,
    bullet_texture: sdl2::render::Texture<'a>,
    player_texture: sdl2::render::Texture<'a>,
}

impl<'a> ImageResources<'a> {
    // loads up images from the resource directory.
    // these images can only be rendered to the canvas of the associated texture creator.
    // kinda wonk imo.
    pub fn from_dir<T>(path: &Path, texture_creator: &'a TextureCreator<T>) -> Self {
        let big_asteroid_p = path.join("images").join("asteroid_big.bmp");
        let bullet_p = path.join("images").join("bullet.bmp");
        let player_p = path.join("images").join("player.bmp");
        let asteroid_text = texture_creator.load_texture(big_asteroid_p).unwrap();
        let bullet_texture = texture_creator.load_texture(bullet_p).unwrap();
        let player_texture = texture_creator.load_texture(player_p).unwrap();
        Self {
            asteroids_texture: asteroid_text,
            bullet_texture,
            player_texture,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MoveAblePos {
    pub pos_x: f64,
    pub pos_y: f64,
    velocity: f64,
    /// can only be values of 0 -> 2PI.
    direction: f64,
}

#[derive(Clone, Debug)]
pub struct Asteroid {
    rust_sux: MoveAblePos,
    radius: f64,
}

impl Asteroid {
    pub fn bounding_box(&self) -> collision::Circle {
        return collision::Circle {
            pos_x: self.rust_sux.pos_x,
            pos_y: self.rust_sux.pos_y,
            radius: self.radius,
        };
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    // todo: rename this to 
    pub rust_sux: MoveAblePos,
    radius: f64,
}

impl Player {
    pub fn bounding_box(&self) -> collision::Circle {
        return collision::Circle {
            pos_x: self.rust_sux.pos_x,
            pos_y: self.rust_sux.pos_y,
            radius: 2.0,
        };
    }
}

#[derive(Clone, Debug)]
struct Bullet {
    rust_sux: MoveAblePos,
    /// amount of update time the bullet will exists for.
    life_time: f64,
    radius: f64,
}

impl Bullet {
    fn bounding_box(&self) -> collision::Circle {
        return collision::Circle {
            pos_x: self.rust_sux.pos_x,
            pos_y: self.rust_sux.pos_y,
            radius: self.radius,
        };
    }
}

#[derive(Clone, Debug)]
pub struct GameState {
    pub asteroids: Vec<Asteroid>,
    pub player: Player,
    bullets: Vec<Bullet>,
    shoot_bullet_cd: i16,
    world_width: f64,
    world_height: f64,
    // if true then the game is finished.
    pub game_over: bool,
    pub game_over_is_win: bool,
    pub score: u64,
}

pub struct GameInput {
    // radians value for what to update the ship with.
    pub rotation: f64,

    // if true then the player is wanting to shoot a bullet
    pub shoot: bool,

    // if true then player is wanting to move forward.
    pub thrusters: bool,
}

pub fn game_init() -> GameState {
    let mut game_state = GameState {
        asteroids: vec![],
        game_over: false,
        game_over_is_win: false,
        player: Player {
            rust_sux: MoveAblePos {
                pos_x: 50.0,
                pos_y: 50.0,
                velocity: 0.0,
                direction: 0.0,
            },
            radius: 10.0,
        },
        bullets: vec![],
        world_width: 100.0,
        world_height: 100.0,
        shoot_bullet_cd: 0,
        score: 0,
    };

    let mut rng = rand::thread_rng();

    for _i in 0..rng.gen_range(5, 10) {
        game_state.asteroids.push(Asteroid {
            rust_sux: MoveAblePos {
                pos_x: rng.gen_range(10.0, 50.0),
                pos_y: rng.gen_range(10.0, 50.0),
                velocity: rng.gen_range(1.0, 2.0),
                direction: rng.gen_range(0.0, std::f64::consts::PI),
            },
            radius: 8.0,
        });
    }
    return game_state;
}

fn update_pos(r: &mut MoveAblePos, dt: f64, world_width: f64, world_height: f64) {
    r.pos_x += dt * r.velocity * (r.direction).cos();
    r.pos_y += dt * r.velocity * (r.direction).sin();

    if r.pos_x > world_width {
        r.pos_x = 0.0;
    }
    if r.pos_y > world_height {
        r.pos_y = 0.0;
    }
    if r.pos_x < 0.0 {
        r.pos_x = world_width;
    }
    if r.pos_y < 0.0 {
        r.pos_y = world_height;
    }
}

// called when the player wishes to shoot a bullet
fn shoot_bullet(game_state: &mut GameState) -> () {
    let p = &game_state.player;
    let bullet = Bullet {
        // maybe could clone the players MoveAblePos
        rust_sux: MoveAblePos {
            pos_x: p.rust_sux.pos_x,
            pos_y: p.rust_sux.pos_y,
            velocity: p.rust_sux.velocity + 2.0,
            direction: p.rust_sux.direction,
        },
        life_time: 20.0,
        radius: 2.0,
    };

    game_state.bullets.push(bullet);
}

// update game logic
fn game_state_update(game_state: GameState, dt: f64, game_input: &GameInput) -> GameState {
    let mut new_state = game_state.clone();

    new_state.shoot_bullet_cd = game_state.shoot_bullet_cd - 1;

    if new_state.shoot_bullet_cd < 0 {
        new_state.shoot_bullet_cd = 0;
    }
    if game_input.shoot && new_state.shoot_bullet_cd == 0 {
        shoot_bullet(&mut new_state);
        // todo: what should the cd be?
        new_state.shoot_bullet_cd = 20;
    }

    if game_input.thrusters {
        new_state.player.rust_sux.velocity = 2.0;
    } else {
        // need some sort of decay
        new_state.player.rust_sux.velocity = 0.0;
    }

    // todo: add in wrap around for bullets and asteroids and player etc.
    new_state.player.rust_sux.direction += 0.5 * game_input.rotation * dt;

    if new_state.player.rust_sux.direction > 2.0 * std::f64::consts::PI {
        new_state.player.rust_sux.direction -= 2.0 * std::f64::consts::PI;
    }

    if new_state.player.rust_sux.direction < 0.0 {
        new_state.player.rust_sux.direction += 2.0 * std::f64::consts::PI;
    }

    let player = &mut new_state.player;

    update_pos(
        &mut player.rust_sux,
        dt,
        game_state.world_width,
        game_state.world_height,
    );

    for ast in new_state.asteroids.iter_mut() {
        update_pos(
            &mut ast.rust_sux,
            dt,
            game_state.world_width,
            game_state.world_height,
        );
    }

    for bullet in new_state.bullets.iter_mut() {
        update_pos(
            &mut bullet.rust_sux,
            dt,
            game_state.world_width,
            game_state.world_height,
        );
        bullet.life_time -= 1.0 * dt;
    }

    new_state.bullets.retain(|bull| bull.life_time > 0.0);

    // check for collision
    let mut new_asteroids = Vec::new();

    // update for asteroids and bullets.
    for ast in new_state.asteroids.iter() {
        let mut deleted_aster = false;
        // todo: switch to filter on lifetime and can move retain to after this double loop?
        for bull in new_state.bullets.iter_mut() {
            if collision::collides(&ast.bounding_box(), &bull.bounding_box()) {
                // break the asteroid into two, and give some random direction and velocity.
                // remove the bullet.

                // only make new asteroids from those that are large enough.
                // large asteroid
                if ast.radius > 3.0 {
                    // add two asteroids.
                    new_asteroids.push(Asteroid {
                        rust_sux: MoveAblePos {
                            pos_x: ast.rust_sux.pos_x,
                            pos_y: ast.rust_sux.pos_y,
                            // todo: change this at some point.
                            velocity: ast.rust_sux.velocity - 0.1,
                            direction: ast.rust_sux.direction,
                        },
                        radius: ast.radius / 2.0,
                    });

                    new_asteroids.push(Asteroid {
                        rust_sux: MoveAblePos {
                            pos_x: ast.rust_sux.pos_x,
                            pos_y: ast.rust_sux.pos_y,
                            // todo: change this at some point.
                            velocity: ast.rust_sux.velocity + 0.1,
                            // send this one in the opposite direction.
                            direction: (ast.rust_sux.direction + std::f64::consts::PI * 0.5),
                        },
                        radius: 3.0,
                    });
                }
                deleted_aster = true;
                // 100 points per asteroid killed.
                new_state.score += 100;
                bull.life_time = 0.0;
                break;
            }
        }
        // is this good here? wouldn't want a bullet to
        // be able to kill two asteroids right?
        new_state.bullets.retain(|bull| bull.life_time > 0.0);

        if !deleted_aster {
            new_asteroids.push(ast.clone());
        }
    }

    // update for player asteroid collision.
    for ast in new_state.asteroids.iter() {
        if collision::collides(&ast.bounding_box(), &new_state.player.bounding_box()) {
            new_state.game_over = true;
            break;
        }
    }

    new_state.asteroids = new_asteroids;

    if new_state.asteroids.len() == 0 {
        new_state.game_over = true;
        new_state.game_over_is_win = true;
    }
    return new_state;
}

#[cfg(feature = "gui")]
pub fn game_sdl2_render(game_state: &GameState, canvas: &mut Canvas<Window>,
                        image_resources: &ImageResources) -> () {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // put this into a asteroids specific draw function.

    let texture_creator = canvas.texture_creator();
    let mut new_texture = texture_creator.create_texture_target(texture_creator.default_pixel_format(), 150, 150).unwrap();

    canvas.with_texture_canvas(&mut new_texture, |texture_canvas| {
        texture_canvas.clear();
        for ast in game_state.asteroids.iter() {
            let dest_reg = Rect::new(
                ast.rust_sux.pos_x as i32,
                ast.rust_sux.pos_y as i32,
                ast.radius as u32,
                ast.radius as u32,
            );

            texture_canvas.copy(&image_resources.asteroids_texture,
                                None, dest_reg).unwrap();
        }
        for bull in game_state.bullets.iter() {
            let dest_reg = Rect::new(
                bull.rust_sux.pos_x as i32,
                bull.rust_sux.pos_y as i32,
                bull.radius as u32,
                bull.radius as u32,
            );
            texture_canvas.copy(&image_resources.bullet_texture, None, dest_reg).unwrap();
        }

        let player_rect = Rect::new(
            game_state.player.rust_sux.pos_x as i32,
            game_state.player.rust_sux.pos_y as i32,
            game_state.player.radius as u32,
            game_state.player.radius as u32,
        );

        texture_canvas.copy_ex(&image_resources.player_texture,
                               None, player_rect, game_state.player.rust_sux.direction * 57.29578,
                               None, false, false).unwrap();
    }).unwrap();
    canvas.copy(&new_texture, None, None).unwrap();
}    


pub fn game_update(game_state: GameState, dt: f64, game_input: &GameInput) -> GameState {
    game_state_update(game_state, dt, &game_input)
}

#[cfg(all(test, not(feature = "gui")))]
mod tests {
    use super::*;

    #[test]
    fn test_pos_zero_vec() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 0.0,
            direction: 0.0,
        };

        update_pos(&mut pos_thing, 1.0, 100.0, 100.0);
        assert_eq!(pos_thing.pos_x, 0.0);
        assert_eq!(pos_thing.pos_y, 0.0);
    }

    #[test]
    fn test_pos_vec_one_zero_dir() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 1.0,
            direction: 0.0,
        };

        update_pos(&mut pos_thing, 1.0, 100.0, 100.0);
        assert_eq!(pos_thing.pos_x, 1.0);
        assert_eq!(pos_thing.pos_y, 0.0);
    }

    #[test]
    fn test_pos_vec_one_90_dir() {
        let mut pos_thing = MoveAblePos {
            pos_x: 0.0,
            pos_y: 0.0,
            velocity: 1.0,
            direction: std::f64::consts::PI * 0.5,
        };

        update_pos(&mut pos_thing, 1.0, 100.0, 100.0);
        assert!(pos_thing.pos_x < 0.00001);
        assert!(pos_thing.pos_x > -0.0001);
        assert!(pos_thing.pos_y == 1.0);
    }

    fn test_game_shoot() {
        let game_state = game_init();

        let game_input = GameInput {
            rotation: 0.0,
            shoot: false,
            thrusters: false,
        };

        let mut new_state = game_update(&game_state, 0.1, &game_input);
        assert_eq!(new_state.bullets.len(), 0);

        // currently can't assert that bullet shoots cause it could collide with an asteroid
        // todo: update game init to take number of asteroids then can test if bullets shoot or not.
    }

    #[test]
    fn test_shoot_bullet() {
        let mut game_state = game_init();

        shoot_bullet(&mut game_state);

        assert_eq!(game_state.bullets.len(), 1);
    }
}
