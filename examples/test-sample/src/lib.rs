// 単体テスト

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// testsモジュールを定義し、#[cfg(test)]マクロをつけてテスト時のみビルド対象にする
#[cfg(test)]
mod tests {
    // add関数はtestsモジュールの親にあたるのでsuperでパスをインポートする
    use super::*;

    // テスト関数には#[test]マクロをつける
    #[test]
    fn it_works() {
        let result = add(2, 2);

        // assert_eq!マクロで左辺と右辺を比較する。
        // 左辺と右辺が一致しなければpanicしてテスト失敗
        assert_eq!(result, 4);
    }
}
