#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error("ここにエラーメッセージを書ける")]
    IOError(#[from] std::io::Error),
    #[error("'='が見つかりませんでした")]
    NotFound,
}

// 戻り値がanyhowのResultになった
fn get_first_cell(path: &str) -> anyhow::Result<String> {
    let content = std::fs::read_to_string(path).map_err(|e| MyError::IOError(e))?;
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
