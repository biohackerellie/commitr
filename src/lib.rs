pub mod git_ops;
pub mod ui;
use colored::Colorize;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    ui::splash::show_splash_screen()?;

    git_ops::run_git_workflow()?;

    ui::confetti::show_confetti()?;

    println!(
        "{}",
        "All operations completed successfully."
            .green()
            .bold()
            .underline()
    );
    Ok(())
}
