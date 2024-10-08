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

fn draw_player(player: &Player, camera_pos: &Vector2, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	handle.draw_rectangle_v(
		player.pos - *camera_pos,
		player.size,
		player.color,
	);
}

fn draw_asset_on_player(
	player: &Player,
	asset: &Asset,
	camera_pos: &Vector2,
	handle: &mut raylib::core::drawing::RaylibDrawHandle
) {
	let scale = asset.info.scale;

	// TODO: get the assets size
	let asset_center = Vector2 { x: 500.0, y: 500.0 } / 2.0;
	let asset_draw_pos = player.pos - (asset_center * scale);

	handle.draw_texture_ex(
		&asset.texture,
		asset_draw_pos + asset.info.offset - *camera_pos,
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
	camera_pos: Vector2,
	player: Option<Player>,
	blocks: Vec<Block>,
}

const GRAVITY: f32 = 2750.0;

impl Scene for GameScene {
	fn init(&mut self) {
		self.blocks.push(Block {
			pos: Vector2 { x: -640.0, y: 240.0 },
			dim: Vector2 { x: 1280.0, y: 120.0 },
			color: Color { r: 125, g: 125, b: 130, a: 255 },
		});

		self.blocks.push(Block {
			pos: Vector2 { x: -560.0, y: -60.0 },
			dim: Vector2 { x: 150.0, y: 120.0 },
			color: Color {r: 128, g: 123,b: 130, a: 255},
		});


		self.blocks.push(Block {
			pos: Vector2 { x: -340.0, y: -160.0 },
			dim: Vector2 { x: 30.0, y: 40.0 },
			color: Color {r: 130, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: -40.0, y: -160.0 },
			dim: Vector2 { x: 30.0, y: 40.0 },
			color: Color {r: 130, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 260.0, y: -160.0 },
			dim: Vector2 { x: 30.0, y: 40.0 },
			color: Color {r: 130, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 460.0, y: -160.0 },
			dim: Vector2 { x: 100.0, y: 40.0 },
			color: Color {r: 100, g: 110, b: 120, a: 255},
		});

		self.blocks.push(Block {
			pos: Vector2 { x: 560.0, y: -360.0 },
			dim: Vector2 { x: 30.0, y: 240.0 },
			color: Color {r: 100, g: 110, b: 120, a: 255},
		});

		self.player = Some(Player {
			pos: Vector2 {
				x: 0.0,
				y: 0.0,
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
		if let Some(player) = &mut self.player {
			handle_player_movement(player, delta, d);

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

		let camera_pos = self.camera_pos - game_state.window_size / 2.0;

		for block in self.blocks.iter() {
			d.draw_rectangle_v(
				block.pos - camera_pos,
				block.dim,
				block.color,
			)
		}

		if let Some(player) = &self.player {
			draw_player(&player, &camera_pos, d);
			draw_asset_on_player(&player, &game_state.assets["hat"], &camera_pos, d);
			draw_asset_on_player(&player, &game_state.assets["bowtie"], &camera_pos, d);
		}
	}

}
