pub trait Iterator {
    type Item;
    fn hasNext(&mut self) -> bool;
    fn next(&mut self) -> &Self::Item;
}

struct Music {
    id: u64,
    title: &'static str,
    artist: &'static str,
}

struct Album {
    curr: usize,
    next: usize,
    musics: Vec<&'static Music>,
}

impl Iterator for Album {
    type Item = Music;
    fn hasNext(&mut self) -> bool {
        true
    }

    fn next(&mut self) -> &Self::Item {
        let result = self.musics[self.curr];
        &result
    }
}

fn main() {
    println!("hoge")
}
