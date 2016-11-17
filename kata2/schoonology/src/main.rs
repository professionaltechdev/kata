extern crate rustbox;

use rustbox::{Color, Key, RustBox};
use std::error::Error;
use std::time::{self, Duration};

const STEP_HZ:u64 = 100;

#[derive(Debug, Clone)]
enum Direction {
  North,
  South,
  East,
  West,
}

fn turn_left(dir:Direction) -> Direction {
  match dir {
    Direction::North => Direction::West,
    Direction::South => Direction::East,
    Direction::East => Direction::North,
    Direction::West => Direction::South,
  }
}

fn turn_right(dir:Direction) -> Direction {
  match dir {
    Direction::North => Direction::East,
    Direction::South => Direction::West,
    Direction::East => Direction::South,
    Direction::West => Direction::North,
  }
}

#[derive(Debug)]
struct Ant {
  x:usize,
  y:usize,
  dir:Direction,
}

struct Game {
  ant:Ant,
  grid:Vec<Vec<bool>>,
}

impl Game {
  fn new(width:usize, height:usize) -> Game {
    let mut grid = Vec::with_capacity(width);

    for _ in 0..width {
      let mut vec = Vec::with_capacity(height);
      vec.resize(height, false);
      grid.push(vec);
    }

    Game{
      ant:Ant{
        x:width / 2,
        y:height / 2,
        dir:Direction::East,
      },
      grid:grid,
    }
  }

  fn width(&self) -> usize {
    self.grid.len()
  }

  fn height(&self) -> usize {
    self.grid[0].len()
  }

  fn turn_ant(&mut self) {
    if self.grid[self.ant.x][self.ant.y] {
      self.ant.dir = turn_left(self.ant.dir.clone());
    } else {
      self.ant.dir = turn_right(self.ant.dir.clone());
    }
  }

  fn flip_tile(&mut self) {
    self.grid[self.ant.x][self.ant.y] = !self.grid[self.ant.x][self.ant.y];
  }

  fn move_ant(&mut self) {
    let width = self.width();
    let height = self.height();

    match self.ant.dir {
      Direction::North => self.ant.y = (self.ant.y + height - 1) % height,
      Direction::South => self.ant.y = (self.ant.y + 1) % height,
      Direction::East => self.ant.x = (self.ant.x + 1) % width,
      Direction::West => self.ant.x = (self.ant.x + width - 1) % width,
    }
  }

  fn tick(&mut self) {
    self.turn_ant();
    self.flip_tile();
    self.move_ant();
  }
}

struct Renderer {
  rustbox:RustBox,
  step_delay:Duration,
}

impl Renderer {
  fn new() -> Renderer {
    Renderer {
      rustbox:RustBox::init(Default::default()).expect("Rustbox failed to initialize."),
      step_delay:time::Duration::from_millis(1000 / STEP_HZ),
    }
  }

  fn width(&self) -> usize {
    self.rustbox.width()
  }

  fn height(&self) -> usize {
    self.rustbox.height()
  }

  fn should_quit(&self) -> bool {
    match self.rustbox.peek_event(self.step_delay, false) {
      Ok(rustbox::Event::KeyEvent(key)) => {
        match key {
          Key::Char('q') => true,
          _ => false,
        }
      },
      Err(e) => panic!("{}", e.description()),
      _ => false,
    }
  }

  fn draw_char(&self, x:usize, y:usize, c:char) {
    self.rustbox.print_char(x, y, rustbox::RB_BOLD, Color::White, Color::Black, c);
  }

  fn draw(&self, game:&Game) {
    for x in 0..game.grid.len() {
      for y in 0..game.grid[0].len() {
        self.draw_char(x, y, if game.grid[x][y] { '#' } else { ' ' });
      }
    }

    self.draw_char(
      game.ant.x, game.ant.y,
      match game.ant.dir {
        Direction::North => '^',
        Direction::South => '.',
        Direction::East => '>',
        Direction::West => '<',
      }
    );

    self.rustbox.present();
  }
}

fn main() {
  let renderer = Renderer::new();
  let mut game = Game::new(renderer.width(), renderer.height());

  loop {
    if renderer.should_quit() {
      break;
    }

    renderer.draw(&game);

    game.tick();
  }
}
