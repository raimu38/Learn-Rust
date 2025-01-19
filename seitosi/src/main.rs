// ライフゲームのプログラム
use lazyrand::rand_usize;
use std::thread;
use std::io::{stdout, Result};
use crossterm::{cursor, execute, terminal,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor}};
// 定数の宣言
const WIDTH: usize = 80; // グリッドの列数
const HEIGHT: usize = 35; // グリッドの行数
const MAX_TERN: usize = 1000; // 最大世代数
// メイン関数を記述
fn main() -> Result<()> {
    // 画面をクリアしてカーソルを(0, 0)に移動
    execute!(stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0)
    )?;
    let mut cells = init_cells();
    for i in 0..MAX_TERN { // 繰り返し世代を勧める
        // セルを描画
        draw_cells(&cells)?;
        println!("{}/{}", i, MAX_TERN);
        // 300ミリ秒待つ
        thread::sleep(std::time::Duration::from_millis(300));
        // 次の世代のセルを計算
        cells = next_generation(&cells);
    }
    Ok(())
}

// ランダムにセルの初期化を行う関数
fn init_cells() -> Vec<Vec<bool>> {
    // 2次元のベクタ配列を作成してfalseで初期化
    let mut cells = vec![vec![false; WIDTH]; HEIGHT];
    // 適当にライフゲームの初期状態を作成
    for _ in 0..(WIDTH * HEIGHT / 13) {
        cells[rand_usize() % HEIGHT][rand_usize() % WIDTH] = true;
    }
    cells
}

// セルをターミナルに描画する
fn draw_cells(cells: &Vec<Vec<bool>>) -> Result<()> {
    execute!(stdout(), cursor::MoveTo(0, 0))?;
    for row in cells {
        for &cell in row {
            if cell {
                execute!(stdout(),
                    SetForegroundColor(Color::Yellow),
                    SetBackgroundColor(Color::Red),
                    Print("+"))?;
            } else {
                execute!(stdout(),
                    SetForegroundColor(Color::Blue),
                    SetBackgroundColor(Color::Black),
                    Print("-"))?;
            }
        }
        execute!(stdout(), Print("\n"))?;
    }
    // execute!(stdout(), ResetColor)?;
    Ok(())
}

// 次世代のセルを計算する関数
fn next_generation(cells: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // 次世代のセルを生成
    let mut new_cells = vec![vec![false; WIDTH]; HEIGHT];
    // 現在のセルについてルールを適用
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // 周囲の生存セルを数える
            let mut count = 0;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dy == 0 && dx == 0 { continue; }
                    let ny = (y as isize + dy + HEIGHT as isize) as usize % HEIGHT;
                    let nx = (x as isize + dx + WIDTH as isize) as usize % WIDTH;
                    if cells[ny][nx] { count += 1; } // 生存セルなら+1
                }
            }
            // ライフゲームのルールの沿って次世代の生死を判定
            new_cells[y][x] = match (cells[y][x], count) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }
    new_cells
}