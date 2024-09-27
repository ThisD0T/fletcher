use crossterm::event::{self, Event, KeyCode};
use ratatui::style::Stylize;
use ratatui::{
    prelude::Alignment,
    backend::CrosstermBackend,
    layout::{Constraint, Layout},
    style::Modifier,
    widgets::{Paragraph, Block},
    Terminal,
};

use std::io::Stdout;

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
    incorrect_indexes: Vec<usize>,
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
            incorrect_indexes: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        let _ = self.terminal.clear();
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
            let [head_area, body_area, foot_area] = layout.areas(f.area());

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
            let help_info = Paragraph::new("j for next, h for incorrect, l for correct").alignment(Alignment::Center);

            f.render_widget(display_text, body_area);
            f.render_widget(correct_text, head_area);
            f.render_widget(card_count, head_area);
            f.render_widget(help_info, foot_area);
        });         
    }
    fn draw_end(&mut self) {
        let _ = self.terminal.draw(|f| {
            let block = Block::bordered().title(format!("{} wrong ... q to quit", self.num_correct)).title_alignment(Alignment::Center);

            let mut incorrect_string: String = String::new(); 
            self.incorrect_indexes.iter().for_each(|index| {
                incorrect_string += (format!("\nCard {}, title: {}", index, self.deck.cards[*index].front)).as_str()
            });
            let incorrect_list = Paragraph::new(incorrect_string).block(block);

            f.render_widget(incorrect_list, f.area());
        });
    }

    fn handle_input(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            // TODO: Make this not shit
            match key.code {
                KeyCode::Char('h') => {// incrrect answer
                    if self.show_front { return }
                    self.incorrect_indexes.push(self.card_index);
                    self.increment();
                }
                KeyCode::Char('l') => {// correct answer
                    if self.show_front { return }
                    self.num_correct += 1;
                    self.increment();
                }
                KeyCode::Char('j') | KeyCode::Right => {// next
                    if !self.show_front { return }
                    self.increment()
                }
                KeyCode::Char('q') | KeyCode::Esc => {// exit app
                    self.state = AppState::Exit;
                }
                _ => (),
            }
        }
    }

    // this is shit
    fn increment(&mut self) {
        // if reached the end of the deck
        if !self.show_front && self.card_index + 1 == self.deck.cards.len() {
            self.state = AppState::ShowEnd;
            return
        }
        if self.show_front {
            self.show_front = false;
        } else {
            self.show_front = true;
            self.card_index += 1;
        }
    }
}
