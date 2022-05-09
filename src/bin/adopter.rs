pub trait Print {
    fn print_weak(&self);
    fn print_strong(&self);
}

struct Banner {
    string: String,
}

impl Banner {
    fn show_with_paren(&self) {
        println!("({})", self.string)
    }
    fn show_with_astr(&self) {
        println!("*{}*", self.string)
    }
}

impl Print for Banner {
    fn print_weak(&self) {
        self.show_with_paren();
    }
    fn print_strong(&self) {
        self.show_with_astr();
    }
}

fn main() {
    let b = Banner {
        string: String::from("hoge"),
    };
    b.print_weak();
    b.print_strong();
}
