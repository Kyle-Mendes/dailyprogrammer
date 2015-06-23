extern crate rand;

use std::fmt;
use rand::{StdRng, Rng};
use std::io::stdin;

#[derive(Copy, Clone, Debug)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades
}

#[derive(Copy, Clone, Debug)]
enum Face {
     Ace = 12,
     Two = 0,
     Three = 1,
     Four = 2,
     Five = 3,
     Six = 4,
     Seven = 5,
     Eight = 6,
     Nine = 7,
     Ten = 8,
     Jack = 9,
     Queen = 10,
     King = 11
}

/// Pushes all face values into an array for easy use and iteration.
impl Face {
    pub fn as_array() -> [Face;13] {
        [Face::Ace, Face::Two, Face::Three, Face::Four, Face::Five, Face::Six, Face::Seven, Face::Eight, Face::Nine, Face::Ten, Face::Jack, Face::Queen,Face::King]
    }
}

/// Cards that will go into the deck.
/// Each cards has a suit and face value.
struct Card {
    suit: Suit,
    face: Face
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} of {:?}", self.face, self.suit)
    }
}

/// The deck. The deck contains a list of cards (52).
/// Also has a RNG assigned to it for shuffling.
struct Deck {
    cards: Vec<Card>,
    rng: StdRng  //Make a RNG available to our struct
}

impl Deck {
    /// Creates a new deck of 52 cards. 
    ///
    /// @return Deck    A newly formed deck of 52 cards
    pub fn new() -> Deck {
        let mut v: Vec<Card> = Vec::new(); // create a mutable vector to be filled with cards

        for suit in [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades].iter() {
            for face in Face::as_array().iter() {
                v.push(Card{ suit: *suit, face: *face});  // Fill the deck with one face card of each suit
            }
        }

        Deck { cards: v, rng: StdRng::new().unwrap() }
    }

    /// The deck shuffles itself.
    pub fn shuffle(&mut self) {
        self.rng.shuffle(&mut self.cards)
    }

    /// Deals a hand of cards.  The cards are removed from the deck so they can't be dealt again
    ///
    /// @param  self:       self    The deck
    /// @param  c:          u32     The number of cards to deal out.
    ///
    /// @return Hand                The cards that were dealt
    pub fn deal(&mut self, c: u32) -> Hand {
        let mut hand: Vec<Card> = Vec::new(); // Vec to hold the cards of the hand
        for _ in 0..c { //deal 2 cards to each player
            match self.cards.pop() {
                Some(card) => hand.push(card),
                None       => panic!("No cards left in deck.")
            }
        }
        Hand { cards: hand }
    }
}

struct Hand {
    cards: Vec<Card>
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_card: bool = true;
        for card in &self.cards {
            if !first_card {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}", card));
            first_card = false;
        }

        Ok(())
    }
}

struct Player {
    hand: Hand,
    name: String
}

impl Player {
    pub fn new(name: String, hand: Hand) -> Player {
        Player { name: name, hand: hand }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "{}'s hand: {}", self.name, self.hand));
        Ok(())
    }
}

/// Deals cards to the players.
///
/// @param  player_count: u32   The number of players to deal cards out to
/// @param  card_count:   u32   The number of cards to deal to each player
/// @param  mut deck:     Deck  The deck that we're working off of.  Want to make sure we're always
///                             dealing from the same deck!  Mutable since we're going to deal cards
///
/// @return (Vec<Player>, Deck) Returns a vector of the players that we dealt to and the deck.
///                             We need to return the deck back to main(), so we can use it again
fn deal_to_players(player_count: u32, card_count: u32, mut deck: Deck) -> (Vec<Player>, Deck) {
    let mut players = Vec::new();
    for p in 0..player_count {
        let name = if p == 0 {
            "Player".to_string()
        } else {
            "Computer ".to_string() + &p.to_string()
        };

        players.push(Player::new(name, (deck.deal(card_count))));
    }

    (players, deck)
}

/// Deals cards to the table.
/// First deals the 3 cards flop, then the 1 card turn and river.
/// In between dealing, checks to see if anyone folds.  Finally, checks the winner.
///
/// @param players:     Vec<Player>     The players at the table. 
/// @param  mut deck:   Deck            The deck that we're working off of.  Want to make sure we're always
///                                     dealing from the same deck!  Mutable since we're going to deal cards
///
/// @return Deck                        Returns a vector of the players that we dealt to and the deck.
///                                     We need to return the deck back to main(), so we can use it again
fn deal_to_table(mut players: Vec<Player>, mut deck: Deck) {
    let table: Hand = deck.deal(5);
    println!("");
    println!("Flop: {}, {}, {}", table.cards[0], table.cards[1], table.cards[2]);
    fold(players, &table);
    println!("Turn: {}", table.cards[3]);
    // fold();
    println!("River: {}", table.cards[4]);
    winner();
}

/// Checks to see if any player is going to fold.
///
/// @param players:     Vec<Player>     The players at the table.
///
/// @TODO: this should make a copy of players, and only remove them from this round of dealing.
fn fold(mut players: Vec<Player>, table: &Hand) -> Vec<Player> {
    let check_if_fold = |player: &Player| -> bool {

        true
    };

    for player in players.iter() {
        if check_if_fold(player) {
            println!("{} folds!", player.name);
        }
    };

    players
}

/// Checks to see who wins.
///
/// @param players:     Vec<Player>     The players at the table.
///
fn winner(mut players: Vec<Player>, table: &Hand) {
    let get_score = |player: &Player| -> i32 {

        true
    };

    for player in players.iter() {
        let score = get_score(player);
    };

    players
}

fn main() {
    let mut deck: Deck = Deck::new();
    deck.shuffle();

    // @todo: players and cards should be validated
    println!("Welcome to the World Series of Poker.");
    println!("How many players? (2 - 8)");
    let mut players = String::new();
    stdin().read_line(&mut players).unwrap();
    let player_count: u32 = players.trim().parse().unwrap();

    println!("How many cards per player? (2 - 5)");
    let mut cards = String::new();
    stdin().read_line(&mut cards).unwrap();
    let card_count: u32 = cards.trim().parse().unwrap();
    println!("");

    // Tuple because the deck is returned to give back ownership.
    let (players, deck) = deal_to_players(player_count, card_count, deck);

    for player in players.iter() {
        println!("{}", player);
    }

    deal_to_table(players, deck);
}

