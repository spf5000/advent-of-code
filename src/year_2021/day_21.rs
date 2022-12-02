pub struct Day21 {}

impl Default for Day21 {
    fn default() -> Self {
        Self { }
    }
}

impl crate::DayAnswers for Day21 {
    fn get_answer(&self, _question: crate::model::Question) -> anyhow::Result<()> {
        main()
    }
}

const PLAYER_1_START: u32 = 2;
const PLAYER_2_START: u32 = 2;

const MAX_SPACES: u32 = 10;
const MAX_DIE: u32 = 100;
const MAX_SCORE_1: u32 = 1000;
const MAX_SCORE_2: u32 = 21;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Player {
    score: u32,
    location: u32,
    max_score: u32
}

impl Player {
    fn new(location: u32, max_score: u32) -> Self {
        Self::new_with_score(location, 0, max_score)
    }

    fn new_with_score(location: u32, score: u32, max_score: u32) -> Self {
        Self { score, location, max_score }
     }

    fn move_spaces(&mut self, spaces: u32) {
        self.location += spaces % MAX_SPACES;
        self.location %= MAX_SPACES;
        self.score += if self.location == 0 { MAX_SPACES } else { self.location }
    }

    fn has_won(&self) -> bool {
        self.score >= self.max_score
    }
}

fn main() -> anyhow::Result<()> {
    // Puzzle 1
    let mut player_1 = Player::new(PLAYER_1_START, MAX_SCORE_1);
    let mut player_2 = Player::new(PLAYER_2_START, MAX_SCORE_1);

    let mut die = 1;
    let mut die_rolls = 0;
    let mut player_1_turn = true;
    while !player_1.has_won() && !player_2.has_won() {
        let spaces = roll_dice(&mut die, 3);
        die_rolls += 3;
        if player_1_turn {
            player_1.move_spaces(spaces);
        } else {
            player_2.move_spaces(spaces)
        }
        player_1_turn = !player_1_turn;
    }

    if player_1.has_won() {
        println!("Player 1 won! Puzzle answer is {}", player_2.score * die_rolls);
    } else {
        println!("Player 2 won! Puzzle answer is {}", player_1.score * die_rolls);
    }

    // Puzzle 2
    // let player_1 = Player::new(4, MAX_SCORE_2);
    // let player_2 = Player::new(8, MAX_SCORE_2);
    let player_1 = Player::new(PLAYER_1_START, MAX_SCORE_2);
    let player_2 = Player::new(PLAYER_2_START, MAX_SCORE_2);

    let (player_1_wins, player_2_wins) = play_quantom_game(player_1, player_2);
    println!("Quantom Game: Player 1: {}, Player 2: {}", player_1_wins, player_2_wins);
    if player_1_wins > player_2_wins {
        println!("Player 1 has more quantom wins with {}", player_1_wins);
    } else {
        println!("Player 2 has more quantom wins with {}", player_2_wins);
    }
    
    Ok(())
}

fn roll_dice(die_num: &mut u32, rolls: u8) -> u32 {
    let mut output = 0;
    for _ in 0..rolls {
        output += *die_num;
        *die_num += 1;
        if *die_num > MAX_DIE {
            *die_num -= MAX_DIE;
        }
    }
    output
}


// Collection of spaces to move and the number of times we get there with three rolls.
// See NOTES for explaining on how I came to these numbers.
// const QUANTOM_GAME_SIMPLIFIED: [(u32, u8); 1] = [(4, 3)];
const QUANTOM_GAME_SIMPLIFIED: [(u32, u8); 7] = [
    (3, 1), 
    (4, 3), 
    (5, 6), 
    (6, 7),
    (7, 6),
    (8, 3),
    (9, 1)
];

fn play_quantom_game(turn_player: Player, other_player: Player) -> (u128, u128) {
    // Other player's last move has won them the game
    if other_player.has_won() {
        return (0, 1);
    }
    
    // This is a quantom game, so there are 27 different roll outcomes with 7 different space
    // changes. We got through those outcomes to reduce the number of games we need to explore.
    // See NOTES at the bottom for explaination.
    let mut output = (0, 0);
    for (spaces, games) in QUANTOM_GAME_SIMPLIFIED.into_iter() {
        let mut clone = turn_player.clone();
        clone.move_spaces(spaces);
        let (other_player_wins, turn_player_wins) = play_quantom_game(other_player.clone(), clone);
        output.0 += turn_player_wins * games as u128;
        output.1 += other_player_wins * games as u128;
    }

    output
}

// NOTES:
// for puzzle 2, every "turn" creates 27 new universes:
// First roll: 1, 2, or 3 (3 new universes)
// Second roll: 1, 2, or 3 + the original 1, 2, or 3 (3 * 3 = 9 universes)
// Second roll: 1, 2, or 3 + the original 9 universes (3 * 3 = 27 universes)
// 
// Of those 27 universes, the values are:
// 1, 1, 1 = 3
// 1, 1, 2 = 4
// 1, 1, 3 = 5
// 1, 2, 1 = 4
// 1, 2, 2 = 5
// 1, 2, 3 = 6
// 1, 3, 1 = 5
// 1, 3, 2 = 6
// 1, 3, 3 = 7
//
// 2, 1, 1 = 4
// 2, 1, 2 = 5
// 2, 1, 3 = 6
// 2, 2, 1 = 5
// 2, 2, 2 = 6
// 2, 2, 3 = 7
// 2, 3, 1 = 6
// 2, 3, 2 = 7
// 2, 3, 3 = 8
//
// 3, 1, 1 = 5
// 3, 1, 2 = 6
// 3, 1, 3 = 7
// 3, 2, 1 = 6
// 3, 2, 2 = 7
// 3, 2, 3 = 8
// 3, 3, 1 = 7
// 3, 3, 2 = 8
// 3, 3, 3 = 9
//
// Condense this down and you have
// 3: once
// 4: three times
// 5: six times
// 6: seven times
// 7: six times
// 8: three times
// 9: one time
// 1 + 3 + 6 * 2 = 20 + 7 = 27 woot!


