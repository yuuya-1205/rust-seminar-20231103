/// トレイト境界（trait bound）

trait Eat {
    fn eat(self);
}

trait Fly {
    fn fly() {}
}

// Bird は Eat も Fly も実装する

#[derive(Clone)]
struct Bird;

impl Eat for Bird {
    fn eat(self) {}
}
impl Fly for Bird {
    fn fly() {}
}

impl std::fmt::Display for Bird {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

// Dog は Eat のみ実装

struct Dog;

impl Eat for Dog {
    fn eat(self) {}
}

// 関数callはEat trait と Fly traitを要求する
// これをトレイト境界（train bound）という
fn call<T>(input: T)
where
    T: Eat + Fly,
{
    // Eat トレイトで定義されている関数が呼び出せる
    input.eat();
}

// こういう書き方もできる
fn call2<T: Eat + Fly>(input: T) {
    // do something
}

fn main() {
    let bird = Bird {};
    let dog = Dog {};

    call(bird.clone());
    // これはコンパイルエラー
    // call(&dog);
    //
    // the trait bound `Dog: Fly` is not satisfied
    // the trait `Fly` is implemented for `Bird`

    // println!マクロの {} は std::fmt::Display トレイトを要求する
    println!("{}", bird);

    bird.eat();

    // eat()は変数birdの所有権を奪うので、以下のコードはコンパイルエラー
    // bird.eat();

    // TODO:
    // 動的ディスパッチ
    // 静的ディスパッチ
    // [安定化間近！Rustのimpl Traitを今こそ理解する - 簡潔なQ](https://qnighy.hatenablog.com/entry/2018/01/28/220000)
}
