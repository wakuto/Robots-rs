use rand::Rng;
use ncurses::*;

mod internal;
use internal::*;
use internal::{KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT};

fn input(field: &Field, x_org: &mut usize, y_org: &mut usize) -> bool {
  let mut rng = rand::thread_rng();
  let mut x = *x_org;
  let mut y = *y_org;

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
    KEY_RAND  => {
      x = rng.gen::<usize>() % field.x_size;
      y = rng.gen::<usize>() % field.y_size;
    },
    KEY_QUIT  => { return false; },
    KEY_STAY  => (),
    _ => (),
  };
  *x_org = x;
  *y_org = y;
  true
}

fn main() {
  initscr();
  noecho();
  nonl();
  intrflush(stdscr(), true);
  keypad(stdscr(), true);
  addstr("***Robots***");
  refresh();

  let mut field = Field::new(Point{x:5, y:5}, 150, 40, 10);
  field.print();

  let mut x = field.player_pos.x;
  let mut y = field.player_pos.y;
  let mut robot_res;
  loop {
    // 入力
    if !input(&field, &mut x, &mut y) {
      endwin();
      return;
    }

    // プレイヤーの移動
    if !field.player_move(Point::new(x, y)) {
      x = field.player_pos.x;
      y = field.player_pos.y;
      continue;
    }

    // 勝ち負けを判定
    robot_res = field.robots_move();
    field.print();

    if !robot_res {
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
