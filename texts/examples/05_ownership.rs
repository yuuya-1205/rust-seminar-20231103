struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    let s = Color {
        r: 49,
        g: 78,
        b: 91,
    };
    let t = s;

    // この時点でデータの所有権は t に移っている
    // 所有権を奪われた s は未初期化状態になる

    // 未初期化状態の s から所有権を移動しようするとコンパイルエラー
    // let u = s;
}
