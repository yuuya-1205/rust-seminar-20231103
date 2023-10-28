// 単一のファイルはファイル名がそのままモジュール名になる

// デフォルトの公開範囲はプライベート
fn a_private() {}

// pub キーワードをつけると公開される
pub fn a_public() {}

// pub(crate) キーワードをつけると、このクレート内のみ公開になる
pub(crate) fn a_pub_crate() {}
