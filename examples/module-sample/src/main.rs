// ```
// $ tree ./src
// ./src
// ├── main.rs
// ├── module_a.rs
// ├── module_b
// │   ├── module_b_x.rs
// │   └── module_b_y.rs
// └── module_b.rs
// ```

mod module_a;
mod module_b;

// use キーワードでパスをスコープに持ち込める。そうすると
// module_b::module_b_x::b_x() と書かずに
// module_b_x::b_x() と短く書ける。
use module_b::*;

fn main() {
    module_a::a_pub_crate();
    module_a::a_public();
    b();
    module_b_x::b_x();

    // b_y()はプライベートなので呼べない
    // module_b_y::b_y();

    let _by = module_b_y::BY {};
}
