//! 本プログラムは授業用に制作したRobotsのプログラムです

use rand::Rng;
use ncurses::*;

mod internal;
use internal::*;
use internal::{KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT};

/// endwin()を呼んでmain()からreturnする
macro_rules! exit {
  () => {
    endwin();
    return;
  }
}

/// ステータスの位置にlevel, scoreを表示する
macro_rules! print_status {
  ($level:expr, $score:expr) => {
    mv(3, 0);
    addstr(&format!("level: {}, score: {}", $level, $score));
  }
}

/// プレイ結果の位置にリザルトを表示する
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

  let mut scr_w: i32 = 0;
  let mut scr_h: i32 = 0;
  getmaxyx(stdscr(), &mut scr_h, &mut scr_w);

  loop {
    let mut field = Field::new(Point{x:5, y:5}, (scr_w-8) as usize, (scr_h-6) as usize, std::cmp::min((level*5) as usize, 40));
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
        Some(scr) => { score += scr },
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

/// 8方向+その場にとどまる+ランダム移動+終了を入力する
/// 終了の場合はfalseを返し、それ以外はtrueを返す
/// * `field` - フィールドの情報
/// * `x_org` - 現在のプレイヤーのx座標
/// * `y_org` - 現在のプレイヤーのy座標
fn input(field: &Field, x_org: &mut usize, y_org: &mut usize) -> bool {
  let mut rng = rand::thread_rng();
  let mut x = *x_org;
  let mut y = *y_org;

  match getch() {
    KEY_RIGHT => { if x < field.width-1 { x += 1; } },
    KEY_LEFT  => { if x > 0 { x -= 1; } },
    KEY_DOWN  => { if y < field.height-1 { y += 1; } },
    KEY_UP    => { if y > 0 { y -= 1; } },
    KEY_RUP   => { 
      if y > 0 { y -= 1; }
      if x < field.width-1 { x += 1; }
    },
    KEY_LUP   => {
      if y > 0 { y -= 1; }
      if x > 0 { x -= 1; }
    },
    KEY_RDOWN => {
      if y < field.height-1 { y += 1; } 
      if x < field.width-1 { x += 1; }
    },
    KEY_LDOWN => {
      if y < field.height-1 { y += 1; } 
      if x > 0 { x -= 1; }
    },
    KEY_RAND  => {
      x = rng.gen::<usize>() % field.width;
      y = rng.gen::<usize>() % field.height;
    },
    KEY_QUIT  => { return false; },
    KEY_STAY  => (),
    _ => (),
  };
  *x_org = x;
  *y_org = y;
  true
}

