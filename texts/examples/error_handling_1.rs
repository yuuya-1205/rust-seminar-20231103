#[derive(Debug)]
enum MyError {
    IOError,
    UnExpectedError,
}

fn get_first_cell(path: &str) -> Result<String, MyError> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(_) => {
            return Err(MyError::IOError);
        }
    };

    let cell = match content.split_once(',') {
        Some(tuple) => tuple.0,
        None => {
            return Err(MyError::UnExpectedError);
        }
    };

    return Ok(String::from(cell));
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
