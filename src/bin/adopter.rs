pub trait Print {
    fn printWeak(&self, string: &str);
    fn printStrong(&self, string: &str);
}

struct Banner {
    string: String,
}

impl Banner {
    fn showWithParen(&self) {
        println!("({})", self.string)
    }
    fn showWithAstr(&self) {
        println!("*{}*", self.string)
    }
}

fn main() {
    println!("hoge");
}
