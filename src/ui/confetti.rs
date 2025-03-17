use crossterm::{
  ExecutableCommand,
  cursor::{Hide, MoveTo, Show},
  style::{Color, ResetColor, SetForegroundColor},
  terminal::{Clear, ClearType, size},
};
use rand::Rng;
use std::error::Error;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Cell {
  ch: char,
  processed: bool,
  disperse_direction: i32,
  color: Color,
  fixed: bool,
}

impl Cell {
  fn new(ch: char, color: Color) -> Self {
    let direction = if rand::random() { 1 } else { -1 };
    Cell {
      ch,
      processed: false,
      disperse_direction: direction,
      color,
      fixed: false,
    }
  }

  fn fixed(ch: char, color: Color) -> Self {
    Cell {
      ch,
      processed: false,
      disperse_direction: 0,
      color,
      fixed: true,
    }
  }
}

// Returns true if the cell at (row, col) is within bounds and empty.
fn cell_empty(grid: &[Vec<Cell>], row: usize, col: usize) -> bool {
  if row < grid.len() && col < grid[row].len() {
    grid[row][col].ch == ' '
  } else {
    false
  }
}

// Swap two cells in the grid.
fn swap_cells(grid: &mut [Vec<Cell>], row1: usize, col1: usize, row2: usize, col2: usize) {
  if row1 == row2 {
    // If both cells are in the same row, we can split that row.
    let row = &mut grid[row1];
    if col1 == col2 {
      return;
    }
    let (left_idx, right_idx) = if col1 < col2 {
      (col1, col2)
    } else {
      (col2, col1)
    };
    let (left, right) = row.split_at_mut(right_idx);
    std::mem::swap(&mut left[left_idx], &mut right[0]);
  } else {
    // For cells in different rows, we need to split the grid itself.
    if row1 < row2 {
      let (first, second) = grid.split_at_mut(row2);
      // first[row1] is the row at index row1.
      // second[0] is the row at index row2.
      std::mem::swap(&mut first[row1][col1], &mut second[0][col2]);
    } else {
      // row2 < row1
      let (first, second) = grid.split_at_mut(row1);
      // first[row2] is the row at index row2.
      // second[0] is the row at index row1.
      std::mem::swap(&mut first[row2][col2], &mut second[0][col1]);
    }
  }
}

fn place_center_text(grid: &mut [Vec<Cell>], text: &str, color: Color) {
  let total_rows = grid.len();
  if total_rows == 0 {
    return;
  }

  let total_cols = grid[0].len();
  let lines: Vec<&str> = text.lines().collect();
  let num_lines = lines.len();
  let start_row = if num_lines < total_rows {
    (total_rows - num_lines) / 2
  } else {
    0
  };

  for (i, line) in lines.iter().enumerate() {
    let line_chars: Vec<char> = line.chars().collect();
    let line_len = line_chars.len();

    let start_col = if line_len < total_cols {
      (total_cols - line_len) / 2
    } else {
      0
    };
    for (j, &ch) in line_chars.iter().enumerate() {
      if start_row + i < total_rows && start_col + j < total_cols {
        grid[start_row + i][start_col + j] = Cell::fixed(ch, color)
      }
    }
  }
}

// Update the grid for one frame. Returns true if any cell moved.
fn update_grid(
  grid: &mut [Vec<Cell>],
  frame: usize,
  disperse_rate: usize,
  side_noise: bool,
) -> bool {
  let mut updated = false;
  let rows = grid.len();
  let cols = if rows > 0 { grid[0].len() } else { 0 };

  // Rest the proccessed flag for all cells
  for row in grid.iter_mut() {
    for cell in row.iter_mut() {
      cell.processed = false;
    }
  }

  // Process rows from second-to-last up to the top (simulate falling).
  // (The last row is already processed.)
  for r in (0..rows - 1).rev() {
    // Use snake-like iteration: alternating left to right and right to left.
    let col_indices: Vec<usize> = if (frame + r) % 2 == 0 {
      (0..cols).collect()
    } else {
      (0..cols).rev().collect()
    };
    for &c in &col_indices {
      // Skip empty and processed cells.
      if grid[r][c].ch == ' ' || grid[r][c].fixed || grid[r][c].processed {
        continue;
      }
      grid[r][c].processed = true;

      // Side noise: occasionally try to shift sideways.
      if side_noise {
        let side_step_prob = 0.05;
        let random_val: f64 = rand::random();
        if random_val < side_step_prob && c + 1 < cols && cell_empty(grid, r, c + 1) {
          swap_cells(grid, r, c, r, c + 1);
          updated = true;
          continue;
        } else if random_val < 2.0 * side_step_prob && c > 0 && cell_empty(grid, r, c - 1) {
          swap_cells(grid, r, c, r, c - 1);
          updated = true;
          continue;
        }
      }

      // Try to move the cell down.
      if cell_empty(grid, r + 1, c) {
        swap_cells(grid, r, c, r + 1, c);
        updated = true;
      } else {
        // If doward movement is blocked, try to move sideways.
        let mut last_row = r;
        let mut last_col = c;
        let direction = grid[r][c].disperse_direction;
        for d in 1..=disperse_rate {
          let new_col = if direction < 0 {
            c.checked_sub(d)
          } else {
            Some(c + d)
          };
          if let Some(nc) = new_col {
            if nc < cols && cell_empty(grid, r, nc) {
              swap_cells(grid, last_row, last_col, r, nc);
              updated = true;
              last_col = nc;
            }
            if cell_empty(grid, r + 1, nc) {
              swap_cells(grid, last_row, last_col, r + 1, nc);
              updated = true;
              last_row = r + 1;
              last_col = nc;
            }
          } else {
            break;
          }
        }
      }
    }
  }
  updated
}

pub fn show_confetti() -> Result<(), Box<dyn Error>> {
  // Get terminal dimensions
  let (cols, rows) = size()?;
  let cols = cols as usize;
  let rows = rows as usize;

  let mut stdout = stdout();
  stdout.execute(Hide)?;

  // Create a grid filled with empty cells.
  let mut grid: Vec<Vec<Cell>> = vec![vec![Cell::new(' ', Color::Reset); cols]; rows];

  // Seed the grid with confetti in the top half.
  let num_confetti = (cols * rows) / 10; // Adjust density here
  let confetti_chars = ['★', '☆', '✦', '✧', '•'];
  let colors = [
    Color::Magenta,
    Color::Cyan,
    Color::Yellow,
    Color::Blue,
    Color::Green,
    Color::Red,
  ];
  let mut rng = rand::rng();

  for _ in 0..num_confetti {
    let r = rng.random_range(0..(rows / 2));
    let c = rng.random_range(0..cols);
    let ch = confetti_chars[rng.random_range(0..confetti_chars.len())];
    let color = colors[rng.random_range(0..colors.len())];
    grid[r][c] = Cell::new(ch, color);
  }

  let text = r#" 
 ██████████                                ███
░░███░░░░███                              ░███
 ░███   ░░███  ██████  ████████    ██████ ░███
 ░███    ░███ ███░░███░░███░░███  ███░░███░███
 ░███    ░███░███ ░███ ░███ ░███ ░███████ ░███
 ░███    ███ ░███ ░███ ░███ ░███ ░███░░░  ░░░ 
 ██████████  ░░██████  ████ █████░░██████  ███
░░░░░░░░░░    ░░░░░░  ░░░░ ░░░░░  ░░░░░░  ░░░  
"#;

  // Sim params
  let fps = 50;
  let frame_delay = Duration::from_millis(1000 / fps);
  let mut frame = 0;
  let side_noise = true;
  let disperse_rate = 3;

  place_center_text(&mut grid, text, Color::DarkCyan);

  // Animation
  loop {
    frame += 1;
    let updated = update_grid(&mut grid, frame, disperse_rate, side_noise);

    // Draw the grid by moving the cursor to the top-left and printing each cell.
    stdout.execute(MoveTo(0, 0))?;
    for (index, row) in grid.iter().enumerate() {
      for cell in row {
        stdout.execute(SetForegroundColor(cell.color))?;
        print!("{}", cell.ch);
      }
      if index != grid.len() - 1 {
        println!();
      }
    }
    stdout.flush()?;
    thread::sleep(frame_delay);

    if !updated {
      break;
    }
  }

  stdout.execute(ResetColor)?;
  stdout.execute(Show)?;
  stdout.execute(Clear(ClearType::All))?;
  Ok(())
}
