use ggez::event::EventHandler;
use ggez::graphics::Rect;
use ggez::input::keyboard;

const SCREEN_HEIGHT: f32 = 600.;
const SCREEN_WIDTH: f32 = 600.;

const X_OFFSET: f32 = 20.;
const PADDLE_WIDTH: f32 = 12.;
const PADDLE_HEIGHT: f32 = 75.;
const PADDLE_SPEED: f32 = 8.0;

const BALL_RADIUS: f32 = 10.;
const MIN_VEL: f32 = 3.0;
const MAX_VEL: f32 = 5.0;

//We want to store all the data Pong needs here
struct MainState {
    l_paddle: Rect,
    r_paddle: Rect,
    ball: Ball,
    l_score: u16,
    r_score: u16,
}

type Vector = ggez::mint::Vector2<f32>;

struct Ball {
    rect: Rect,
    vel: Vector,
}

impl Ball {
    fn new() -> Self {
        use rand::{thread_rng, Rng};

        let mut rng = thread_rng();
        let mut x_vel = rng.gen_range(MIN_VEL, MAX_VEL);
        let mut y_vel = rng.gen_range(MIN_VEL, MAX_VEL);

        if rng.gen::<bool>() {
            x_vel *= -1.0;
        }
        if rng.gen::<bool>() {
            y_vel *= -1.0;
        }

        Ball {
            rect: Rect::new(
                SCREEN_WIDTH / 2.0 - BALL_RADIUS / 2.0,
                SCREEN_HEIGHT / 2.0 - BALL_RADIUS / 2.0,
                BALL_RADIUS,
                BALL_RADIUS,
            ),
            vel: Vector { x: x_vel, y: y_vel },
        }
    }
}

fn draw_rectangle(ctx: &mut ggez::Context, rect: &Rect) -> () {
    use ggez::graphics;
    use ggez::graphics::Color;
    let rect_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        *rect,
        Color::new(1.0, 1.0, 1.0, 1.0),
    )
    .expect("error creating mesh");
    graphics::draw(ctx, &rect_mesh, graphics::DrawParam::default())
        .expect("error drawing mesh")
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::W) {
            self.l_paddle.y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::S) {
            self.l_paddle.y += PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Up) {
            self.r_paddle.y -= PADDLE_SPEED;
        }
        if keyboard::is_key_pressed(ctx, keyboard::KeyCode::Down) {
            self.r_paddle.y += PADDLE_SPEED;
        }

        self.ball.rect.translate(self.ball.vel);

        // ball paddle collisions
        if (self.ball.vel.x < 0.0 && self.ball.rect.overlaps(&self.l_paddle))
            || (self.ball.vel.x > 0.0 && self.ball.rect.overlaps(&self.r_paddle))
        {
            self.ball.vel.x *= -1.0;
        }

        // ball top/bottom collisions
        if (self.ball.vel.y < 0.0 && self.ball.rect.top() < 0.0)
            || (self.ball.vel.y > 0.0 && self.ball.rect.bottom() > SCREEN_HEIGHT)
        {
            self.ball.vel.y *= -1.0;
        }

        // score keeping
        if self.ball.rect.left() < 0.0 {
            self.r_score += 1;
            std::thread::sleep(std::time::Duration::from_millis(1000));
            self.ball = Ball::new();
        }
        if self.ball.rect.right() > SCREEN_WIDTH {
            self.l_score += 1;
            std::thread::sleep(std::time::Duration::from_millis(1000));
            self.ball = Ball::new();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        use ggez::graphics;
        use ggez::graphics::Color;
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        //all the drawing stuff goes here
        draw_rectangle(ctx, &self.ball.rect);
        draw_rectangle(ctx, &self.r_paddle);
        draw_rectangle(ctx, &self.l_paddle);

        // scoreboard drawing
        let mut scoreboard_text =
            graphics::Text::new(format!("L: {} \t R: {}", self.l_score, self.r_score));
        scoreboard_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));
        let coords = [
            SCREEN_WIDTH / 2.0 - scoreboard_text.width(ctx) as f32 / 2.0,
            10.0,
        ];
        let params = graphics::DrawParam::default().dest(coords);
        graphics::draw(ctx, &scoreboard_text, params).expect("error drawing scoreboard text");

        graphics::present(ctx).expect("error presenting");
        Ok(())
    }
}

fn main() -> ggez::GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Pong", "Ibrahim Moreno")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .unwrap();

    let main_state = &mut MainState {
        l_paddle: Rect::new(
            X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        r_paddle: Rect::new(
            SCREEN_WIDTH - X_OFFSET,
            SCREEN_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0,
            PADDLE_WIDTH,
            PADDLE_HEIGHT,
        ),
        ball: Ball::new(),
        l_score: 0,
        r_score: 0,
    };

    ggez::event::run(ctx, event_loop, main_state)
}
