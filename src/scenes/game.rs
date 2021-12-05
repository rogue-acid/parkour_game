use raylib::prelude::*;

use crate::{ GameState, Scene, SceneType };

struct Player {
	pos: Vector2,
	size: Vector2,
	color:  Color,
	velocity: Vector2,
	controls: Controls,
	movement_speed: f32,
	jump_power: f32,
}

struct Block {
	pos: Vector2,
	dim: Vector2,
	color:  Color,
}

impl Collidable for Player {
	fn get_physics_object(&self) -> PhysicsObject {
		PhysicsObject {
			pos: self.pos.clone(),
			dim: self.size.clone(),
		}
	}

	fn is_colliding(&self, other: &dyn Collidable) -> bool {
		self.get_physics_object().is_colliding(other)
	}
}

impl Collidable for Block {
	fn get_physics_object(&self) -> PhysicsObject {
		PhysicsObject {
			pos: self.pos.clone(),
			dim: self.dim.clone(),
		}
	}

	fn is_colliding(&self, other: &dyn Collidable) -> bool {
		self.get_physics_object().is_colliding(other)
	}
}

struct Controls {
	move_up: KeyboardKey,
	move_down: KeyboardKey,
	move_left: KeyboardKey,
	move_right: KeyboardKey,
	jump: KeyboardKey,
}

fn draw_player(player: &Player, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	handle.draw_rectangle_v(
		player.pos,
		player.size,
		player.color,
	);
}

#[derive(Clone)]
struct PhysicsObject {
	pos: Vector2,
	dim: Vector2,
}

impl PhysicsObject {
	fn contains_point(&self, point: &Vector2) -> bool {
		point.x > self.pos.x &&
		point.x < self.pos.x + self.dim.x &&
		point.y > self.pos.y &&
		point.y < self.pos.y + self.dim.y
	}

	fn corners(&self) -> [Vector2; 4] {
		[
			Vector2 { x: self.pos.x,              y: self.pos.y              },
			Vector2 { x: self.pos.x + self.dim.x, y: self.pos.y              },
			Vector2 { x: self.pos.x,              y: self.pos.y + self.dim.y },
			Vector2 { x: self.pos.x + self.dim.x, y: self.pos.y + self.dim.y },
		]
	}
}

trait Collidable {
	fn get_physics_object(&self) -> PhysicsObject;
	fn is_colliding(&self, other: &dyn Collidable) -> bool;
}

impl Collidable for PhysicsObject {
	fn get_physics_object(&self) -> PhysicsObject {
		self.clone()
	}

	fn is_colliding(&self, other: &dyn Collidable) -> bool {
		let other = other.get_physics_object();

		let mut is_colliding = false;

		for corner in self.corners() {
			if is_colliding {
				break
			}

			if other.contains_point(&corner) {
				is_colliding = true
			}
		}

		for corner in other.corners() {
			if is_colliding {
				break
			}

			if self.contains_point(&corner) {
				is_colliding = true
			}
		}

		is_colliding
	}
}

fn handle_player_movement(player: &mut Player, delta: f32, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	if handle.is_key_pressed(player.controls.jump) {
		player.velocity.y = -player.jump_power * delta
	}
	if handle.is_key_down(player.controls.move_left) {
		player.velocity.x = -player.movement_speed;
	} else if handle.is_key_down(player.controls.move_right) {
		player.velocity.x = player.movement_speed;
	} else {
		player.velocity.x = 0.0;
	}
}

#[derive(Default)]
pub struct GameScene {
	players: Vec<Player>,
	blocks: Vec<Block>,
}

const GRAVITY: f32 = 2750.0;

impl Scene for GameScene {
	fn init(&mut self) {
		self.blocks.push(Block {
			pos: Vector2 { x: 0.0, y: 600.0 },
			dim: Vector2 { x: 1280.0, y: 120.0 },
			color: Color { r: 8, g: 255, b: 65, a: 255 },
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 70.0, y: 300.0 },
			dim: Vector2 { x: 350.0, y: 120.0 },
			color: Color::PURPLE,
		});

		self.players.push(Player {
			pos: Vector2 {
				x: 630.0,
				y: 322.0,
			},
			size: Vector2 {
				x: 20.0,
				y: 75.0,
			},
			color: Color { r: 232, g: 190, b: 172, a: 255 },
			velocity: Vector2 {
				x: 0.0,
				y: 0.0
			},
			controls: Controls {
				move_up: KeyboardKey::KEY_W,
				move_down: KeyboardKey::KEY_S,
				move_left: KeyboardKey::KEY_A,
				move_right: KeyboardKey::KEY_D,
				jump: KeyboardKey::KEY_SPACE,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
		});

		self.players.push(Player {
			pos: Vector2 {
				x: 630.0,
				y: 322.0,
			},
			size: Vector2 {
				x: 20.0,
				y: 75.0,
			},
			color: Color { r: 232, g: 190, b: 172, a: 255 },
			velocity: Vector2 {
				x: 0.0,
				y: 0.0
			},
			controls: Controls {
				move_up: KeyboardKey::KEY_UP,
				move_down: KeyboardKey::KEY_DOWN,
				move_left: KeyboardKey::KEY_LEFT,
				move_right: KeyboardKey::KEY_RIGHT,
				jump: KeyboardKey::KEY_RIGHT_CONTROL,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
		});
	}

	fn update(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState, delta: f32) {
		for mut player in self.players.iter_mut() {
			handle_player_movement(&mut player, delta, d);

			player.velocity.y += GRAVITY * delta;

			player.pos.y += player.velocity.y * delta;
			for block in self.blocks.iter() {
				if player.is_colliding(block) {
					player.pos.y -= player.velocity.y * delta;
					player.velocity.y = 0.0;
				}
			}

			player.pos.x += player.velocity.x * delta;
			for block in self.blocks.iter() {
				if player.is_colliding(block) {
					player.pos.x -= player.velocity.x * delta;
					player.velocity.x = 0.0;
				}
			}
		}

		if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
			game_state.current_scene = SceneType::GamePauseMenu;
		}
	}

	fn display(&mut self, d: &mut RaylibDrawHandle, game_state: &mut GameState) {
		d.clear_background(Color { r: 43, g: 255, b: 241, a: 255 });

		for block in self.blocks.iter() {
			d.draw_rectangle_v(
				block.pos,
				block.dim,
				block.color,
			)
		}

		let player_1 = &self.players[0];
		let player_2 = &self.players[1];

		//player 1
		draw_player(&player_1, d);

		let scale = 0.15;
		d.draw_texture_ex(
			&game_state.assets["hat"],
			player_1.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 10.0, y: 0.0 },
			0.0,
			scale,
			Color::WHITE
		);


		let scale = 0.075;
		d.draw_texture_ex(
			&game_state.assets["bowtie"],
			player_1.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 12.0, y: 40.0 },
			0.0,
			scale,
			Color::WHITE
		);

		// player 2
		let scale = 0.2;
		d.draw_texture_ex(
			&game_state.assets["branch"],
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 12.0, y: 40.0 },
			0.0,
			scale,
			Color::WHITE
		);

		let scale = 0.2;
		d.draw_texture_ex(
			&game_state.assets["bird"],
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 25.0, y: 22.0 },
			0.0,
			scale,
			Color::WHITE
		);

		draw_player(&player_2, d);

		let scale = 0.4;
		d.draw_texture_ex(
			&game_state.assets["pirate_hat"],
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 8.0, y: 0.0 },
			0.0,
			scale,
			Color::WHITE
		);

	}

}
