fn main() {
    let s = vec!["a".to_string(), "b".to_string(), "c".to_string()];
    let t = s;

    // この時点でデータの所有権は t に移っている
    // 所有権を奪われた s は未初期化状態になる

    // 未初期化状態の s から所有権を移動しようするとコンパイルエラー
    let u = s;
}
