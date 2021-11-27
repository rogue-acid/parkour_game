use raylib::prelude::*;

struct Controls {
	move_up: KeyboardKey,
	move_down: KeyboardKey,
	move_left: KeyboardKey,
	move_right: KeyboardKey,
	jump: KeyboardKey,
}

struct Player {
	pos: Vector2,
	width: f32,
	height: f32,
	color:  Color,
	velocity: Vector2,
	controls: Controls,
	movement_speed: f32,
	jump_power: f32,
}

struct Clothes {
	model: Vec<Vector2>,
	offset: Vector2,
}

fn draw_rectangle(player: &Player, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	handle.draw_rectangle(
		player.pos.x as i32,
		player.pos.y as i32,
		player.width as i32,
		player.height as i32,
		player.color,
	);
}

fn handle_player_movement(player: &mut Player, delta: f32, handle: &mut raylib::core::drawing::RaylibDrawHandle) {
	if handle.is_key_pressed(player.controls.jump) {
		player.velocity.y = -player.jump_power * delta
	}
	if handle.is_key_down(player.controls.move_left) {
		player.pos.x -= player.movement_speed * delta
	}
	if handle.is_key_down(player.controls.move_right) {
		player.pos.x += player.movement_speed * delta
	}
}

fn main() {
	let (mut rl, thread) = raylib::init()
		.size(1280, 720)
		.title("Hello, World")
		.build();

	rl.set_target_fps(300);


	let hat_texture = rl.load_texture(&thread, "hat.png").unwrap();
	let bowtie_texture = rl.load_texture(&thread, "bowtie.png").unwrap();
	let pirate_hat_texture = rl.load_texture(&thread, "pirate_hat.png").unwrap();

	let mut player_1 = Player {
		pos: Vector2 {
			x: 630.0,
			y: 322.0,
		},
		width: 20.0,
		height: 75.0,
		color: Color { r: 232, g: 190, b: 172, a: 200 },
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
	};

	let mut player_2 = Player {
		pos: Vector2 {
			x: 630.0,
			y: 322.0,
		},
		width: 20.0,
		height: 75.0,
		color: Color { r: 232, g: 190, b: 172, a: 200 },
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
	};

	let gravity = 2750.0;

	while !rl.window_should_close() {
		let delta = rl.get_frame_time();
		let mut d = rl.begin_drawing(&thread);

		//ground
		d.draw_rectangle(
			0,
			600,
			1280,
			120,
			Color { r: 8, g: 255, b: 65, a: 255 },
		);

		d.clear_background(Color { r: 43, g: 255, b: 241, a: 0 });


		handle_player_movement(&mut player_1, delta, &mut d);
		handle_player_movement(&mut player_2, delta, &mut d);

		player_1.pos += player_1.velocity * delta;
		player_1.velocity.y += gravity * delta;

		if player_1.pos.y + player_1.height > 600.0 {
			player_1.pos.y = 600.0 - player_1.height;
			player_1.velocity.y = 0.0;
		}

		player_2.pos += player_2.velocity * delta;
		player_2.velocity.y += gravity * delta;

		if player_2.pos.y + player_2.height > 600.0 {
			player_2.pos.y = 600.0 - player_2.height;
			player_2.velocity.y = 0.0;
		}

		//players
		draw_rectangle(&player_1, &mut d);
		draw_rectangle(&player_2, &mut d);

		//player 1
		let scale = 0.15;
		d.draw_texture_ex(
			&hat_texture,
			player_1.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 10.0, y: 0.0 },
			0.0,
			scale,
			Color::WHITE
		);

		
		let scale = 0.075;
		d.draw_texture_ex(
			&bowtie_texture,
			player_1.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 30.0, y: 55.0 },
			0.0,
			scale,
			Color::WHITE
		);

		// player 2
		let scale = 0.4;
		d.draw_texture_ex(
			&pirate_hat_texture,
			player_2.pos - (Vector2 { x: 500.0, y: 500.0 } / 2.0 * scale) + Vector2 { x: 8.0, y: 0.0 },
			0.0,
			scale,
			Color::WHITE
		);
	}
}

/*
fn get_screen_size(handle: &mut raylib::RaylibHandle) -> Vector2 {
	Vector2 {
		x: handle.get_screen_width() as f32,
		y: handle.get_screen_height() as f32,
	}
}
*/
