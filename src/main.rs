mod config;
mod downloader;
mod error;
mod extractor;
mod ui;
mod utils;

use crossterm::event;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use error::AppResult;
use ratatui::prelude::*;
use std::io::stdout;

fn main() -> AppResult<()> {
    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Create app state
    let mut app = ui::App::new();
    let os_type = config::detect_os();

    // Main loop
    while !app.should_quit {
        terminal.draw(|f| ui::draw(f, &app))?;

        if event::poll(std::time::Duration::from_millis(50))? {
            if let Ok(event) = event::read() {
                app.handle_event(event)?;
            }
        }

        if app.should_quit {
            let config = app.get_config(&os_type);
            if !config.name.is_empty() {
                // Exit TUI mode before starting the download process
                disable_raw_mode()?;
                stdout().execute(LeaveAlternateScreen)?;
                println!("\n🚀 Creating project: {}", config.name);
                
                let zip_file_name = "repo.zip";
                let repo_url = config::get_template_url(&config.language, &config.os_type);
                
                println!("📥 Downloading template...");
                downloader::download_zip(&repo_url, zip_file_name)?;
                
                println!("📦 Extracting files...");
                extractor::extract_zip(zip_file_name)?;

                let current_dir = std::env::current_dir()?;
                let branch_name = if repo_url.contains("/windows-c.zip") {
                    "windows-c"
                } else if repo_url.contains("/windows-cpp.zip") {
                    "windows-cpp"
                } else if repo_url.contains("/linux-c.zip") {
                    "linux-c"
                } else {
                    "linux-cpp"
                };
                let old_path = current_dir.join(format!("C-Template-{}", branch_name));
                let new_path = current_dir.join(&config.name);
                
                println!("📝 Setting up project structure...");
                utils::rename_folder(old_path.to_str().unwrap(), new_path.to_str().unwrap())?;
                utils::delete_file(zip_file_name)?;
                
                println!("\n✨ Project {} has been created successfully!", config.name);
                println!("   Happy coding! 🎉\n");
                return Ok(());
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}