mod deck;
mod ui;
use deck::*;
use ui::*;

fn main() {
    // open current directory
    // iterate through all files recursively
    // put all data into a vector<(String(name of file), String(contents of file))>
    // shuffle the vector
    // 
    //
    // UI flow: see front of card, press button -> see back of card, press button -> see next front
    // of card, etc.
    
    let mut deck = Deck::new();
    if let Err(error) = deck.initialize() {
        panic!("ERROR: {}", error);
    }
    deck.shuffle();
    let mut app = App::new(deck);
    app.run();
    ratatui::restore();
}
