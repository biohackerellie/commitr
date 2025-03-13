use crossterm::{
    ExecutableCommand,
    cursor::{Hide, MoveTo, Show},
    style::{Color, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, size},
};
use std::error::Error;
use std::io::{Write, stdout};
use std::thread;
use std::time::Duration;

pub fn show_splash_screen() -> Result<(), Box<dyn Error>> {
    let mut stdout = stdout();
    stdout.execute(Hide)?;
    stdout.execute(Clear(ClearType::All))?;

    let (cols, rows) = size()?;
    let terminal_width = cols as usize;
    let terminal_height = rows as usize;

    let splash = r#"
   █████████                                          ███  █████            
  ███░░░░░███                                        ░░░  ░░███             
 ███     ░░░   ██████  █████████████  █████████████  ████ ███████  ████████ 
░███          ███░░███░░███░░███░░███░░███░░███░░███░░███░░░███░  ░░███░░███
░███         ░███ ░███ ░███ ░███ ░███ ░███ ░███ ░███ ░███  ░███    ░███ ░░░ 
░░███     ███░███ ░███ ░███ ░███ ░███ ░███ ░███ ░███ ░███  ░███ ███░███     
 ░░█████████ ░░██████  █████░███ ██████████░███ ██████████ ░░█████ █████    
  ░░░░░░░░░   ░░░░░░  ░░░░░ ░░░ ░░░░░░░░░░ ░░░ ░░░░░░░░░░   ░░░░░ ░░░░░     
    "#;

    let splash_lines: Vec<&str> = splash.lines().filter(|l| !l.is_empty()).collect();
    let splash_height = splash_lines.len();
    let vertical_padding = (terminal_height - splash_height) / 2;

    let centered_lines: Vec<String> = splash_lines
        .iter()
        .map(|line| {
            let line_length = line.chars().count();
            let horizontal_padding = (terminal_width - line_length) / 2;
            format!("{}{}", " ".repeat(horizontal_padding), line)
        })
        .collect();

    let start_color = (0u8, 0u8, 0u8);
    let end_color = (0u8, 139u8, 139u8);
    let fade_steps = 20;

    for step in 0..=fade_steps {
        stdout.execute(MoveTo(0, 0))?;

        let r = start_color.0
            + ((end_color.0 - start_color.0) as f32 * (step as f32 / fade_steps as f32)) as u8;
        let g = start_color.1
            + ((end_color.1 - start_color.1) as f32 * (step as f32 / fade_steps as f32)) as u8;

        let b = start_color.2
            + ((end_color.2 - start_color.2) as f32 * (step as f32 / fade_steps as f32)) as u8;
        let color = Color::Rgb { r, g, b };

        let mut output = String::new();
        for (i, line) in centered_lines.iter().enumerate() {
            let vertical_pos = vertical_padding + i;
            output.push_str(&format!("\x1B[{};{}H{}", vertical_pos + 1, 1, line));
        }

        stdout.execute(SetForegroundColor(color))?;
        write!(stdout, "{}", output)?;
        stdout.flush()?;
        thread::sleep(Duration::from_millis(30))
    }

    thread::sleep(Duration::from_secs(1));

    stdout.execute(ResetColor)?;
    stdout.execute(Clear(ClearType::All))?;
    stdout.execute(Show)?;
    Ok(())
}
