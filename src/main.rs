use water_jug_problem::game::*;
use water_jug_problem::beam_search;
use water_jug_problem::best_first_search;
use water_jug_problem::breadth_first_search;
use water_jug_problem::random_search;

fn print_answer(game: &Game, answer: &[Action]) {
    let mut state = game.initial_state();

    for action in answer {
        state = game.next_state(&state, &action);

        println!("{:?}, {:?}", action, state.pitchers());
    }
}

fn main() {
    let game = Game::new(&[8, 5, 3]);

    println!("# random search");
    if let Some(answer) = random_search::answer(&game) {
        print_answer(&game, &answer);
    } else {
        println!("no answer...");
    }
    println!("");

    println!("# breadth first search");
    if let Some(answer) = breadth_first_search::answer(&game) {
        print_answer(&game, &answer);
    } else {
        println!("no answer...");
    }
    println!("");

    println!("# best first search");
    if let Some(answer) = best_first_search::answer(&game) {
        print_answer(&game, &answer);
    } else {
        println!("no answer...");
    }
    println!("");

    println!("# beam search");
    if let Some(answer) = beam_search::answer(&game, 2) {
        print_answer(&game, &answer);
    } else {
        println!("no answer...");
    }
    println!("");
}
