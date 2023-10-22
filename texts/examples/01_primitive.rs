fn main() {
    // 真偽値
    //
    let t = true;
    let f = false;

    // and演算子
    let and = true && true;
    // or演算子
    let or = true || false;

    // 数値
    //
    // 整数のデフォルトはi32型
    let x = 1;
    // _の後ろに型名を書くこともできる
    // `let y: i64 = 2;`と同じ
    let y = 2_i64;
    // _で区切ることで読みやすくすることが出来る
    let z = 100_000_000;

    // 小数部分を含む数値のデフォルトはf64型
    let a = 1.5;
    // 浮動小数点型はf64とf32のみ

    // usize(isize) は配列やベクタの要素にアクセスするときに登場する

    // 大きさ	 符号付き	符号なし
    // --------------------------
    // 8-bit	i8	      u8
    // 16-bit	i16	      u16
    // 32-bit	i32	      u32
    // 64-bit	i64	      u64
    // arch	    isize	  usize

    // 暗黙の型変換はされないのでi32とf64の加算はコンパイルエラー
    // let sum = x + a;

    // as でキャストするか、From（TryFrom）トレイト or Into（TryInto）トレイトを使う
    // ※トレイトについては後述
    let sum = (x as f64) + a;
    let sum = f64::try_from(x).unwrap() + a;
}
