pub trait Iterator {
    type Item;
    fn has_next(&self) -> bool;
    fn next(&mut self) -> &Self::Item;
}

struct Music {
    title: String,
}

struct Album {
    title: String,
    musics: Vec<Music>,
}

struct AlbumIterator<'a> {
    album: &'a Album,
    index: usize,
}

impl Album {
    fn iter(&self) -> AlbumIterator {
        AlbumIterator {
            album: self,
            index: 0,
        }
    }
}

impl Iterator for AlbumIterator<'_> {
    type Item = Music;
    fn has_next(&self) -> bool {
        if self.index < self.album.musics.len() {
            return true;
        }
        false
    }

    fn next(&mut self) -> &Self::Item {
        let result = &self.album.musics[self.index];
        self.index += 1;
        return result;
    }
}

fn main() {
    let song_a = Music {
        title: String::from("hoge"),
    };
    let song_b = Music {
        title: String::from("fuga"),
    };
    let song_c = Music {
        title: String::from("foo"),
    };
    let song_d = Music {
        title: String::from("bar"),
    };

    let album = Album {
        title: String::from("hogera"),
        musics: vec![song_a, song_b, song_c, song_d],
    };
    println!("Album:{}", album.title);

    let mut it = album.iter();
    while it.has_next() {
        let music = it.next();
        println!("  {}", music.title)
    }
}
