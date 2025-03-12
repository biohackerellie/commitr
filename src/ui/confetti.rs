use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use rand::Rng;
use std::error::Error;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Confetti {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ch: char,
    color: Color,
}

pub fn show_confetti() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let (cols, rows) = crossterm::terminal::size()?;

    stdout.execute(Hide)?;

    let center_x = (cols / 2) as f32;
    let center_y = (rows / 2) as f32;

    let confetti_chars = vec!['★', '☆', '✦', '✧', '•'];
    let colors = vec![
        Color::Magenta,
        Color::Cyan,
        Color::Yellow,
        Color::Blue,
        Color::Green,
        Color::Red,
    ];
    let mut rng = rand::rng();
    let num_pieces = (cols as u32 * rows as u32) / 20;
    let mut pieces: Vec<Confetti> = Vec::new();

    for _ in 0..num_pieces {
        let angle: f32 = rng.random_range(0.0..(2.0 * std::f32::consts::PI));
        let speed: f32 = rng.random_range(1.0..10.0);
        let vx = speed * angle.cos();
        let vy = speed * angle.sin();
        let ch = confetti_chars[rng.random_range(0..confetti_chars.len())];
        let color = colors[rng.random_range(0..colors.len())];
        pieces.push(Confetti {
            x: center_x,
            y: center_y,
            vx,
            vy,
            ch,
            color,
        })
    }
    let dt: f32 = 0.01;
    let gravity: f32 = 0.1;
    let frames = 300;

    for _ in 0..frames {
        stdout.execute(Clear(ClearType::All))?;

        for piece in pieces.iter_mut() {
            piece.x += piece.vx * dt;
            piece.y += piece.vy * dt;

            piece.vy += gravity * dt;

            let x = piece.x.round() as u16;
            let y = piece.y.round() as u16;

            if x < cols && y < rows {
                stdout.execute(MoveTo(x, y))?;
                stdout.execute(SetForegroundColor(piece.color))?;
                print!("{}", piece.ch);
            }
        }

        stdout.flush()?;
        thread::sleep(Duration::from_millis((dt * 1000.0) as u64));
    }
    stdout.execute(ResetColor)?;
    stdout.execute(Show)?;
    stdout.execute(Clear(ClearType::All))?;

    Ok(())
}
