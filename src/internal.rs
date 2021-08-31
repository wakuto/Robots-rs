use rand::Rng;
use ncurses::*;
use std::collections::HashSet;

/// プログラムを終了
pub const KEY_QUIT:  i32 = b'q' as i32;
/// 左
pub const KEY_LEFT:  i32 = b'j' as i32;
/// 下
pub const KEY_DOWN:  i32 = b',' as i32;
/// 上
pub const KEY_UP:    i32 = b'i' as i32;
/// 右
pub const KEY_RIGHT: i32 = b'l' as i32;
/// 移動しない
pub const KEY_STAY:  i32 = b' ' as i32;
/// 右上
pub const KEY_RUP:   i32 = b'o' as i32;
/// 右下
pub const KEY_RDOWN: i32 = b'.' as i32;
/// 左上
pub const KEY_LUP:   i32 = b'u' as i32;
/// 左下
pub const KEY_LDOWN: i32 = b'm' as i32;
/// ランダム
pub const KEY_RAND:  i32 = b'k' as i32;
/// これ以降は動かない
pub const KEY_STOP:  i32 = b'0' as i32;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
/// フィールド上の位置を示します
pub struct Point {
  /// x座標
  pub x: usize,
  /// y座標
  pub y: usize,
}

impl Point {
  pub fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }
}

#[derive(Copy, Clone, Debug)]
/// フィールド上のオブジェクトの種類を表します
pub enum Object {
  /// プレイヤー
  Player,
  /// ロボット
  Robot,
  /// スクラップ
  Scrap,
  /// 何もない
  Null,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
/// 次に行う処理の種類を表します
pub enum Status {
  /// 通常の処理
  Normal,
  /// 未定義の処理
  Unknown,
  /// ゲーム結果が決まるまで動かない
  Stop,
  /// ゲーム終了
  Exit,
}

/// ゲームフィールドを表します
pub struct Field {
  /// fieldが配置される場所
  pub pos: Point,
  /// 横幅
  pub width: usize,
  /// 縦幅
  pub height: usize,
  /// playerの位置
  pub player_pos: Point,
  /// robotの位置のリスト
  pub robots_pos: Vec<Point>,
  /// scrapの位置のリスト
  pub scraps_pos: HashSet<Point>,
  /// fieldを表すリスト
  pub field: Vec<Vec<Object>>,
}

impl Field {
  /// fieldを生成し、robotをランダムに配置します
  /// * `pos` - fieldが配置される場所
  /// * `width` - fieldの横幅
  /// * `height` - fieldの縦幅
  /// * `robots_num` - robotの数
  pub fn new(pos: Point, width: usize, height: usize, robots_num: usize) -> Field {
    let mut rng = rand::thread_rng();
    let mut field = vec![vec![Object::Null; width]; height];
    let mut robots = vec![Point::new(0, 0); robots_num];
    let scraps: HashSet<Point> = HashSet::new();
    let player = Point::new(width>>1, height>>1);

    let mut player_idx = 0;
    let mut coord_list = vec![Point::new(0, 0); width*height];
    for y in 0..height {
      for x in 0..width {
        coord_list[y*width + x] = Point::new(x, y);
        if Point::new(x, y) == player {
          player_idx = y*width + x;
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
      pos,
      width,
      height,
      player_pos: Point::new(width>>1, height>>1),
      robots_pos: robots,
      scraps_pos: scraps,
      field,
    }
  }

  /// playerを移動させます
  /// 指定の座標に移動できないときは`false`を返します
  /// * `pos` - 移動先の座標
  pub fn player_move(&mut self, pos: Point) -> bool {
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

  /// robotをplayerの方向に移動させます
  /// playerが負けた場合は`None`
  /// それ以外の場合は獲得したscoreを返します
  pub fn robots_move(&mut self) -> Option<u64> {
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

      self.robots_pos[rob_idx].x = (robot.x as i32 + x) as usize;
      self.robots_pos[rob_idx].y = (robot.y as i32 + y) as usize;
    }

    let score = self.check_scrap();   // スクラップにする
    let res = self.check_player_pos();// プレイヤーの安全を確認する

    // field情報の更新
    self.field_clear();
    self.field[self.player_pos.y][self.player_pos.x] = Object::Player;
    self.field_set(self.robots_pos.clone(), Object::Robot);
    self.field_set(self.scraps_pos.clone().into_iter().collect(), Object::Scrap);

    if res {
      Some(score)
    } else {
      None
    }
  }

  /// fieldを`Object::Null`で埋めます
  fn field_clear(&mut self) {
    for y in 0..self.height {
      for x in 0..self.width {
        self.field[y][x] = Object::Null;
      }
    }
  }

  /// fieldの指定した座標を指定の`Object`に設定します
  /// * `points` - 設定する座標のリスト
  /// * `obj` - 設定するObjectのタイプ
  fn field_set(&mut self, points: Vec<Point>, obj: Object) {
    for p in points {
      self.field[p.y][p.x] = obj;
    }
  }

  /// 衝突したrobotをscrapにします
  /// 倒したrobotの数から計算した`score`を返します
  fn check_scrap(&mut self) -> u64 {
    let mut score = 0;

    // 同じ座標にあるrobotを削除・scrapに追加
    let rob_num = self.robots_pos.len();
    let mut rem_idx = Vec::<usize>::new();
    let mut hash_pos = HashSet::<Point>::new();

    for rob_idx in 0..rob_num {
      if !hash_pos.contains(&self.robots_pos[rob_idx]) {
        hash_pos.insert(self.robots_pos[rob_idx]);
      } else {
        rem_idx.push(rob_idx);
        if !self.scraps_pos.contains(&self.robots_pos[rob_idx]) {
          self.scraps_pos.insert(self.robots_pos[rob_idx]);
        }
      }
    }
    for rob_idx in rem_idx.iter().rev() {
      self.robots_pos.remove(*rob_idx);
      score += 1;
    }

    // scrapと同じ座標にあるrobotを削除
    let rob_num = self.robots_pos.len();
    let mut rem_idx: Vec<usize> = Vec::new();
    for rob_idx in 0..rob_num {
      if self.scraps_pos.contains(&self.robots_pos[rob_idx]) {
        rem_idx.push(rob_idx);
      }
    }
    for rob_idx in rem_idx.iter().rev() {
      self.robots_pos.remove(*rob_idx);
      score += 1;
    }
    score
  }

  /// playerが安全な場所に居るかを判定します
  /// playerが安全なら`true`
  /// それ以外なら`false`を返します
  fn check_player_pos(&self) -> bool {
    // playerとrobotの座標を比較
    let mut res = true;
    for rob in &self.robots_pos {
      if *rob == self.player_pos {
        res = false;
        break;
      }
    }

    // playerとscrapの座標を比較
    if res {
      for scrap in &self.scraps_pos {
        if *scrap == self.player_pos {
          res = false;
          break;
        }
      }
    }
    res
  }

  /// fieldをフレーム付きでncursesのウィンドウに描画します
  pub fn print(&self) {
    let x = self.pos.x as i32;
    let y = self.pos.y as i32;

    let mut frame = String::new();
    for _i in 0..self.width {
      frame = format!("{}-", &frame);
    }
    // フレームの描画
    mv(y-1, x);
    addstr(&frame);
    mv(y+self.height as i32, x);
    addstr(&frame);
    // プレイヤーの描画
    for pos_y in 0..self.height {
      mv(y + pos_y as i32, x-1);
      addstr("|");
      for pos_x in 0..self.width {
        mv(y + pos_y as i32, x + pos_x as i32);
        match &self.field[pos_y][pos_x] {
          Object::Player => addstr("@"),
          Object::Robot  => addstr("+"),
          Object::Scrap  => addstr("*"),
          _              => addstr(" "),
        };
      }
      addstr("|");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

#[test]
  fn point_new_test() {
    let p = Point::new(1,2);
    let q = Point { x:1, y:2 };
    assert_eq!(p, q);
  }

#[test]
  fn field_new_test() {
    let field = Field::new(Point::new(0, 0), 50, 20, 10);
    let mut rob_count = 0;
    let mut player_count = 0;
    let mut scrap_count = 0;
    for y in 0..field.height {
      for x in 0..field.width {
        match field.field[y][x] {
          Object::Robot => { rob_count += 1; },
          Object::Player => { player_count += 1; },
          Object::Scrap => { scrap_count += 1; },
          _ => (),
        }
      }
    }
    assert_eq!(rob_count, 10);
    assert_eq!(player_count, 1);
    assert_eq!(scrap_count, 0);
  }

#[test]
  fn player_move_test() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 0);

    field.field[0][0] = Object::Null;
    assert!(field.player_move(Point::new(0, 0)));
    assert_eq!(Point::new(0, 0), field.player_pos);

    field.field[0][1] = Object::Robot;
    assert!(!field.player_move(Point::new(1, 0)));
    assert_eq!(Point::new(0, 0), field.player_pos);

    field.field[0][1] = Object::Scrap;
    assert!(!field.player_move(Point::new(1, 0)));
    assert_eq!(Point::new(0, 0), field.player_pos);

    field.field[0][1] = Object::Null;
    assert!(field.player_move(Point::new(1, 0)));
    assert_eq!(Point::new(1, 0), field.player_pos);
  }

#[test]
  fn robots_move_test() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 0);

    field.robots_pos.push(Point::new(0, 0));
    field.player_move(Point::new(10, 0));
    assert_eq!(field.robots_move(), Some(0));
    assert_eq!(field.robots_pos[0], Point::new(1, 0));

    field.player_move(Point::new(1, 10));
    assert_eq!(field.robots_move(), Some(0));
    assert_eq!(field.robots_pos[0], Point::new(1, 1));

    field.robots_pos[0] = Point::new(10, 10);
    field.player_move(Point::new(0, 0));
    assert_eq!(field.robots_move(), Some(0));
    assert_eq!(field.robots_pos[0], Point::new(9, 9));

    field.player_move(Point::new(8, 8));
    assert_eq!(field.robots_move(), None);
    assert_eq!(field.robots_pos[0], Point::new(8, 8));

    field.robots_pos[0] = Point::new(10, 10);
    field.robots_pos.push(Point::new(12, 10));
    field.player_move(Point::new(11, 0));
    assert_eq!(field.robots_move(), Some(2));
    assert_eq!(field.robots_pos.len(), 0);
    assert!(field.scraps_pos.contains(&Point::new(11, 9)))
  }

#[test]
  fn field_clear_test() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 40);

    field.field_clear();
    
    let mut null_count = 0;
    for y in 0..field.height {
      for x in 0..field.width {
        match field.field[y][x] {
          Object::Null => { null_count += 1; },
          _ => (),
        }
      }
    }
    assert_eq!(null_count, field.height*field.width);
  }

#[test]
  fn field_set_test() {
    let mut field = Field::new(Point::new(0, 0), 50, 20, 40);

    field.field_clear();
    for i in 0..10 {
      field.robots_pos.push(Point::new(i, i));
    }

    field.field_set(field.robots_pos.clone(), Object::Robot);
    for i in 0..10 {
      assert!(match field.field[i][i] {
        Object::Robot => true,
        _ => false,
      });
    }

  }

}
