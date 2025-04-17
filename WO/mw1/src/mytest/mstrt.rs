/*
Actuakl Struct test
*/
#[derive(Debug)]
struct Woman {
    boobs: String,
    pussy: String,
    ass: String,
}

impl Woman {
    fn lick(&self) -> String {
        format!(
            "Woman boobs {} and smelly {} giant {}",
            self.boobs, self.pussy, self.ass
        )
    }
}

pub fn struct_impl_test1() {
    pswg("Struct and Implementation Test".to_string());

    let woman1 = Woman {
        boobs: String::from("Big"),
        pussy: String::from("Hairy"),
        ass: String::from("fluffy"),
    };

    println!("{}", woman1.lick());
}
