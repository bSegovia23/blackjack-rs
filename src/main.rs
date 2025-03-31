use text_io::read;
mod objects;
use objects::{Hand, Deck};

fn main() {
    println!("Welcome to Blackjack!");
    println!("Dealer must draw to 16 and stand on all 17s");

    loop {
        let mut deck = Deck::new();
        deck.shuffle();

        let mut player_hand = Hand::new();
        let mut dealer_hand = Hand::new();

        // Initial deal: 2 cards each
        for _ in 0..2 {
            player_hand.add_card(deck.deal().expect("Deck ran out!"));
            dealer_hand.add_card(deck.deal().expect("Deck ran out!"));
        }

        let mut decisive_end: bool = false;

        // Player round
        loop {
            println!("\nDealer hand: ??, {}", dealer_hand.last_card().expect("Dealer hand empty!")); // dealer keeps first card concealed until he starts hitting
            println!("  Your hand: {}", player_hand);

            if player_hand.value() == 21 {
                if player_hand.is_blackjack() {
                    decisive_end = true;
                    println!("Blackjack!");
                    if dealer_hand.is_blackjack() {
                        println!("Dealer also has blackjack! It's a push!")
                    }
                }
                // If player has blackjack, autowin unless dealer also has blackjack.
                // Else if player still has 21, auto-stand. The dealer can only hope, at best, to match 21.
                break;
            }
            else if player_hand.value() == 0 {
                decisive_end = true;
                println!("Bust! You lose!");
                break;
            }
            else {
                println!("\nWhat would you like to do?");
                println!("1. Hit (H)");
                println!("2. Stand (S)");

                let choice: String = read!("{}\n");
                match choice.to_ascii_lowercase().trim() {
                    "1" | "hit" | "h" | "" => {
                        player_hand.add_card(deck.deal().expect("Deck ran out!"));
                    },
                    "2" | "stand" | "s" => {
                        break;
                    },
                    _ => {println!("Invalid command. Please try again.");}
                }
            }
        }

        // Dealer round
        if !decisive_end {
            loop {
                println!("\nDealer hand: {}", dealer_hand);
                println!("  Your hand: {}", player_hand);

                let (d, p) = (dealer_hand.value(), player_hand.value());
                match d {
                    0 => { println!("Dealer busted! You win!"); break; },
                    v if v < 17 => { dealer_hand.add_card(deck.deal().expect("Deck ran out!")); },
                    _ => {
                        // dealer stands
                        if d > p { println!("You lost!")}
                        else if d < p { println!("You won!"); }
                        else if d == p { println!("It's a push!"); }
                        break;
                    }
                }
            }
        }

        println!("Thank you for playing! Would you like to play again? (y/n)");
        let choice: String = read!("{}\n");
        match choice.to_ascii_lowercase().trim() {
            "y" | "yes" | "" => {},
            _ => { println!("Until next time!"); break; }
        }
    }
}
