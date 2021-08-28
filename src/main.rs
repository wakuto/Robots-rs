use rand::Rng;
use ncurses::*;

#[derive(Copy, Clone, Debug, PartialEq)]
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
enum Object {
  Player,
  Robot,
  Scrap,
  Null,
}

struct Field {
  pos: Point,
  x_size: usize,
  y_size: usize,
  player_pos: Point,
  robots_pos: Vec<Point>,
  scraps_pos: Vec<Point>,
  field: Vec<Vec<Object>>,
}

impl Field {
  fn new(pos: Point, x_size: usize, y_size: usize, robots_num: usize) -> Field {
    let mut rng = rand::thread_rng();
    let mut field = vec![vec![Object::Null; x_size]; y_size];
    let mut robots = vec![Point::new(0, 0); robots_num];
    let scraps: Vec<Point> = Vec::new();
    let player = Point::new(x_size>>1, y_size>>1);

    let mut player_idx = 0;
    let mut coord_list = vec![Point::new(0, 0); x_size*y_size];
    for y in 0..y_size {
      for x in 0..x_size {
        coord_list[y*x_size + x] = Point::new(x, y);
        if Point::new(x, y) == player {
          player_idx = y*x_size + x;
        }
      }
    }
    coord_list.remove(player_idx);


    for i in 0..robots_num {
      let idx = rng.gen::<usize>() % coord_list.len();
      robots[i] = coord_list[idx];
      coord_list.remove(idx);
    }
    for robot in &robots {
      field[robot.y][robot.x] = Object::Robot;
    }
    field[player.y][player.x] = Object::Player;

    Field {
      pos: pos,
      x_size: x_size,
      y_size: y_size,
      player_pos: Point::new(x_size>>1, y_size>>1),
      robots_pos: robots,
      scraps_pos: scraps,
      field: field,
    }
  }

// TODO: ランダム移動の実装
  fn player_move(&mut self, pos: Point) -> bool {
    let res;
    match self.field[pos.y][pos.x] {
      Object::Null | Object::Player => {
        self.field[self.player_pos.y][self.player_pos.x] = Object::Null;
        self.field[pos.y][pos.x] = Object::Player;
        self.player_pos = pos;
        res = true;
      },
      _ => {
        res = false;
      }
    }
    res
  }

// TODO: ロボットをプレイヤー方向に近づける
// スクラップをその場に表示させ続ける
  fn robots_move(&mut self) -> bool {
    let rob_num = self.robots_pos.len();

    // とりあえずロボットを移動させる
    for rob_idx in 0..rob_num {
      // robotからplayerの距離
      let robot = self.robots_pos[rob_idx];
      let mut x = self.player_pos.x as i32 - robot.x as i32;
      let mut y = self.player_pos.y as i32 - robot.y as i32;
      // 進む方向の正規化
      if x != 0 { x /= x.abs(); }
      if y != 0 { y /= y.abs(); }

      let x_next = (robot.x as i32 + x) as usize;
      let y_next = (robot.y as i32 + y) as usize;
      self.robots_pos[rob_idx].x = x_next;
      self.robots_pos[rob_idx].y = y_next;
    }

    // 同じ座標にあるrobotを削除・scrapに追加
    let rob_num = self.robots_pos.len();
    let mut rem_idx = Vec::<usize>::new();
    if rob_num >= 2 {
      for rob_idx_a in 0..(rob_num-1) {
        for rob_idx_b in (rob_idx_a+1)..rob_num {
          if self.robots_pos[rob_idx_a] == self.robots_pos[rob_idx_b] {
            rem_idx.push(rob_idx_a);
            if !self.scraps_pos.contains(&self.robots_pos[rob_idx_a]) {
              self.scraps_pos.push(self.robots_pos[rob_idx_a]);
            }
          }
        }
      }
    }
    for rob_idx in rem_idx.iter().rev() {
      self.robots_pos.remove(*rob_idx);
    }

    // scrapと同じ座標にあるrobotを削除
    let rob_num = self.robots_pos.len();
    let mut rem_idx = Vec::<usize>::new();
    for rob_idx in 0..rob_num {
      if self.scraps_pos.contains(&self.robots_pos[rob_idx]) {
        rem_idx.push(rob_idx);
      }
    }
    for rob_idx in rem_idx.iter().rev() {
      self.robots_pos.remove(*rob_idx);
    }

    // playerとrobotの座標を比較
    let mut res = true;
    for rob in &self.robots_pos {
      if *rob == self.player_pos {
        res = false;
        break;
      }
    }

    // field情報の更新
    self.field_clear();
    self.field[self.player_pos.y][self.player_pos.x] = Object::Player;
    self.field_set(self.robots_pos.clone(), Object::Robot);
    self.field_set(self.scraps_pos.clone(), Object::Scrap);

    res
  }

  fn field_clear(&mut self) {
    for y in 0..self.y_size {
      for x in 0..self.x_size {
        self.field[y][x] = Object::Null;
      }
    }
  }

  fn field_set(&mut self, points: Vec<Point>, obj: Object) {
    for p in points {
      self.field[p.y][p.x] = obj;
    }
  }

/*
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
*/

  fn print(&self) {
    let x = self.pos.x as i32;
    let y = self.pos.y as i32;
    let scr = stdscr();

    let mut frame = String::new();
    for _i in 0..self.x_size {
      frame = format!("{}-", &frame);
    }
    // フレームの描画
    mv(y-1, x);
    waddstr(scr, &frame);
    mv(y+self.y_size as i32, x);
    waddstr(scr, &frame);
    // プレイヤーの描画
    for pos_y in 0..self.y_size {
      for pos_x in 0..self.x_size {
        mv(y + pos_y as i32, x + pos_x as i32);
        match &self.field[pos_y][pos_x] {
          Object::Player => waddstr(scr, "@"),
          Object::Robot  => waddstr(scr, "+"),
          Object::Scrap  => waddstr(scr, "*"),
          _              => waddstr(scr, " "),
        };
      }
    }
  }
}

fn main() {
  initscr();
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
  const KEY_STAY:  i32 = b' ' as i32;
  const KEY_RUP:   i32 = b'o' as i32;
  const KEY_RDOWN: i32 = b'.' as i32;
  const KEY_LUP:   i32 = b'u' as i32;
  const KEY_LDOWN: i32 = b'm' as i32;
  const KEY_RAND:  i32 = b'k' as i32;

  let mut field = Field::new(Point{x:5, y:5}, 150, 40, 10);
  field.print();

    //mv(y + self.player.pos.y as i32, x + self.player.pos.x as i32);
  let mut rng = rand::thread_rng();
  let mut x = field.player_pos.x;
  let mut y = field.player_pos.y;
  let mut player_res;
  let mut robot_res;
  loop {
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
      KEY_STAY  => (),
      KEY_RAND  => {
        x = rng.gen::<usize>() % field.x_size;
        y = rng.gen::<usize>() % field.y_size;
      },
      KEY_QUIT  => {endwin(); return;},
      _ => continue,
    };
    player_res = field.player_move(Point::new(x, y));
    robot_res = field.robots_move();
    field.print();

    // 勝ち負けを判定
    if !player_res | !robot_res {
      mv(1, 0);
      waddstr(stdscr(), "you lose");
      getch();
      endwin();
      break;
    }
    if field.robots_pos.len() == 0 {
      mv(1, 0);
      waddstr(stdscr(), "you win");
      getch();
      endwin();
      break;
    }
  }
}
