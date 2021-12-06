use raylib::prelude::*;

use crate::{ GameState, Scene, SceneType, Asset };

struct Player {
	pos: Vector2,
	size: Vector2,
	color:  Color,
	velocity: Vector2,
	controls: Controls,
	movement_speed: f32,
	jump_power: f32,
	can_jump: bool,
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

enum Controls {
	Keyboard {
		move_up: KeyboardKey,
		move_down: KeyboardKey,
		move_left: KeyboardKey,
		move_right: KeyboardKey,
		jump: KeyboardKey,
	},
	GamePad {
		id: i32,
		jump: GamepadButton,
		deadzone_amount: f32,
		move_left_right: GamepadAxis,
	}
}

fn draw_player(player: &Player, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	handle.draw_rectangle_v(
		player.pos,
		player.size,
		player.color,
	);
}

fn draw_asset_on_player(player: &Player, asset: &Asset, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	let scale = asset.info.scale;

	// TODO: get the assets size
	let asset_center = Vector2 { x: 500.0, y: 500.0 } / 2.0;
	let asset_draw_pos = player.pos - (asset_center * scale);

	handle.draw_texture_ex(
		&asset.texture,
		asset_draw_pos + asset.info.offset,
		0.0,
		scale,
		Color::WHITE
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
	match player.controls {
		Controls::Keyboard { jump, move_left, move_right, .. } => {
			if handle.is_key_down(jump) && player.can_jump {
				player.can_jump = false;
				player.velocity.y = -player.jump_power * delta;
			}

			if handle.is_key_down(move_left) {
				player.velocity.x = -player.movement_speed;
			} else if handle.is_key_down(move_right) {
				player.velocity.x = player.movement_speed;
			} else {
				player.velocity.x = 0.0;
			}
		},

		Controls::GamePad { id, jump, move_left_right, deadzone_amount, .. } => {
			if handle.is_gamepad_button_down(id, jump) {
				player.velocity.y = -player.jump_power * delta
			}

			let left_right_amount = handle.get_gamepad_axis_movement(id, move_left_right);

			if f32::abs(left_right_amount) > deadzone_amount {
				player.velocity.x = player.movement_speed * left_right_amount;
			} else {
				player.velocity.x = 0.0;
			}
		}
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

		// grass
		// self.blocks.push(Block {
		// 	pos: Vector2 { x: 0.0, y: 600.0 },
		// 	dim: Vector2 { x: 1280.0, y: 120.0 },
		// 	color: Color { r: 8, g: 255, b: 65, a: 255 },
		// });

		// rock
		self.blocks.push(Block {
			pos: Vector2 { x: 0.0, y: 600.0 },
			dim: Vector2 { x: 1280.0, y: 120.0 },
			color: Color { r: 125, g: 125, b: 130, a: 255 },
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 80.0, y: 300.0 },
			dim: Vector2 { x: 150.0, y: 120.0 },
			color: Color {r: 128, g: 123,b: 130, a: 255},
		});


		self.blocks.push(Block {
			pos: Vector2 { x: 300.0, y: 200.0 },
			dim: Vector2 { x: 30.0, y: 40.0 },
			color: Color {r: 130, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 600.0, y: 200.0 },
			dim: Vector2 { x: 30.0, y: 40.0 },
			color: Color {r: 130, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 900.0, y: 200.0 },
			dim: Vector2 { x: 30.0, y: 40.0 },
			color: Color {r: 130, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 1100.0, y: 200.0 },
			dim: Vector2 { x: 100.0, y: 40.0 },
			color: Color {r: 100, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 1200.0, y: 0.0 },
			dim: Vector2 { x: 30.0, y: 240.0 },
			color: Color {r: 100, g: 110, b: 120, a: 255},
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
			controls: Controls::GamePad {
				// TODO: this should be auto detected
				id: 0,
				jump: GamepadButton::GAMEPAD_BUTTON_RIGHT_FACE_DOWN,
				deadzone_amount: 0.14,
				move_left_right: GamepadAxis::GAMEPAD_AXIS_LEFT_X,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
			can_jump: true,
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
			controls: Controls::Keyboard {
				move_up: KeyboardKey::KEY_UP,
				move_down: KeyboardKey::KEY_DOWN,
				move_left: KeyboardKey::KEY_LEFT,
				move_right: KeyboardKey::KEY_RIGHT,
				jump: KeyboardKey::KEY_RIGHT_CONTROL,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
			can_jump: true,
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
			controls: Controls::Keyboard {
				move_up: KeyboardKey::KEY_W,
				move_down: KeyboardKey::KEY_S,
				move_left: KeyboardKey::KEY_A,
				move_right: KeyboardKey::KEY_D,
				jump: KeyboardKey::KEY_SPACE,
			},
			movement_speed: 800.0,
			jump_power: 400000.0,
			can_jump: true,
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
					if player.velocity.y > 0.0 {
						player.can_jump = true;
					}
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
		draw_player(&player_1, d);
		draw_asset_on_player(&player_1, &game_state.assets["hat"], d);
		draw_asset_on_player(&player_1, &game_state.assets["bowtie"], d);

		let player_2 = &self.players[1];
		draw_asset_on_player(&player_2, &game_state.assets["branch"], d);
		draw_asset_on_player(&player_2, &game_state.assets["bird"], d);
		draw_player(&player_2, d);
		draw_asset_on_player(&player_2, &game_state.assets["pirate_hat"], d);

		let player_3 = &self.players[2];
		draw_player(&player_3, d);
	}

}
