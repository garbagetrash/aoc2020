use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GameState {
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
}

#[derive(Debug, Clone)]
pub struct GameResult {
    winner: usize,
    deck: VecDeque<usize>,
}

#[aoc_generator(day22)]
pub fn load_input(input: &str) -> GameState {
    let players: Vec<_> = input.split("\n\n").collect();

    let mut player1: VecDeque<usize> = VecDeque::new();
    for line in players[0].lines().skip(1) {
        player1.push_back(line.parse().unwrap());
    }

    let mut player2: VecDeque<usize> = VecDeque::new();
    for line in players[1].lines().skip(1) {
        player2.push_back(line.parse().unwrap());
    }

    GameState { player1, player2 }
}

#[aoc(day22, part1)]
pub fn part1(input: &GameState) -> usize {
    let mut state = input.clone();

    loop {
        if state.player1.is_empty() || state.player2.is_empty() {
            // Game over, break out of the loop
            break;
        }

        let p1 = state.player1.pop_front().unwrap();
        let p2 = state.player2.pop_front().unwrap();

        if p1 > p2 {
            state.player1.push_back(p1);
            state.player1.push_back(p2);
        } else {
            state.player2.push_back(p2);
            state.player2.push_back(p1);
        }
    }

    // Calc score
    let mut acc = 0;
    if state.player1.is_empty() {
        // Player 2 wins
        for i in 0..state.player2.len() {
            acc += (i + 1) * state.player2.pop_back().unwrap();
        }
    } else {
        // Player 1 wins
        for i in 0..state.player1.len() {
            acc += (i + 1) * state.player1.pop_back().unwrap();
        }
    }

    acc
}

static mut GAME_CNTR: usize = 0;

pub fn play_game(state: &GameState, game_cache: &mut HashMap<GameState, GameResult>) -> GameResult {
    unsafe {
        GAME_CNTR += 1;
        if GAME_CNTR % 1000 == 0 {
            println!("{}", GAME_CNTR);
        }
    }

    // Consult the oracle
    if let Some(result) = game_cache.get(state) {
        return result.clone();
    }

    let mut gstate = state.clone();

    let mut played_before = HashSet::new();
    loop {
        if gstate.player1.is_empty() {
            // Player 2 wins
            let res = GameResult {
                winner: 2,
                deck: gstate.player2,
            };
            game_cache.insert(state.clone(), res.clone());
            return res;
        }

        if gstate.player2.is_empty() {
            // Player 1 wins
            let res = GameResult {
                winner: 1,
                deck: gstate.player1,
            };
            game_cache.insert(state.clone(), res.clone());
            return res;
        }

        if played_before.contains(&gstate) {
            // Player 1 insta wins
            let res = GameResult {
                winner: 1,
                deck: gstate.player1,
            };
            game_cache.insert(state.clone(), res.clone());
            return res;
        } else {
            played_before.insert(gstate.clone());
        }

        let p1 = gstate.player1.pop_front().unwrap();
        let p2 = gstate.player2.pop_front().unwrap();

        // Check if both players have enough cards to start a sub game...
        if gstate.player1.len() >= p1 && gstate.player2.len() >= p2 {
            // Yes, recurse a sub game!

            let res = play_game(&gstate, game_cache);

            if res.winner == 1 {
                gstate.player1.push_back(p1);
                gstate.player1.push_back(p2);
            } else {
                gstate.player2.push_back(p2);
                gstate.player2.push_back(p1);
            }
        } else {
            // No, play normally
            if p1 > p2 {
                gstate.player1.push_back(p1);
                gstate.player1.push_back(p2);
            } else {
                gstate.player2.push_back(p2);
                gstate.player2.push_back(p1);
            }
        }
    }
}

#[aoc(day22, part2)]
pub fn part2(input: &GameState) -> usize {
    let mut game_cache: HashMap<GameState, GameResult> = HashMap::new();
    let mut res = play_game(input, &mut game_cache);

    // Calc score
    let mut acc = 0;
    for i in 0..res.deck.len() {
        acc += (i + 1) * res.deck.pop_back().unwrap();
    }

    acc
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn test_part1() {
        let input = read_to_string("input/22a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part1(&input), 306);
    }

    #[test]
    fn test_part2() {
        let input = read_to_string("input/22a.txt").unwrap();
        let input = load_input(&input);
        assert_eq!(part2(&input), 291);
    }
}
