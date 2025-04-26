use crate::logic::card;
use crate::logic::player;
// TODO: ta cel file urihtat

#[derive(Debug)]
pub enum Streets {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

pub struct Game {
    pub street: Streets,
    pub pot: u32,
    pub players: Vec<player::Player>,
    pub deck: Vec<card::Card>,
    pub table_cards: Vec<card::Card>,
    pub player_on_turn: player::PlayerPosition,
    pub round_number: u32,
}

pub fn init_game(player_list: Vec<player::Player>) -> Game {
    let deck = card::Card::make_ordered_deck();
    let deck = card::Card::scramble_deck(deck);
    let mut_player_list = player_list;
    Game {
        street: Streets::PreFlop,
        pot: 30,
        players: mut_player_list,
        deck, 
        table_cards: Vec::new(),
        player_on_turn: player::PlayerPosition::UnderTheGun,
        round_number: 0
    }
}

pub fn begin_round(game : &mut Game) {
    let deck = card::Card::make_ordered_deck();
    let mut deck = card::Card::scramble_deck(deck);
    for player in game.players.iter_mut() {
    player.position = player::PlayerPosition::next_player_position(&player.position);
    let card1 = match deck.pop() {
        None => card::Card {
            color: card::CardColor::Empty,
            number: card::CardNumber::Empty,
        },
        Some(card) => card,
    };
    let card2 = match deck.pop() {
        None => card::Card {
            color: card::CardColor::Empty,
            number: card::CardNumber::Empty,
        },
        Some(card) => card,
    };
    if player.position == player::PlayerPosition::SmallBlind {
        player.money -= 10;
    }  else if player.position == player::PlayerPosition::BigBlind {
        player.money -= 20;
    }
    player.cards = (card1, card2)
    }
}

pub fn make_bets<'a>(round: &mut Game, get_bet: fn(&mut player::Player) -> Option<u32>) -> Vec<(player::Names, u32)> {
    // TODO: return list of players that made bets
    let mut bets = Vec::new();
    for player in round.players.iter_mut() {
        if player.playing {
            match get_bet(player) {
                None => {
                    player.playing = false;
                },
                Some(bet) if bet <= player.money => {
                    player.money -= bet;
                    round.pot += bet;
                    bets.push((player.name.clone(), bet));
                }
                Some(bet) if bet > player.money => {
                    player.money = 0;
                    round.pot += bet;
                    bets.push((player.name.clone(), bet));
                }
                Some(_) => {
                    panic!("Nekaj je šlo narobe pri stavah");
                }
            }
        }
    }
    return bets;
}

pub fn make_flop(round: &mut Game) {
    for _ in 0..3 {
        let card = match round.deck.pop() {
            None => {panic!("Deck is empty")},
            Some(card) => card,
        };
        round.table_cards.push(card);
    }
}

pub fn make_turn(round: &mut Game) {
    let card = match round.deck.pop() {
        None => panic!("Deck is empty"),
        Some(card) => card,
    };
    round.table_cards.push(card);
}