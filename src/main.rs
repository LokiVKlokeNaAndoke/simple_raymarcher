use arr_macro::arr;
use ggez::conf;
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::timer;
use ggez::{Context, ContextBuilder, GameResult};
use num;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Raymarcher", "Dmitry")
        .window_setup(conf::WindowSetup {
            title: "Fractal Raymarcher".to_owned(),
            samples: conf::NumSamples::Zero,
            vsync: true,
            transparent: false,
            icon: "".to_owned(),
            srgb: true,
        })
        .window_mode(conf::WindowMode {
            width: 800.0,
            height: 600.0,
            maximized: false,
            fullscreen_type: conf::FullscreenType::Windowed,
            borderless: false,
            min_width: 0.0,
            max_width: 0.0,
            min_height: 0.0,
            max_height: 0.0,
            hidpi: false,
            resizable: false,
        })
        .build()
        .expect("Couldn't initialize context");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = MyGame::new();

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly"),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct MyGame {
    fps_text_cached: [graphics::Text; 99],
}

impl MyGame {
    fn new() -> Self {
        let mut cached_fps_text = arr![graphics::Text::default();99];
        for (i, item) in cached_fps_text.iter_mut().enumerate() {
            let font = graphics::Font::default();
            let text = graphics::Text::new((i.to_string(), font, 24.0));
            *item = text;
        }

        MyGame {
            fps_text_cached: cached_fps_text,
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(
            ctx,
            [
                (timer::time_since_start(ctx).as_millis() as f32 / 1000. + 3.14 / 2.).cos() / 2.
                    + 0.5,
                (timer::time_since_start(ctx).as_millis() as f32 / 1000.).sin() / 2. + 0.5,
                0.,
                1.,
            ]
            .into(),
        );

        graphics::draw(
            ctx,
            &self.fps_text_cached[num::clamp(timer::fps(ctx).round() as usize, 0, 99)],
            graphics::DrawParam::new()
                .color((1., 1., 1., 1.).into())
                .dest(Point2::new(0., 0.)),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}
