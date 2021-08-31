//! 本プログラムは授業用に制作したRobotsのプログラムです

use rand::Rng;
use ncurses::*;
use std::fs;

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
    let mut stop = false;

    print_result!("         ");
    loop {
      print_status!(level, score);
      // 入力
      if !stop {
        match input(getch(), &field, &mut x, &mut y) {
          Status::Normal => (),
          Status::Unknown => { continue; },
          Status::Exit => { exit!(); },
          Status::Stop => { stop = true; },
        }
      }

      // プレイヤーの移動
      if !field.player_move(Point::new(x, y)) {
        x = field.player_pos.x;
        y = field.player_pos.y;
        continue;
      }

      // 勝ち負けを判定
      robot_res = field.robots_move(stop);
      field.print();

      match robot_res {
        Some(scr) => { score += scr },
        _ => {
          print_result!("you lose");
          let highscore = get_highscore(&get_score_file("score.dat"));
          if score > highscore {
            mv(2, 0);
            addstr(&format!("HighScore!! {} points", score));
            save_record("score.dat", score);
          }
          getch();
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
/// * `ch` - キー入力
/// * `field` - フィールドの情報
/// * `x_org` - 現在のプレイヤーのx座標
/// * `y_org` - 現在のプレイヤーのy座標
fn input(ch: i32, field: &Field, x_org: &mut usize, y_org: &mut usize) -> Status {
  let mut rng = rand::thread_rng();
  let mut x = *x_org;
  let mut y = *y_org;

  match ch {
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
    KEY_QUIT  => { return Status::Exit; },
    KEY_STAY  => (),
    KEY_STOP  => { return Status::Stop; },
    _ => { return Status::Unknown; },
  };
  *x_org = x;
  *y_org = y;
  Status::Normal
}

/// 記録をファイルから読み込み、文字列として返します
/// * `filename` - 記録のファイル名
fn get_score_file(filename: &str) -> String {
  match fs::read_to_string(&filename) {
    Ok(val) => val.trim().to_string(),
    _ => "".to_string(),
  }
}

/// 最も高いスコアを返します
/// * `score_file` - 記録ファイルの文字列データ
fn get_highscore(file_str: &str) -> u64 {
  if file_str != "" {
    let score_str: Vec<&str> = file_str.split('\n').collect::<Vec<&str>>();
    let mut scores: Vec<u64> = score_str.iter().map(|x| x.parse()
      .expect("スコアファイルの形式が違います")).collect();
    scores.sort();
    scores[scores.len()-1]
  } else {
    0
  }
}

/// 記録をスコアファイルに追記します
/// * `filename` - 保存するファイル名
/// * `score` - 保存するスコア
fn save_record(filename: &str, score: u64) {
  let score_file = get_score_file(filename);
  let new_score_file = format!("{}\n{}", score_file, score);
  fs::write(&filename, new_score_file.trim())
    .expect("スコアデータの書き込みに失敗しました");
}

#[cfg(test)]
mod tests {
  use super::*;

#[test]
  fn input_test_up() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 0);
    field.player_move(Point::new(1, 0));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LUP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 0));

    assert_eq!(input(KEY_UP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 0));

    assert_eq!(input(KEY_RUP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(1, 0));

    field.player_move(Point::new(1, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LUP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 2));

    assert_eq!(input(KEY_UP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 1));

    assert_eq!(input(KEY_RUP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(1, 0));

    field.player_move(Point::new(0, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LUP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 2));

    field.player_move(Point::new(field.width-1, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_RUP as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(field.width-1, 2));

    assert_eq!(input(KEY_QUIT as i32, &field, &mut x, &mut y), Status::Exit);
  }

#[test]
  fn input_test_mid() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 0);
    field.player_move(Point::new(1, 0));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LEFT as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 0));

    assert_eq!(input(KEY_STAY as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 0));

    assert_eq!(input(KEY_RIGHT as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(1, 0));

    field.player_move(Point::new(0, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LEFT as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 3));

    field.player_move(Point::new(field.width-1, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_RIGHT as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(field.width-1, 3));
  }

#[test]
  fn input_test_down() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 0);
    field.player_move(Point::new(1, field.height-1));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LDOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, field.height-1));

    assert_eq!(input(KEY_DOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, field.height-1));

    assert_eq!(input(KEY_RDOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(1, field.height-1));

    field.player_move(Point::new(1, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LDOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 4));

    assert_eq!(input(KEY_DOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 5));

    assert_eq!(input(KEY_RDOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(1, 6));

    field.player_move(Point::new(0, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_LDOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(0, 4));

    field.player_move(Point::new(field.width-1, 3));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    assert_eq!(input(KEY_RDOWN as i32, &field, &mut x, &mut y), Status::Normal);
    assert_eq!(Point::new(x, y), Point::new(field.width-1, 4));
  }

#[test]
  fn input_test_other() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 0);
    field.player_move(Point::new(5, 5));
    let mut x = field.player_pos.x;
    let mut y = field.player_pos.y;

    for i in 0..256 {
      match i {
        KEY_UP | KEY_DOWN | KEY_RIGHT | KEY_LEFT | KEY_QUIT | KEY_STAY |
        KEY_RUP | KEY_RDOWN | KEY_LUP | KEY_LDOWN | KEY_RAND | KEY_STOP
        => (),
        _ => {
          assert_eq!(input(i, &field, &mut x, &mut y), Status::Unknown);
          assert_eq!(Point::new(x,y), Point::new(5, 5)); 
        },
      }
    }
  }
}
