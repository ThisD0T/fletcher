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

#[derive(PartialEq)]
enum AppState {
    ShowEnd,
    ShowDeck,
    Exit,
}

pub struct App {
    terminal: Terminal<CrosstermBackend<Stdout>>,

    deck: Deck,

    card_index: usize,
    show_front: bool,
    state: AppState,

    num_correct: i32,
}

impl App {
    pub fn new(deck: Deck) -> Self {
        App {
            terminal: ratatui::init(),
            deck,
            card_index: 0,
            show_front: true,
            state: AppState::ShowDeck,
            num_correct: 0,
        }
    }

    pub fn run(&mut self) {
        self.draw();
        loop {
            match self.state {
                AppState::ShowDeck => {
                    self.handle_input();
                    self.draw();
                }
                AppState::ShowEnd => {
                    self.handle_input();
                    self.draw_end();
                }
                AppState::Exit => {
                    break;
                }
            }
        }
        while self.state == AppState::ShowDeck {
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
    fn draw_end(&mut self) {
        let _ = self.terminal.draw(|f| {
            let paragraph = Paragraph::new(format!("num correct: {}", self.num_correct)).alignment(Alignment::Center);
            f.render_widget(paragraph, f.area());
        });
    }

    fn handle_input(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            // TODO: Make this not shit and also make it less exploitable
            match key.code {
                KeyCode::Char('h') => {
                    if self.show_front { return }
                    self.increment(true);
                }
                KeyCode::Char('l') => {
                    if self.show_front { return }
                    self.num_correct += 1;
                    self.increment(true);
                }
                KeyCode::Char('j') | KeyCode::Right => {// next
                    if !self.show_front { return }
                    self.increment(true)
                }
                KeyCode::Char('k') | KeyCode::Left => {// previous
                    self.increment(false)
                }
                KeyCode::Char('q') | KeyCode::Esc => {// exit app
                    self.state = AppState::Exit;
                }
                _ => (),
            }
        }
    }

    // this is shit
    fn increment(&mut self, forward: bool) {
        // if reached the end of the deck
        if forward && !self.show_front && self.card_index == self.deck.cards.len() - 1 {
            self.state = AppState::ShowEnd;
            return
        }
        if forward {
            if self.show_front {
                self.show_front = false;
            } else {
                self.show_front = true;
                self.card_index += 1;
            }
        } else {
            if self.show_front {
                self.card_index -= 1;
                self.show_front = false;
            } else {
                self.show_front = true;
            }
        }
    }
}
