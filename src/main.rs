trait Jieshao {
    fn jieshao(&self) -> String;
}

struct Maozi {
    yanse: String,
}

struct Xiezi {
    chicun: i32,
}

impl Jieshao for Maozi {
    fn jieshao(&self) -> String {
        format!("一顶{}的帽子。", self.yanse)
    }
}

impl Jieshao for Xiezi {
    fn jieshao(&self) -> String {
        format!("一双{}码的鞋子。", self.chicun)
    }
}

fn show(item: &impl Jieshao) {
    println!("{}", item.jieshao());
}

fn main() {
    let redhat = Maozi {
        yanse: String::from("红色"),
    };

    let nikeshoes = Xiezi { chicun: 42 };

    show(&redhat);
    show(&nikeshoes);
}
