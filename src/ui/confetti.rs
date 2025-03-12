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

pub fn show_confetti() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    let (cols, rows) = crossterm::terminal::size()?;

    let confetti_chars = vec!['★', '☆', '✦', '✧', '•'];
    let colors = vec![
        Color::Magenta,
        Color::Cyan,
        Color::Yellow,
        Color::Blue,
        Color::Green,
        Color::Red,
    ];

    stdout.execute(Hide)?;

    let mut rng = rand::rng();

    for _ in 0..50 {
        stdout.execute(Clear(ClearType::All))?;
        let pieces = (cols as u32 * rows as u32) / 10;

        for _ in 0..pieces {
            let x = rng.random_range(0..cols);
            let y = rng.random_range(0..rows);
            let ch = confetti_chars[rng.random_range(0..confetti_chars.len())];
            let color = colors[rng.random_range(0..colors.len())];

            stdout.execute(MoveTo(x, y))?;
            stdout.execute(SetForegroundColor(color))?;
            print!("{}", ch);
        }

        stdout.flush()?;
        thread::sleep(Duration::from_millis(100));
    }

    stdout.execute(ResetColor)?;
    stdout.execute(Show)?;
    stdout.execute(Clear(ClearType::All))?;
    Ok(())
}
