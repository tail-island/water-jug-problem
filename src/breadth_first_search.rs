use std::collections::*;
use super::game::*;

// 次の解答を取得します。clone()してpush()するだけでもよいのですけど、少しだけ、メモリ効率を考えてみました。
fn next_answer(answer: &[Action], action: &Action) -> Vec<Action> {
    let mut result = Vec::with_capacity(answer.len() + 1);

    result.extend_from_slice(answer);
    result.push(action.clone());

    result
}

// 幅優先探索。
pub fn answer(game: &Game) -> Option<Vec<Action>> {
    // ノード。状態と、その状態に至る解答です。
    struct Node {
        state:  State,
        answer: Vec<Action>
    }

    // キューと探索済みの集合を管理するためのセットを作成します。
    let mut queue   = VecDeque::new();
    let mut visited = HashSet::new();

    // キューに初期状態を追加します。
    queue.push_back(Node {
        state:  game.initial_state(),
        answer: Vec::new()
    });

    // 初期状態を探索済みにしておきます。
    visited.insert(game.initial_state());

    // キューが空でなければ、処理を繰り返します。
    while !queue.is_empty() {
        // 次のノードを取得します。
        let node = queue.pop_front().unwrap();
        c += 1;

        // 合法手すべてでて……
        for action in game.legal_actions(&node.state) {
            // 次の状態を取得します。
            let next_state = game.next_state(&node.state, &action);

            // 次の状態が探索済みでなければ……
            if visited.insert(next_state.clone()) {
                // 次の解答を取得します。
                let next_answer = next_answer(&node.answer, &action);

                // ゴールなら、その解答をリターンして終了します。
                if game.is_goal(&next_state) {
                    return Some(next_answer);
                }

                // 新しいノードをキューに追加します。
                queue.push_back(Node {
                    state:  next_state,
                    answer: next_answer
                });
            }
        }
    }

    // 正解が見つからない場合は、Noneを返します。
    None
}
