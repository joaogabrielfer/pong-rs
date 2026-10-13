use raylib::{
    RaylibHandle, RaylibThread,
    drawing::RaylibDraw,
    ffi::{Color, KeyboardKey},
};

const BALL_SIZE: i32 = 30;
const PADLE_DISTANCE: i32 = 30;
const PADLE_WIDTH: i32 = 25;
const PADLE_HEIGHT: i32 = 125;
const PADLE_SPEED: i32 = 500;

struct Ball {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

impl Ball {
    fn with_speed(rl: &mut RaylibHandle) -> Self {
        Ball {
            x: (rl.get_render_width() / 2 - BALL_SIZE / 2) as f32,
            y: (rl.get_render_height() / 2 - BALL_SIZE / 2) as f32,
            dx: 500.,
            dy: 500.,
        }
    }
    fn still(rl: &mut RaylibHandle) -> Self {
        Ball {
            x: (rl.get_render_width() / 2 - BALL_SIZE / 2) as f32,
            y: (rl.get_render_height() / 2 - BALL_SIZE / 2) as f32,
            dx: 0.,
            dy: 0.,
        }
    }
}

#[derive(PartialEq)]
enum GameState {
    Start,
    Running,
    PointHappened,
    GameOver(Player),
}

#[derive(PartialEq, Clone, Copy)]
enum Player {
    Player1,
    Player2,
}

struct Game {
    state: GameState,
    ball: Ball,
    p1_points: u32,
    p2_points: u32,
    padle_1_y: f32,
    padle_2_y: f32,
}

fn main() {
    let (mut rl, thread) = raylib::init().title("pong").width(900).height(720).build();
    rl.set_target_fps(165);

    let mut g = Game {
        state: GameState::Start,
        ball: Ball::with_speed(&mut rl),
        p1_points: 0,
        p2_points: 0,
        padle_1_y: rl.get_render_height() as f32 / 2.,
        padle_2_y: rl.get_render_height() as f32 / 2.,
    };

    while !rl.window_should_close() {
        match g.state {
            GameState::Start => render_start_state(&mut rl, &mut g, &thread),
            GameState::Running | GameState::PointHappened => {
                render_game_state(&mut rl, &mut g, &thread)
            }
            GameState::GameOver(player) => render_game_over_state(&mut rl, &mut g, &thread, player),
        }
        if g.p1_points >= 5 {
            g.state = GameState::GameOver(Player::Player1)
        }
        if g.p2_points >= 5 {
            g.state = GameState::GameOver(Player::Player2)
        }
    }
}

fn render_start_state(rl: &mut RaylibHandle, g: &mut Game, thread: &RaylibThread) {
    if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        g.state = GameState::Running
    }
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::new(18, 18, 18, 255));
    d.draw_text("press space to begin", 0, 0, 48, Color::WHITE);
}

fn render_game_state(rl: &mut RaylibHandle, g: &mut Game, thread: &RaylibThread) {
    let dt = rl.get_frame_time();

    let x_offset = g.ball.dx * dt;
    let y_offset = g.ball.dy * dt;

    if x_offset + g.ball.x + BALL_SIZE as f32 >= (rl.get_render_width() as f32) {
        // player 1 scored
        g.state = GameState::PointHappened;
        g.p1_points += 1;
        g.ball = Ball::still(rl);
    } else if x_offset + g.ball.x < 0. {
        // player 2 scored
        g.state = GameState::PointHappened;
        g.p2_points += 1;
        g.ball = Ball::still(rl);
    } else {
        if g.state == GameState::Running {
            g.ball.x += x_offset;
        }
    }

    if y_offset + g.ball.y + BALL_SIZE as f32 >= (rl.get_render_height() as f32)
        || y_offset + g.ball.y < 0.
    {
        g.ball.dy *= -1.;
    } else {
        if g.state == GameState::Running {
            g.ball.y += y_offset;
        }
    }

    if rl.is_key_down(KeyboardKey::KEY_W) && (g.padle_1_y as i32) > PADLE_HEIGHT / 2 {
        g.padle_1_y -= PADLE_SPEED as f32 * dt
    }
    if rl.is_key_down(KeyboardKey::KEY_S)
        && (g.padle_1_y as i32) < rl.get_render_height() - PADLE_HEIGHT / 2
    {
        g.padle_1_y += PADLE_SPEED as f32 * dt
    }

    if rl.is_key_down(KeyboardKey::KEY_K) && (g.padle_2_y as i32) > PADLE_HEIGHT / 2 {
        g.padle_2_y -= PADLE_SPEED as f32 * dt
    }
    if rl.is_key_down(KeyboardKey::KEY_J)
        && (g.padle_2_y as i32) < rl.get_render_height() - PADLE_HEIGHT / 2
    {
        g.padle_2_y += PADLE_SPEED as f32 * dt
    }

    if g.state == GameState::PointHappened && rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
        g.ball = Ball::with_speed(rl);
        g.state = GameState::Running;
    }

    if (g.ball.x as i32 + BALL_SIZE) >= rl.get_render_width() - PADLE_DISTANCE - PADLE_WIDTH / 2
        && g.ball.y >= g.padle_2_y - PADLE_HEIGHT as f32
        && g.ball.y < g.padle_2_y + PADLE_HEIGHT as f32
    {
        g.ball.dx *= -1.;
    }

    if (g.ball.x as i32 - BALL_SIZE) < PADLE_DISTANCE
        && g.ball.y >= g.padle_1_y - PADLE_HEIGHT as f32
        && g.ball.y < g.padle_1_y + PADLE_HEIGHT as f32
    {
        g.ball.dx *= -1.;
    }

    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::new(18, 18, 18, 255));

    d.draw_text(
        &format!("{}", g.p1_points),
        d.get_render_width() / 2 - PADLE_DISTANCE,
        PADLE_DISTANCE,
        30,
        Color::WHITE,
    );

    d.draw_text(
        &format!("{}", g.p2_points),
        d.get_render_width() / 2 + PADLE_DISTANCE - 15,
        PADLE_DISTANCE,
        30,
        Color::WHITE,
    );

    // draw padle 1
    d.draw_rectangle(
        PADLE_DISTANCE,
        (g.padle_1_y as i32) - PADLE_HEIGHT / 2,
        PADLE_WIDTH,
        PADLE_HEIGHT,
        Color::WHITE,
    );

    // draw padle 2
    d.draw_rectangle(
        d.get_render_width() - PADLE_DISTANCE - PADLE_WIDTH,
        (g.padle_2_y as i32) - PADLE_HEIGHT / 2,
        PADLE_WIDTH,
        PADLE_HEIGHT,
        Color::WHITE,
    );

    // draw ball
    d.draw_rectangle(
        g.ball.x as i32,
        g.ball.y as i32,
        BALL_SIZE,
        BALL_SIZE,
        Color::WHITE,
    );

    // draw middle line
    d.draw_line(
        d.get_render_width() / 2,
        0,
        d.get_render_width() / 2,
        d.get_render_height(),
        Color::WHITE,
    );
}

fn render_game_over_state(
    rl: &mut RaylibHandle,
    g: &mut Game,
    thread: &RaylibThread,
    player: Player,
) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::new(18, 18, 18, 255));
    d.draw_text(
        &format!(
            "player {} won",
            if player == Player::Player1 { "1" } else { "2" }
        ),
        0,
        0,
        48,
        Color::WHITE,
    );
}
