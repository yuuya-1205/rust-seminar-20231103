#[derive(Debug)]
enum MyError {
    IOError,
    NotFound,
}

fn get_first_cell(path: &str) -> Result<String, MyError> {
    // read_to_stringが返すResultのエラー型はstd::io::Errorでget_first_cell関数が返すResultのエラー型と不一致
    // map_errでエラーの型をMyErrorに変換すると?演算子で早期リターンできる。
    let content = std::fs::read_to_string(path).map_err(|_| MyError::IOError)?;

    // split_once()はOptionを返すが、もしNoneを返したときはエラーにしたい。
    // ok_or_elseを使うとOptionをResult型に変換できる。Some(T)はOk(T)に、
    // Noneのときは引数のクロージャが実行されてErr(E)に変換される。
    let cell = content.split_once(',').ok_or_else(|| MyError::NotFound)?;

    return Ok(String::from(cell.0));
}

fn main() {
    let result = get_first_cell("/path/hoge.csv");
    match result {
        Ok(cell) => {
            println!("first cell: {}", cell)
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
