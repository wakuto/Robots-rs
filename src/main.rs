use rand::{Rng, thread_rng};
use ncurses::*;

#[derive(Copy, Clone, Debug)]
struct Point {
  x: usize,
  y: usize,
}

impl Point {
  fn new(x: usize, y: usize) -> Point {
    Point { x: x, y: y }
  }
}

#[derive(Copy, Clone, Debug)]
struct Direction {
  x: i32,
  y: i32,
}

impl Direction {
  fn new(x: i32, y: i32) -> Direction {
    Direction { x: x, y: y }
  }
}

trait Movable {
  fn moving(&mut self, field: &Field, dir: Direction);
}

struct Player {
  pos: Point,
}

impl Movable for Player {
  fn moving(&mut self, field: &Field, dir: Direction) {
    let x = self.pos.x as i32 + dir.x;
    let y = self.pos.y as i32 + dir.y;
    if x >= 0 && y >= 0 && x < field.pos.x as i32 && y < field.pos.y as i32 {
      self.pos.x = x as usize;
      self.pos.y = y as usize;
    }
  }
}
impl Player {
  fn new(pos: Point) -> Player {
    Player {
      pos: pos,
    }
  }
}

struct Robot {
  pos: Point,
}

impl Movable for Robot {
  fn moving(&mut self, field: &Field, dir: Direction) {
    let x = self.pos.x as i32 + dir.x;
    let y = self.pos.y as i32 + dir.y;
    if x >= 0 && y >= 0 {
      self.pos.x = x as usize;
      self.pos.y = y as usize;
    }
  }
}
impl Robot {
  fn new(pos: Point) -> Robot{
    Robot {
      pos: pos,
    }
  }
}

struct Field {
  pos: Point,
  x_size: usize,
  y_size: usize,
  player: Player,
  robots: Vec<Robot>,
}

impl Field {
  fn new(pos: Point, x_size: usize, y_size: usize, robots_num: usize) -> Field {
    let mut rng = thread_rng();
    let mut rob: Vec<Robot> = vec![Robot::new(Point{x:rng.gen_range(0..x_size), y:rng.gen_range(0..y_size)})];
    for _ in 1..robots_num {
      rob.push(Robot::new(Point{x:rng.gen_range(0..x_size), y:rng.gen_range(0..y_size)}));
    }
    Field {
      pos: pos,
      x_size: x_size,
      y_size: y_size,
      player: Player::new(Point{x:x_size/2, y:y_size/2}),
      robots: rob,
    }
  }
  // fieldから出ない範囲で周囲のマスを取得
  fn get_around(&self, pos: Point) -> (usize, usize, usize, usize) {
    let up = match pos.y > 0 {
      true => pos.y-1,
      _ => pos.y,
    };
    let down = match pos.y < self.y_size-1 {
      true => pos.y+1,
      _ => pos.y,
    };
    let right = match pos.x < self.x_size-1 {
      true => pos.x+1,
      _ => pos.x,
    };
    let left = match pos.x > 0 {
      true => pos.x-1,
      _ => pos.x,
    };
    (up, right, down, left)
  }

  fn print(&self) {
    let x = self.pos.x as i32;
    let y = self.pos.y as i32;
    let scr = stdscr();

    let mut frame = String::new();
    for i in 0..self.x_size {
      frame = format!("{}-", &frame);
    }
    // フレームの描画
    mv(y-1, x);
    waddstr(scr, &frame);
    mv(y+self.y_size as i32, x);
    waddstr(scr, &frame);
    // プレイヤーの描画
    mv(y + self.player.pos.y as i32, x + self.player.pos.x as i32);
    waddstr(scr, "@");
    // robotの描画
    for rob in &self.robots {
      mv(y + rob.pos.y as i32, x + rob.pos.x as i32);
      waddstr(scr, "+");
    }
  }
}

fn main() {
  let window = initscr();
  noecho();
  nonl();
  intrflush(stdscr(), true);
  keypad(stdscr(), true);
  addstr("***Robots***");
  refresh();

  const KEY_QUIT:  i32 = 'q' as i32;
  const KEY_LEFT:  i32 = b'j' as i32;
  const KEY_DOWN:  i32 = b',' as i32;
  const KEY_UP:    i32 = b'i' as i32;
  const KEY_RIGHT: i32 = b'l' as i32;
  const KEY_STAY:  i32 = b'k' as i32;
  const KEY_RUP:   i32 = b'o' as i32;
  const KEY_RDOWN: i32 = b'.' as i32;
  const KEY_LUP:   i32 = b'u' as i32;
  const KEY_LDOWN: i32 = b'm' as i32;

  let mut field = Field::new(Point{x:5, y:5}, 20, 20, 3);
  field.print();

    //mv(y + self.player.pos.y as i32, x + self.player.pos.x as i32);
  let mut x = field.pos.x + field.player.pos.x;
  let mut y = field.pos.y + field.player.pos.y;
  let mut res = true;
  loop {
    //mv((field.pos.y + y) as i32, (field.pos.x + x) as i32);
    let prev_x = x;
    let prev_y = y;
    match getch() {
      KEY_RIGHT => { if x < field.x_size-1 { x += 1; } },
      KEY_LEFT  => { if x > 0 { x -= 1; } },
      KEY_DOWN  => { if y < field.y_size-1 { y += 1; } },
      KEY_UP    => { if y > 0 { y -= 1; } },
      KEY_RUP   => { 
        if y > 0 { y -= 1; }
        if x < field.x_size-1 { x += 1; }
      },
      KEY_LUP   => {
        if y > 0 { y -= 1; }
        if x > 0 { x -= 1; }
      },
      KEY_RDOWN => {
        if y < field.y_size-1 { y += 1; } 
        if x < field.x_size-1 { x += 1; }
      },
      KEY_LDOWN => {
        if y < field.y_size-1 { y += 1; } 
        if x > 0 { x -= 1; }
      },
      KEY_QUIT  => {endwin(); return;},
      _ => continue,
    };
    let dir = Direction::new(x as i32-prev_x as i32, y as i32-prev_y as i32);
    field.player.moving(&field, dir);
    mv((field.pos.y + y) as i32, (field.pos.x + x) as i32);
    field.print();
  }
  while getch() != KEY_QUIT {
  }

  endwin();
}
