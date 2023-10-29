trait Player {
    // 第１引数が&selfなので、自身を参照する。
    fn name(&self) -> String;

    // トレイト定義中に関数の実体を書けばこれがデフォルト実装になる。
    // このトレイトを実装する側がオーバーライドしなければデフォルト実装が呼び出される。
    fn show_name(&self) {
        println!("My name is {}", self.name());
    }

    // 第１引数がselfなので所有権を奪う
    fn damaged(self, damage: u32) -> Self;

    // 引数にself系を含まない関数はPlayer::new()で呼び出せる
    fn new(name: String) -> Self;
}

struct Fighter {
    name: String,
    current_hp: u32,
}

// 構造体 Fighter に Player トレイトを実装
impl Player for Fighter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn damaged(self, damage: u32) -> Self {
        Self {
            current_hp: self.current_hp - damage,
            ..self
        }
    }

    fn new(name: String) -> Self {
        Fighter {
            name,
            current_hp: 100,
        }
    }
}

fn main() {
    let kirby = Fighter::new("Kirby".to_string());

    kirby.show_name();

    kirby.damaged(10);
}
