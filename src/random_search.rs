use rand::*;
use rand::seq::*;
use super::game::*;

pub fn answer(game: &Game) -> Option<Vec<Action>> {
    // ランダム・ナンバー・ジェネレーター。
    let mut rng = thread_rng();

    // 初期状態を取得します。
    let mut state = game.initial_state();

    // 解答の入れ物を作成します。
    let mut answer = Vec::new();

    // 10000回ほど……
    for _ in 0..10000 {
        // 合法手の中からランダムで選んで、
        let action = game.legal_actions(&state).into_iter().choose(&mut rng).unwrap();

        // 次の状態に遷移します。
        state = game.next_state(&state, &action);

        // 解答も作成します。
        answer.push(action);

        // ゴールなら、解答をリターンします。
        if game.is_goal(&state) {
            return Some(answer);
        }
    }

    // 正解が見つからない場合は、Noneを返します。
    None
}
