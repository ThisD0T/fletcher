use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::style::Stylize;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Layout, Direction, Constraint::Ratio, Alignment},
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    text::Text,
    widgets::{Cell, ListItem, Row, Table, TableState, Paragraph},
    Terminal,
    Frame,
};

use std::{io::Stdout};

use crate::deck::Deck;

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    deck: Deck,

    card_index: usize,
    show_front: bool,
    run: bool,
}

impl App {
    pub fn new(deck: Deck) -> Self {
        App {
            terminal: ratatui::init(),
            deck,
            card_index: 0,
            show_front: true,
            run: true,
        }
    }

    pub fn run(&mut self) {
        self.draw();
        while self.run {
            self.handle_input();
            self.draw();
        }
    }

    fn draw(&mut self) {
        let _ = self.terminal.draw(|f| {
            let display_text: Paragraph = if self.show_front {
                Paragraph::new(self.deck.cards[self.card_index].front.clone()).alignment(Alignment::Center)
            } else {
                Paragraph::new(self.deck.cards[self.card_index].back.clone())
            };

            f.render_widget(display_text, f.area());
        });         
    }

    fn handle_input(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Char('j') | KeyCode::Right => {// next
                    if self.show_front {
                        self.show_front = false;
                    } else {
                        self.show_front = true;
                        self.card_index += 1;
                    }
                }
                KeyCode::Char('k') | KeyCode::Left => {// previous
                    if self.show_front {
                        self.show_front = false;
                        self.card_index -= 1;
                    } else {
                        self.show_front = true;
                    }
                }
                KeyCode::Char('q') | KeyCode::Esc => {// exit app
                    self.run = false;
                }
                _ => (),
            }
        }
    }
}
