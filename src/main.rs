#![feature(try_from)]
extern crate ggez;
extern crate byteorder;
extern crate nphysics2d;
extern crate nalgebra as na;
extern crate ncollide_geometry;
extern crate ncollide;

use ggez::conf::Conf;
use ggez::event::{EventHandler, run};
use ggez::graphics::{DrawMode, Point2 as GgezPoint, circle, clear, line, Text, Font, draw};
use ggez::Context;
use ggez::GameResult;
use ggez::graphics::present;

use std::fs::File;
use std::io::Read;
use std::convert::TryFrom;
use ggez::event::Keycode;
use ggez::event::Mod;
use track_name::TrackName;
use nphysics2d::world::World;
use na::Vector2;
use nphysics2d::object::RigidBody;
use ncollide_geometry::shape::{Polyline, Ball2, Plane};
use std::sync::Arc;
use na::Point2;
use std::cell::RefCell;
use na::Translation2;

mod mrg;
mod level;
mod track_name;
mod path;
mod point;
mod game_state;

use mrg::Mrg;
use game_state::GameState;
use nphysics2d::object::RigidBodyHandle;
use std::borrow::BorrowMut;

pub struct App {
    pub mrg: Mrg,
    pub game_state: GameState,
    pub track_id: u64,
    pub world: RefCell<World<f64>>,
    pub previous_track_id: u64,
    pub current_track_handle: Option<RigidBodyHandle<f64>>,
    pub balls: RefCell<Vec<RigidBodyHandle<f64>>>
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

    fn update_track_physics(&mut self, ctx: &mut Context, track: Vec<GgezPoint>) {
        if self.previous_track_id != self.track_id {
            let mut world = self.world.borrow_mut();
            self.remove_track_from_world(&mut world);
//            let new_track_handle = self.add_track_to_world(&mut world, track);
//            self.current_track_handle = Some(new_track_handle);
        }
    }

    fn render_track(&self, ctx: &mut Context) -> Vec<GgezPoint> {
        let font = Font::default_font().unwrap();
        let track_name = self.get_current_track().name.as_ref();
        let text = Text::new(ctx, track_name, &font).unwrap();
        draw(ctx, &text, GgezPoint::new(0.0, 255.0), 0.0).expect("error when draw");
        let track : Vec<GgezPoint> = self.get_current_track()
            .clone()
            .path
            .as_ref()
            .unwrap()
            .clone()
            .into();
        let track : Vec<GgezPoint> = track.into_iter()
            .map(|mut item: GgezPoint| { item.y += 500.0; item })
            .collect();
        if track.len() > 1 {
            line(ctx, &track, 5.0).expect("error when draw line");
            circle(
                ctx,
                DrawMode::Fill,
                track
                    .last()
                    .expect("нет послднего поинта")
                    .clone(),
                10.0,
                1000.0,
            ).expect("error when drawing cicrle");
        }
        track
    }

    fn remove_track_from_world(&self, world: &mut World<f64>) {
        if let Some(ref track_handle) = self.current_track_handle {
            world.remove_rigid_body(track_handle);
        }
    }

    fn add_track_to_world(&self, world: &mut World<f64>, _: Vec<GgezPoint>) -> RigidBodyHandle<f64> {
        let points = vec!(Point2::new(0.0, 600.0),  Point2::new(600.0, 600.0));
        let indices = vec!(Point2::new(0, 1));
        let polyline = Polyline::new(Arc::new(points), Arc::new(indices), None, None);
        let rb = RigidBody::new_static(polyline, 0.3, 0.6);
        world.add_rigid_body(rb)
    }

    fn update_balls(&self, ctx: &mut Context) {
        for ball in self.balls.borrow().iter() {
            let (x,y) = {
                let ball = ball.borrow();
                let transform = ball.position();
                let pos = transform.translation.vector;
                let x = pos.x as f32 * 20.0;
                let y = pos.y as f32 * 20.0;
                circle(ctx, DrawMode::Fill, Point2::new(x, y), 25.0, 1.0).expect("draw circle error");
                (x,y)
            };
//            println!("transform: {:?}", transform);

//            let mut ball = ball.as_ref().borrow_mut();
//            if y > 480.0 {
//                ball.clear_forces();
//                ball.append_lin_force(Vector2::new(0.0, -1.0));
//            } else if y < 0.0 {
//                ball.clear_forces();
//                ball.append_lin_force(Vector2::new(0.0, 1.0));
//            }
        }
    }
}

impl EventHandler for App {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        clear(ctx);
        let track = self.render_track(ctx);
        self.update_track_physics(ctx, track);
        self.update_balls(ctx);
        let world = self.world.get_mut();
        world.step(0.01);
        present(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, _context: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
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
            },
            Keycode::Space => {
                for ball in self.balls.borrow().iter() {
                    let mut ball = ball.as_ref().borrow_mut();
                    use nphysics2d::math::Orientation;
                    ball.apply_angular_momentum(Orientation::new(25.0));
                }
            },
            Keycode::D => {
//                println!("add ball to world");
                let mut balls = self.balls.borrow_mut();
                let ball = Ball2::new(1.25);
                let mut ball = RigidBody::new_dynamic(ball, 1.0, 0.3, 0.6);
                ball.append_translation(&Translation2::new(50.0, 20.1));
                let mut world = self.world.get_mut();
                balls.push(world.add_rigid_body(ball));
            }
            _ => {}
        }
    }
}

pub fn main() {
    let mut world = World::new();
    world.set_gravity(Vector2::new(0.0, 9.81));
    let mut c = Conf::new();
    c.window_mode.width = 1920;
    c.window_mode.height = 1080;
    let ctx = &mut Context::load_from_conf("eventloop", "loomaclin", c).unwrap();
    let mut file = File::open("levels.mrg").expect(
        "Не удалось открыть файл с описанием уровней",
    );
    let mut content = Vec::new();
    file.read_to_end(&mut content).expect(
        "Не удалось прочесть файл в вектор",
    );
    let mut app = App {
        mrg: Mrg::try_from(content).expect("Не удалось получить уровни из указанного файла"),
        game_state: game_state::GameState::Initial,
        track_id: 0,
        world: RefCell::new(world),
        previous_track_id: 0,
        current_track_handle: None,
        balls: RefCell::new(Vec::new()),
    };
    run(ctx, &mut app).expect("error when run app");
    println!(
        "App: {:?}",
        app.mrg.levels.first().unwrap().tracks.get(5).unwrap()
    );
}
