extern crate termion;
extern crate tui;

use std::io::{self};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState};
use tui::Terminal;

use git2::{Error, Repository};

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

    let repos = vec![
        Repo {
            name: "pycirmax".to_string(),
            url: "https://github.com/aitorru/pycirmax".to_string(),
        },
        Repo {
            name: "marsz".to_string(),
            url: "https://github.com/aitorru/marsz".to_string(),
        },
        Repo {
            name: "xyi".to_string(),
            url: "https://github.com/aitorru/xyi".to_string(),
        },
    ];

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
                    match handle_selection(&repos[selected]) {
                        Ok(_) => {
                            terminal.clear()?;
                            // Warn the user that the repo was cloned
                            terminal.draw(|f| {
                                let size = f.size();
                                let text = vec![ListItem::new("Repo cloned!")
                                    .style(Style::default().fg(Color::White))];
                                let list = List::new(text)
                                    .block(
                                        Block::default()
                                            .borders(Borders::ALL)
                                            .title("Select an Item"),
                                    )
                                    .highlight_style(Style::default().fg(Color::Yellow))
                                    .highlight_symbol(">>");
                                f.render_widget(list, size);
                            })?;
                            break;
                        }
                        Err(_) => {}
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

    Ok(())
}

fn handle_selection(repo: &Repo) -> Result<Repository, Error> {
    // Placeholder function for handling the selection
    // You can replace this with actual functionality
    Repository::clone(&repo.url, &repo.name)
}
