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
    widgets::{Cell, ListItem, Row, Table, TableState, Paragraph, block, Block},
    Terminal,
    Frame,
};

use std::{fmt::write, io::Stdout};

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
        self.terminal.clear();
        self.draw();
        loop {
            self.handle_input();
            match self.state {
                AppState::ShowDeck => {
                    self.draw();
                }
                AppState::ShowEnd => {
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
            
            let layout = Layout::vertical(
                [Constraint::Min(3),
                Constraint::Percentage(95),
                Constraint::Min(3)]
            );
            let [head_area, body_area, _foot_area] = layout.areas(f.area());

            // text to display
            let display_text: Paragraph = if self.show_front {
                let block = Block::bordered()
                    .title("Card Title");
                Paragraph::new(self.deck.cards[self.card_index].front.clone()).alignment(Alignment::Center).add_modifier(Modifier::BOLD).block(block)
            } else {
                let block = Block::bordered()
                    .title("Card Contents");
                Paragraph::new(self.deck.cards[self.card_index].back.clone()).block(block)
            };


            let correct_text = Paragraph::new(format!("{} correct", self.num_correct)).alignment(Alignment::Right);
            let card_count = Paragraph::new(format!("{}/{} Cards complete", self.card_index, self.deck.cards.len())).alignment(Alignment::Left);

            f.render_widget(display_text, body_area);
            f.render_widget(correct_text, head_area);
            f.render_widget(card_count, head_area);
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
        if forward && !self.show_front && self.card_index + 1 == self.deck.cards.len() {
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

fn calc_percentage(num1: &i32, num2: &i32) -> i32 {
    if *num2 <= 0 { return 0 }
    (num1 / num2) * 100
}
