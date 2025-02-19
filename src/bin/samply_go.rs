use sisyphus32::{fen::FenString, search::Search};

fn main() {
    let mut search = Search::default();
    search.go(&FenString::kiwipete().parse().unwrap(), 7, None);
}
