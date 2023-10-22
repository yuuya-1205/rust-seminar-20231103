// XはA trait, B traitを実装するが,
// YはA traitのみ実装し、B traitを実装しない

// 飛ぶ
// 食べる

trait A {}
trait B {}

struct X;

impl A for X {}
impl B for X {}

impl std::fmt::Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct Y;

impl A for Y {}

// 関数callはA trait と　B traitを要求する
// これをトレイト境界（train bound）という
fn call<T>(input: &T)
where
    T: A + B,
{
    // do something
}

// こういう書き方もできる
fn call2<T: A + B>(input: &T) {
    // do something
}

fn main() {
    let x = X {};
    let y = Y {};

    call(&x);
    // これはコンパイルエラー
    // call(y);
    //
    // the trait bound `Y: B` is not satisfied
    // the trait `B` is implemented for `X`rustcClick for full compiler diagnostic
    // trait.rs(27, 5): required by a bound introduced by this call
    // trait.rs(18, 12): required by a bound in `call`

    // {}はstd::fmt::Display トレイトを要求する
    println!("{}", x);

    // TODO:
    // 動的ディスパッチ
    // 静的ディスパッチ
    // [安定化間近！Rustのimpl Traitを今こそ理解する - 簡潔なQ](https://qnighy.hatenablog.com/entry/2018/01/28/220000)
}
