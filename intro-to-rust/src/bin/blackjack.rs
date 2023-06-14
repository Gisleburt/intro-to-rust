use device_query::{DeviceQuery, DeviceState, Keycode};
use intro_to_rust::BlackjackError;
use intro_to_rust::BlackjackHand;
use intro_to_rust::Deck;
use std::cmp::Ordering;

enum GameResult {
    PlayerWins,
    PlayerBust,
    PlayerBlackjack,
    DealerWins,
    DealerBust,
    Tie,
}

impl ToString for GameResult {
    fn to_string(&self) -> String {
        match self {
            GameResult::PlayerWins => "You win, congratulations!".to_string(),
            GameResult::PlayerBust => "You went bust, dealer wins".to_string(),
            GameResult::PlayerBlackjack => "Blackjack! You win!".to_string(),
            GameResult::DealerWins => "Dealers hand wins".to_string(),
            GameResult::DealerBust => "Dealer bust, you win".to_string(),
            GameResult::Tie => "Tie, dealer wins".to_string(),
        }
    }
}

fn prep_dealer(deck: &mut Deck) -> Result<BlackjackHand, BlackjackError> {
    let mut dealer = BlackjackHand::new();

    while dealer.value() < 17 {
        dealer.add(deck.take().unwrap())?;
    }

    Ok(dealer)
}

fn prep_player(deck: &mut Deck) -> Result<BlackjackHand, BlackjackError> {
    let mut player = BlackjackHand::new();

    // Add two cards to the players hand
    player.add(deck.take().unwrap())?;
    player.add(deck.take().unwrap())?;

    Ok(player)
}

fn player_info(player: &BlackjackHand) {
    println!("Your hand is: {} ({})", player, player.value());
    println!("Do you (H)it or (S)tick");
}

fn determine_end_state(dealer: &BlackjackHand, player: &BlackjackHand) -> GameResult {
    match (dealer.value().cmp(&player.value()), player.value()) {
        (Ordering::Less, x) if x == BlackjackHand::max() => GameResult::PlayerBlackjack,
        (Ordering::Less, _) => GameResult::PlayerWins,
        (Ordering::Greater, _) => GameResult::DealerWins,
        (Ordering::Equal, _) => GameResult::Tie,
    }
}

fn game() -> GameResult {
    let mut deck = Deck::new();
    deck.shuffle();

    let Ok(dealer) = prep_dealer(&mut deck) else {
        return GameResult::DealerBust;
    };
    println!("The dealers hand is: {} ({})", dealer, dealer.value());

    let Ok(mut player) = prep_player(&mut deck) else {
        // This should never happen but its safer to deal with this than a potential programming mistake
        return GameResult::PlayerBust;
    };
    player_info(&player);

    loop {
        let device_state = DeviceState::new();
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            match key {
                Keycode::Y | Keycode::H => {
                    if player.add(deck.take().unwrap()).is_err() {
                        return GameResult::PlayerBust;
                    }
                    player_info(&player);
                }
                Keycode::N | Keycode::S => return determine_end_state(&dealer, &player),
                _ => {}
            }
        }
    }
}

fn main() {
    let result = game();
    println!("{}", result.to_string());
}
