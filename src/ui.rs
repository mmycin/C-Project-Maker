use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use crate::config::{ProjectConfig, ProjectLanguage, OsType};
use crate::error::AppResult;

#[derive(Clone)]
pub enum InputMode {
    Normal,
    Editing,
}

pub enum FocusField {
    ProjectName,
    Language,
}

pub struct App {
    pub project_name: String,
    pub selected_language: ProjectLanguage,
    pub input_mode: InputMode,
    pub focus_field: FocusField,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project_name: String::new(),
            selected_language: ProjectLanguage::C,
            input_mode: InputMode::Normal,
            focus_field: FocusField::ProjectName,
            should_quit: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_event(&mut self, event: Event) -> AppResult<()> {
        match event {
            Event::Key(key) => {
                match (&self.input_mode, &self.focus_field, key.code) {
                    // Normal mode controls
                    (InputMode::Normal, _, KeyCode::Char('e')) => {
                        self.input_mode = InputMode::Editing;
                    }
                    (InputMode::Normal, _, KeyCode::Char('q')) => {
                        self.should_quit = true;
                    }
                    (InputMode::Normal, _, KeyCode::Tab) => {
                        self.focus_field = match self.focus_field {
                            FocusField::ProjectName => FocusField::Language,
                            FocusField::Language => FocusField::ProjectName,
                        };
                    }
                    // Project name editing
                    (InputMode::Editing, FocusField::ProjectName, KeyCode::Char(c)) => {
                        if (c.is_ascii_alphanumeric() || c == '-' || c == '_') && key.kind == KeyEventKind::Press {
                            self.project_name.push(c);
                        }
                    }
                    (InputMode::Editing, FocusField::ProjectName, KeyCode::Backspace) => {
                        self.project_name.pop();
                    }
                    (InputMode::Editing, _, KeyCode::Enter | KeyCode::Esc) => {
                        self.input_mode = InputMode::Normal;
                    }
                    // Language selection
                    (_, FocusField::Language, KeyCode::Left | KeyCode::Right) => {
                        self.selected_language = match self.selected_language {
                            ProjectLanguage::C => ProjectLanguage::Cpp,
                            ProjectLanguage::Cpp => ProjectLanguage::C,
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn get_config(&self, os_type: &OsType) -> ProjectConfig {
        ProjectConfig {
            name: self.project_name.clone(),
            language: self.selected_language.clone(),
            os_type: os_type.clone(),
        }
    }
}

pub fn draw(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(8),  // ASCII Art
            Constraint::Length(5),  // Description
            Constraint::Length(3),  // Project Name
            Constraint::Length(3),  // Language Selection
            Constraint::Length(3),  // Status Line
            Constraint::Length(3),  // Help Text
            Constraint::Min(0),    // Padding
        ])
        .split(f.size());

    // ASCII Art Title
    let title = vec![
        "   ██████╗██████╗ ██████╗  ██████╗      ██╗",
        "  ██╔════╝██╔══██╗██╔══██╗██╔═══██╗     ██║",
        "  ██║     ██████╔╝██████╔╝██║   ██║     ██║",
        "  ██║     ██╔═══╝ ██╔══██╗██║   ██║██   ██║",
        "  ╚██████╗██║     ██║  ██║╚██████╔╝╚█████╔╝",
        "   ╚═════╝╚═╝     ╚═╝  ╚═╝ ╚═════╝  ╚════╝ "
    ];

    let title_widget = Paragraph::new(title.join("\n"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double));
    f.render_widget(title_widget, chunks[0]);

    // Description
    let description = "A powerful C/C++ project generator by Mycin\nCreate professional C/C++ projects with ease\nUse [Tab] to navigate, [E] to edit, and arrow keys to select";
    let desc_widget = Paragraph::new(description)
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(desc_widget, chunks[1]);

    // Project name input
    let input = Paragraph::new(app.project_name.as_str())
        .style(match app.focus_field {
            FocusField::ProjectName => Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            _ => Style::default(),
        })
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 📁 Project Name ")
            .title_alignment(Alignment::Center)
            .border_style(match app.focus_field {
                FocusField::ProjectName => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .border_type(BorderType::Rounded));
    f.render_widget(input, chunks[2]);

    // Language selection with improved visuals
    let languages = vec!["C", "C++"];
    let language_index = match app.selected_language {
        ProjectLanguage::C => 0,
        ProjectLanguage::Cpp => 1,
    };

    let language_items: Vec<ListItem> = languages.iter()
        .map(|&lang| ListItem::new(lang))
        .collect();

    let language_select = List::new(language_items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" 🔧 Language ")
            .title_alignment(Alignment::Center)
            .border_style(match app.focus_field {
                FocusField::Language => Style::default().fg(Color::Yellow),
                _ => Style::default(),
            })
            .border_type(BorderType::Rounded))
        .highlight_style(Style::default().bg(Color::Yellow).fg(Color::Black).add_modifier(Modifier::BOLD))
        .highlight_symbol("➜ ")
        .style(match app.focus_field {
            FocusField::Language => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        });
    f.render_stateful_widget(
        language_select,
        chunks[3],
        &mut ListState::default().with_selected(Some(language_index)),
    );

    // Status line with mode indicator
    let status = match app.input_mode {
        InputMode::Normal => "🔍 Normal Mode",
        InputMode::Editing => "✏️  Editing Mode",
    };
    let status_widget = Paragraph::new(status)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(status_widget, chunks[4]);

    // Help text with improved visibility
    let help_text = match app.input_mode {
        InputMode::Normal => "[E]dit | [Q]uit | [Tab] Switch Focus | [←/→] Change Language",
        InputMode::Editing => "[Enter] Finish Editing | [Esc] Cancel",
    };
    let help = Paragraph::new(help_text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded));
    f.render_widget(help, chunks[5]);
}