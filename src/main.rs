#![feature(try_from)]
extern crate ggez;
extern crate byteorder;
extern crate nphysics2d;
extern crate nalgebra as na;

use ggez::conf::Conf;
use ggez::event::{EventHandler, run};
use ggez::graphics::{DrawMode, Point, circle, clear, line, Text, Font, draw};
use ggez::Context;
use ggez::GameResult;
use ggez::graphics::present;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::convert::TryFrom;
use ggez::event::Keycode;
use ggez::event::Mod;
use track_name::TrackName;
use nphysics2d::world::World;
use na::Vector2;
use nphysics2d::object::RigidBody;

mod mrg;
mod level;
mod track_name;
mod path;
mod point;
mod game_state;

use mrg::Mrg;
use game_state::GameState;



#[derive(Debug)]
pub struct App {
    pub mrg: Mrg,
    pub game_state: GameState,
    pub track_id: u64,
}

impl App {
    fn get_current_track(&self) -> &TrackName {
        self.mrg
            .levels
            .first()
            .as_ref()
            .unwrap()
            .tracks
            .get(self.track_id as usize)
            .as_ref()
            .unwrap()
    }

    fn render_track(&self, ctx: &mut Context) -> Vec<Point> {
        self.get_current_track()
            .clone()
            .path
            .as_ref()
            .unwrap()
            .clone()
            .into()
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context, dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx);
        let mut world = World::new();
        world.set_gravity(Vector2::new(0.0, 9.81));
        let track = self.render_track(ctx);
        line(ctx, &track);
        circle(
            ctx,
            DrawMode::Fill,
            track
                .last()
                .expect("нет послднего поинта")
                .clone(),
            10.0,
            10,
        );
        let font = Font::default_font().unwrap();
        let track_name = self.get_current_track().name.as_ref();
        let text = Text::new(ctx, track_name, &font).unwrap();
        draw(ctx, &text, Point { x: 400.0, y: 400.0 }, 0.0);
        present(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => {
                if self.track_id + 1 < self.mrg.levels.first().unwrap().tracks.len() as u64 {
                    self.track_id += 1;
                }
            }
            Keycode::S => {
                if self.track_id > 0 {
                    self.track_id -= 1;
                }
            }
            _ => {}
        }
    }
}

pub fn main() {
    let mut c = Conf::new();
    c.window_height = 1000;
    c.window_width = 1900;
    let mut ctx = &mut Context::load_from_conf("eventloop", "ggez", c).unwrap();
    let mut file = File::open("levels.mrg").expect(
        "Не удалось открыть файл с описанием уровней",
    );
    let mut content = Vec::new();
    let content_length = file.read_to_end(&mut content).expect(
        "Не удалось прочесть файл в вектор",
    );
    let mut app = App {
        mrg: Mrg::try_from(content).expect("Не удалось получить уровни из указанного файла"),
        game_state: game_state::GameState::Initial,
        track_id: 0,
    };
    run(ctx, &mut app);
    println!(
        "App: {:?}",
        app.mrg.levels.first().unwrap().tracks.get(5).unwrap()
    );
}
