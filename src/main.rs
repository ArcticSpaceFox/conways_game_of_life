mod cell;
mod grid;
mod point;

use std::borrow::BorrowMut;

use crate::grid::Grid;
use crate::point::Point;
use clap::{App, Arg};

use ggez::event::EventHandler;
use ggez::graphics::{self, Color};
use ggez::{conf, event};
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use rayon::prelude::*;

const GRID: bool = false;
//const CELL_SIZE: f32 = SCREEN_SIZE.0 / GRID_WIDTH as f32;

#[allow(dead_code)]
const BLINKER: [(usize, usize); 3] = [(4, 4), (4, 5), (4, 6)];
#[allow(dead_code)]
const TOAD: [(usize, usize); 6] = [(4, 4), (4, 5), (4, 6), (5, 3), (5, 4), (5, 5)];
#[allow(dead_code)]
const GLIDER: [(usize, usize); 5] = [(1, 2), (3, 2), (2, 3), (3, 3), (2, 4)];
#[allow(dead_code)]
const GLIDER_GUN: [(usize, usize); 36] = [
    (5, 1),
    (5, 2),
    (6, 1),
    (6, 2),
    (5, 11),
    (6, 11),
    (7, 11),
    (4, 12),
    (3, 13),
    (3, 14),
    (8, 12),
    (9, 13),
    (9, 14),
    (6, 15),
    (4, 16),
    (5, 17),
    (6, 17),
    (7, 17),
    (6, 18),
    (8, 16),
    (3, 21),
    (4, 21),
    (5, 21),
    (3, 22),
    (4, 22),
    (5, 22),
    (2, 23),
    (6, 23),
    (1, 25),
    (2, 25),
    (6, 25),
    (7, 25),
    (3, 35),
    (4, 35),
    (3, 36),
    (4, 36),
];
#[allow(dead_code)]
const BEACON: [(usize, usize); 6] = [(1, 1), (2, 1), (1, 2), (4, 3), (3, 4), (4, 4)];

/// Config for the start of the game
#[derive(Debug, Clone)]
pub struct Config {
    pub grid_width: usize,
    pub grid_height: usize,
    pub cell_size: f32,
    pub screen_size: (f32, f32),
    pub fps: u32,
    pub initial_state: String,
}

struct MainState {
    grid: Grid,
    config: Config,
}
impl MainState {
    pub fn new(_ctx: &mut Context, config: Config) -> Self {
        // Initialize the grid based on configuration
        let mut grid = Grid::new(config.grid_width, config.grid_height);
        // Initialize starting configuration
        // let mut start_cells_coords: Vec<Point> = vec![];
        let start_cells_coords = match &config.initial_state[..] {
            "glider-gun" => GLIDER_GUN.iter().map(|&p| p.into()).collect::<Vec<Point>>(),
            "toad" => TOAD.iter().map(|&p| p.into()).collect::<Vec<Point>>(),
            "glider" => GLIDER.iter().map(|&p| p.into()).collect::<Vec<Point>>(),
            "blinker" => BLINKER.iter().map(|&p| p.into()).collect::<Vec<Point>>(),
            "beacon" => BEACON.iter().map(|&p| p.into()).collect::<Vec<Point>>(),
            _ => {
                // let mut _tmp: Vec<Point> = vec![];
                // let mut rng = rand::thread_rng();
                // for i in 0..config.grid_width {
                //     for j in 0..config.grid_height {
                //         if rng.gen::<bool>() {
                //             _tmp.push((i, j).into());
                //         }
                //     }
                // }
                // _tmp

                // FIXME : make me beautiful
                (0..config.grid_height)
                    .collect::<Vec<usize>>()
                    .into_par_iter()
                    .map(move |i| {
                        (0..config.grid_width)
                            .collect::<Vec<usize>>()
                            .into_par_iter()
                            .filter_map(move |j| {
                                if rand::thread_rng().gen::<bool>() {
                                    Some((i, j).into())
                                } else {
                                    None
                                }
                            })
                    })
                    .flatten()
                    .collect::<Vec<Point>>()
            }
        };
        // Convert the starting states into a vector of points
        grid.set_state(&start_cells_coords);
        MainState { grid, config }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ggez::timer::check_update_time(ctx, self.config.fps) {
            self.grid.update();
        }
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0, 0, 0));
        // Mesh builder
        let mut builder = graphics::MeshBuilder::new();
        // Init, otherwise doesn't work for some reason
        builder.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect::new(0., 0., 0., 0.),
            Color::from_rgb(0, 0, 0),
        )?;
        // Draw cells
        for (idx, cell) in self.grid.cells.iter().enumerate() {
            if cell.is_alive() {
                let pos = self.grid.index_to_coords(idx);
                let color = graphics::Color::new(0., 200., 0., 1.); // Green
                builder.rectangle(
                    graphics::DrawMode::fill(),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                )?;
            }
        }
        // Draw grid
        if GRID {
            for idx in 0..self.grid.cells.len() {
                let color = graphics::Color::new(10., 10., 10., 1.); // ?
                let pos = self.grid.index_to_coords(idx);
                builder.rectangle(
                    graphics::DrawMode::stroke(1.),
                    graphics::Rect::new(
                        pos.x as f32 * self.config.cell_size,
                        pos.y as f32 * self.config.cell_size,
                        self.config.cell_size,
                        self.config.cell_size,
                    ),
                    color,
                )?;
            }
        }
        let mesh = builder.build(ctx)?;
        // Draw
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        // Present on screen
        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    // CLI
    let matches = App::new("Game of Life")
        .version("0.1")
        .author("ArcticSpaceFox")
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .help("Grid width")
                .value_name("width")
                .takes_value(true)
                .required(false)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .long("height")
                .help("Grid height")
                .value_name("height")
                .takes_value(true)
                .required(false)
                .default_value("64"),
        )
        .arg(
            Arg::with_name("initial_state")
                .short("s")
                .long("initial-state")
                .help("Initial state options: blinker, toad, glider, glider-gun, random")
                .value_name("initial_state")
                .takes_value(true)
                .required(false)
                .default_value("random"),
        )
        .arg(
            Arg::with_name("fps")
                .short("f")
                .long("fps")
                .help("Updates per second")
                .value_name("fps")
                .takes_value(true)
                .required(false)
                .default_value("20"),
        )
        .get_matches();

    // Get Configurations
    let grid_width = matches.value_of("width").unwrap().parse::<usize>().unwrap();
    let grid_height = matches
        .value_of("height")
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let initial_state = matches.value_of("initial_state").unwrap();
    let screen_size = (720., 720.);
    let fps = matches.value_of("fps").unwrap().parse::<u32>().unwrap();
    // Set configuration
    let config: Config = Config {
        grid_width,
        grid_height,
        cell_size: screen_size.0 / grid_width as f32,
        screen_size,
        fps,
        initial_state: initial_state.to_string(),
    };

    // Setup ggez stuff
    let cb = ContextBuilder::new("Game of life", "ArcticSpaceFox")
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_size.0, screen_size.1));
    let (mut ctx, event_loop) = cb.build()?; // `?` because the build function may fail
    graphics::set_window_title(&ctx, "Game of life");
    // Setup game state -> game loop
    let state = MainState::new(ctx.borrow_mut(), config);
    event::run(ctx, event_loop, state);
    Ok(())
}
