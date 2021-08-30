use rand::Rng;
use ncurses::*;

mod internal;
use internal::*;
use internal::{KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT};

macro_rules! exit {
  () => {
    endwin();
    return;
  }
}

macro_rules! print_status {
  ($level:expr, $score:expr) => {
    mv(3, 0);
    addstr(&format!("level: {}, score: {}", $level, $score));
  }
}

macro_rules! print_result {
  ($res: expr) => {
    mv(1, 0);
    addstr($res);
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

  let mut score:u64 = 0;
  let mut level:u32 = 1;

  loop {
    let mut field = Field::new(Point{x:5, y:5}, 150, 40, std::cmp::min((level*5) as usize, 40));
    field.print();

    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;
    let mut robot_res;

    print_result!("         ");
    loop {
      print_status!(level, score);
      // 入力
      if !input(&field, &mut x, &mut y) {
        exit!();
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

      match robot_res {
        Some(x) => { score += x },
        _ => {
          print_result!("you lose");
          while getch() != KEY_QUIT {}
          exit!();
        }
      }
      if field.robots_pos.len() == 0 {
        print_result!("you win");
        print_status!(level, score);

        score += (level * 10) as u64;
        getch();
        break;
      }
    }
    level += 1;
  }
}

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

