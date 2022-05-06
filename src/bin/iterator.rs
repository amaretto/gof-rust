pub trait Iterator {
    type Item;
    fn hasNext(&mut self) -> bool;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Music {
    id: u64,
    title: &'static str,
    artist: &'static str,
}

struct Album {
    curr: usize,
    next: usize,
    musics: Vec<Music>,
}

impl Iterator for Album {
    type Item = Music;
    fn hasNext(&mut self) -> bool {
        true
    }

    fn next(&mut self) -> Option<Self::Item> {
        Some(&self.musics[self.curr])
    }
}

fn main() {
    println!("hoge")
}
