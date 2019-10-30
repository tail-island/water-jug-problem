use std::collections::*;
use std::cmp::*;
use super::game::*;

// 次の解答を取得します。clone()してpush()するだけでもよいのですけど、少しだけ、メモリ効率を考えてみました。
fn next_answer(answer: &[Action], action: &Action) -> Vec<Action> {
    let mut result = Vec::with_capacity(answer.len() + 1);

    result.extend_from_slice(answer);
    result.push(action.clone());

    result
}

// 評価関数。元ネタと同じ。
fn score(game: &Game, state: &State, answer: &[Action]) -> i32 {
    let target = game.pitcher_capacities()[0] / 2;

    -(answer.len() as i32) + state.pitchers().iter().fold(0, |acc, pitcher| acc + if pitcher % target == 0 { 10 } else { 0 } - (target - pitcher).abs())
    // -(answer.len() as i32) - (state.pitchers().iter().map(|pitcher| (target - pitcher).abs()).min().unwrap())
    // -(answer.len() as i32)
}

// 最良優先探索。
pub fn answer(game: &Game) -> Option<Vec<Action>> {
    // ノード。評価結果を入れるscoreを追加しました。
    struct Node {
        state:  State,
        answer: Vec<Action>,
        score:  i32
    }

    // BinaryHeap（priority queueが必要な場合はこれを使えとRustのドキュメントに書いてあった）で使えるように、PartialEqとEq、PartialOrd、Ordを実装します。
    impl PartialEq for Node {
        fn eq(&self, other: &Self) -> bool {
            self.score == other.score
        }
    }
    impl Eq for Node {}
    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(&other))
        }
    }
    impl Ord for Node {
        fn cmp(&self, other: &Self) -> Ordering {
            self.score.cmp(&other.score)
        }
    }

    // 優先度付きキューと探索済みの集合を管理するためのセットを作成します。
    let mut queue   = BinaryHeap::new();
    let mut visited = HashSet::new();

    // キューに初期状態を追加します。
    queue.push(Node {
        state:  game.initial_state(),
        answer: Vec::new(),
        score:  0
    });

    // 初期状態を探索済みにしておきます。
    visited.insert(game.initial_state());

    // キューが空でなければ、処理を繰り返します。
    while !queue.is_empty() {
        // 次のノードを取得します。
        let node = queue.pop().unwrap();

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

                // 次のスコアを取得します。
                let next_score = score(&game, &next_state, &next_answer);

                // 新しいノードをキューに追加します。
                queue.push(Node {
                    state:  next_state,
                    answer: next_answer,
                    score:  next_score
                });
            }
        }
    }

    // 正解が見つからない場合は、Noneを返します。
    None
}
