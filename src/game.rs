use std::cmp::*;
use std::iter::*;
use itertools::iproduct;

// ゲーム。今回は、ピッチャーの容量の集合。
pub struct Game {
    pitcher_capacities: Vec<i32>
}

// 状態。今回は、ピッチャーに入っているヨーグルト・スムージーの量の集合。
#[derive(Clone, Eq, Hash, PartialEq)]
pub struct State {
    pitchers: Vec<i32>
}

// 手。今回は、移動元と移動先のピッチャーのインデックス。実装が面倒だったので、型エイリアスで。
pub type Action = (usize, usize);

impl Game {
    // 新しいゲームを作成します。
    pub fn new(pitcher_capacities: &[i32]) -> Game {
        Game {
            pitcher_capacities: pitcher_capacities.to_vec()
        }
    }

    // ピッチャーの容量の集合を取得します。
    pub fn pitcher_capacities(&self) -> &Vec<i32> {
        &self.pitcher_capacities
    }

    // 初期状態を取得します。
    pub fn initial_state(&self) -> State {
        State {
            pitchers: self.pitcher_capacities.iter().take(1).copied().chain(repeat(0)).take(self.pitcher_capacities.len()).collect()  // 最初のピッチャーはいっぱい、他は空。
        }
    }

    // 合法手一覧を取得します。
    pub fn legal_actions(&self, state: &State) -> Vec<Action> {
        iproduct!(0..self.pitcher_capacities.len(), 0..self.pitcher_capacities.len()).filter(|&(f, t)| {
            f != t && state.pitchers[f] != 0 && state.pitchers[t] != self.pitcher_capacities[t]  // ピッチャーが異なっていて、移動元が空でなくて、移動先がいっぱいでないなら、合法
        }).collect()
    }

    // 次の状態（stateにactionを実行した結果）を取得します。
    pub fn next_state(&self, state: &State, action: &Action) -> State {
        let mut next_pitchers = state.pitchers.clone();

        let &(f, t) = action;

        next_pitchers[f] = max(state.pitchers[f] - (self.pitcher_capacities[t] - state.pitchers[t]), 0);
        next_pitchers[t] = min(state.pitchers[t] + state.pitchers[f], self.pitcher_capacities[t]);

        State {
            pitchers: next_pitchers
        }
    }

    // ゴールに到達したかチェックします。
    pub fn is_goal(&self, state: &State) -> bool {
        state.pitchers.iter().any(|&pitcher| pitcher == self.pitcher_capacities[0] / 2)  // 最初のピッチャーのサイズの半分のピッチャーがあるなら、はんぶんこ成功！
    }
}

impl State {
    // ピッチャーに入っているヨーグルト・スムージーの量の集合を取得します。
    pub fn pitchers(&self) -> &Vec<i32> {
        &self.pitchers
    }
}
