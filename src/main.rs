trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}

struct Tweet {
    username: String,
    body: String,
}


impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Book name: {}, Author name: {}.", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{} tweets out: {}.", self.username, self.body)
    }
}

fn print_summary<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}

fn main() {
    let aa = Article {
        title: String::from("Rust入门"),
        author: String::from("作者A"),
    };

    let bb = Tweet {
        username:  String::from("qwerty"),
        body: String::from("rust for zk + crypto"),
    };

    print_summary(&aa);
    print_summary(&bb);
}