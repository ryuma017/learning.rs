fn main() {
    let mut user = {
        struct Yametaro<'a>(&'a str);
        impl std::convert::From<&'static str> for Yametaro<'_> {
            fn from(s: &'static str) -> Self {
                Self(s)
            }
        }
        impl std::fmt::Display for Yametaro<'static> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "やめ太郎")
            }
        }
        struct User<'a> {
            name: Yametaro<'a>,
        }
        User { name: "".into() }
    };

    user.name = "たかし".into();

    console::log(user.name);
}

mod console {
    pub fn log<T: std::fmt::Display>(v: T) {
        println!("{v}");
    }
}
