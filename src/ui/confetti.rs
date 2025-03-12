use crossterm::{
    ExecutableCommand,
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::error::Error;
use std::io::stdout;
use std::thread;
use std::time::Duration;

pub fn show_confetti() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    for _ in 0..5 {
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(SetForegroundColor(Color::Magenta))?;
        println!("{}", "★ ☆ ★ ☆ ★ ☆ ★ ☆ ★ ☆".repeat(2));
        stdout.execute(ResetColor)?;
        thread::sleep(Duration::from_millis(150));
        stdout.execute(Clear(ClearType::All))?;
        stdout.execute(SetForegroundColor(Color::Cyan))?;
        println!("{}", "★ ☆ ★ ☆ ★ ☆ ★ ☆ ★ ☆".repeat(2));
        stdout.execute(ResetColor)?;
        thread::sleep(Duration::from_millis(150));
    }
    Ok(())
}
