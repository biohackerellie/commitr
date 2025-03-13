use git2::{BranchType, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use inquire::{
    Confirm, Editor, Select, Text,
    ui::{Color as InquireColor, RenderConfig, Styled},
};
use std::error::Error;
use std::process::Command;
use std::time::Duration;

use colored::Colorize;

pub fn run_git_workflow() -> Result<(), Box<dyn Error>> {
    // discover repo and current branch
    let repo = Repository::discover(".")?;
    let head = repo.head()?;
    let source_branch = head.shorthand().unwrap_or("unknown").to_string();
    println!("Current branch: {}", source_branch.green());

    // Interactive prompts for commit details
    let commit_types = vec![
        "fix", "feat", "docs", "style", "refactor", "bump", "chore", "revert",
    ];
    let commit_type = Select::new("Select the type of change:", commit_types).prompt()?;
    let bump_types = vec!["major", "minor", "patch"];
    let bump_type = Select::new("Select the type of bump:", bump_types).prompt()?;
    let mut default_summary = format!("{}: ", commit_type);
    if bump_type != "patch" {
        default_summary = format!("{}({}): ", commit_type, bump_type);
    }

    let commit_summary = Text::new("Summary of changes:")
        .with_initial_value(&default_summary)
        .prompt()?;

    let commit_description = Editor::new("Details of this change:")
        .with_formatter(&|submission| {
            let char_count = submission.chars().count();
            if char_count == 0 {
                String::from("<skipped>")
            } else if char_count <= 20 {
                submission.into()
            } else {
                let mut substr: String = submission.chars().take(17).collect();
                substr.push_str("...");
                substr
            }
        })
        .with_render_config(description_render_config())
        .prompt()?;

    if !Confirm::new("Commit changes?(y/n)").prompt()? {
        println!("{}", "Operation cancelled.".red());
        return Ok(());
    }

    println!("{}", "Proceeding with commit...".blue());
    println!("Commit Type: {}", commit_type);
    println!("Bump Type: {}", bump_type);
    println!("Summary: {}", commit_summary);
    println!("Description: {}", commit_description);

    // Git Operations
    run_with_loading("git", &["add", "-A"])?;
    run_with_loading(
        "git",
        &["commit", "-m", &commit_summary, "-m", &commit_description],
    )?;
    run_with_loading("git", &["push", "origin", &source_branch])?;

    // dyanic merging into other branches
    loop {
        if !Confirm::new("Merge your changes into another branch?").prompt()? {
            break;
        }

        let available_branches: Vec<String> = list_local_branches(&repo)?
            .into_iter()
            .filter(|b| b != &source_branch)
            .collect();

        if available_branches.is_empty() {
            println!(
                "{}",
                "No other branches available locally. Create or fetch a branch.".red()
            );
            break;
        }

        let target_branch =
            Select::new("Select the branch to merge into:", available_branches).prompt()?;

        run_with_loading("git", &["checkout", &target_branch])?;
        run_with_loading("git", &["pull", "--rebase"])?;
        run_with_loading("git", &["merge", &source_branch])?;
        run_with_loading("git", &["push", "origin", &target_branch])?;
        run_with_loading("git", &["checkout", &source_branch])?;

        println!(
            "{}",
            format!("Changes merged into {} successfully.", target_branch).green()
        );

        let confirm_message = format!("Merge commited changes back into {}?", &source_branch);
        if Confirm::new(&confirm_message).prompt()? {
            run_with_loading("git", &["pull", "origin", &target_branch])?;
        }
    }
    Ok(())
}

fn description_render_config() -> RenderConfig<'static> {
    RenderConfig::default()
        .with_canceled_prompt_indicator(Styled::new("<skipped>").with_fg(InquireColor::DarkYellow))
}

pub fn run_command(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    println!("Running: {} {:?}", command, args);
    let status = Command::new(command).args(args).status()?;
    if !status.success() {
        return Err(format!("Command {:?} {:?} failed", command, args).into());
    }
    Ok(())
}

pub fn run_with_loading(command: &str, args: &[&str]) -> Result<(), Box<dyn Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner} {msg}")
            .expect("Failed to set template"),
    );
    pb.enable_steady_tick(Duration::from_millis(100));
    pb.set_message(format!("Running: {} {:?}", command, args));
    let status = Command::new(command).args(args).status()?;
    pb.finish_with_message("Done");
    if !status.success() {
        return Err(format!("Command {:?} {:?} failed", command, args).into());
    }
    Ok(())
}

pub fn list_local_branches(repo: &Repository) -> Result<Vec<String>, Box<dyn Error>> {
    let mut branches = Vec::new();
    for branch in repo.branches(Some(BranchType::Local))? {
        let (branch, _) = branch?;
        if let Some(name) = branch.name()? {
            branches.push(name.to_string());
        }
    }
    Ok(branches)
}
