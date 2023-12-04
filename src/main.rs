extern crate termion;
extern crate tui;

use std::io::{self};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Alignment;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use tui::Terminal;

use std::io::Write;

use serde::{Deserialize, Serialize};

use git2::{Error, Repository};

#[derive(Serialize, Deserialize)]
struct Repo {
    name: String,
    url: String,
}

fn main() -> Result<(), io::Error> {
    let stdin = io::stdin();
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut list_state = ListState::default();
    list_state.select(Some(0)); // Start with the first item selected

    let repos_json = include_str!("../repos.json");

    let repos: Vec<Repo> = serde_json::from_str(repos_json).unwrap();

    let items = repos
        .iter()
        .map(|repo| {
            ListItem::new(format!("{}", repo.name)).style(Style::default().fg(Color::White))
        })
        .collect::<Vec<ListItem>>();

    terminal.clear()?;

    terminal.draw(|f| {
        let size = f.size();
        let list = List::new(items.clone())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Select an Item"),
            )
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol(">>");
        f.render_stateful_widget(list, size, &mut list_state);
    })?;

    for c in stdin.keys() {
        match c? {
            Key::Char('q') => {
                terminal.clear()?;
                break;
            }
            Key::Char('\n') => {
                if let Some(selected) = list_state.selected() {
                    terminal.clear()?;
                    terminal.draw(|f| {
                        let size = f.size();
                        let text = "Clonning the repo!\nPlease wait...";
                        let block = Paragraph::new(text)
                            .block(Block::default().borders(Borders::ALL).title("Message"))
                            .alignment(Alignment::Center);
                        f.render_widget(block, size);
                    })?;
                    match handle_selection(&repos[selected]) {
                        Ok(_) => {
                            terminal.clear()?;
                            break;
                        }
                        Err(_) => {
                            panic!("Error cloning repo")
                        }
                    };
                }
            }
            Key::Down => {
                let next = match list_state.selected() {
                    Some(i) if i < items.len() - 1 => i + 1,
                    _ => 0,
                };
                list_state.select(Some(next));
            }
            Key::Up => {
                let next = match list_state.selected() {
                    Some(i) if i > 0 => i - 1,
                    _ => items.len() - 1,
                };
                list_state.select(Some(next));
            }
            _ => {}
        }

        terminal.draw(|f| {
            let size = f.size();
            let list = List::new(items.clone())
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Select an Item"),
                )
                .highlight_style(Style::default().fg(Color::Yellow))
                .highlight_symbol(">>");
            f.render_stateful_widget(list, size, &mut list_state);
        })?;
    }

    terminal.draw(|f| {
        let size = f.size();
        let text = "Repo cloned!\nPress Enter to continue";
        let block = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Message"))
            .alignment(Alignment::Center);
        f.render_widget(block, size);
    })?;

    let stdin = io::stdin();
    for c in stdin.keys() {
        if let Key::Char('\n') = c? {
            break;
        }
    }

    terminal.clear()?;
    drop(terminal);

    // Ask the user for input
    println!("Select a security level (1, 2, 3, 4):");
    print!("> ");
    // Flush stdout
    io::stdout().flush()?;
    // Read the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    println!("You selected: {}", input.trim());

    Ok(())
}

fn handle_selection(repo: &Repo) -> Result<Repository, Error> {
    // Placeholder function for handling the selection
    // You can replace this with actual functionality
    Repository::clone(&repo.url, &repo.name)
}
