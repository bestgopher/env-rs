extern crate env_derive;

pub use env_derive::Parser;

#[inline(always)]
pub fn var<T: std::str::FromStr>(k: &str) -> Result<T, String> {
    std::env::var(k)
        .map_err(|e| format!("environment variable `{}` get failed!, err: {:?}", k, e))?
        .parse()
        .map_err(|_e| format!("environment variable `{}` get failed!", k))
}

pub trait Parser: Sized {
    fn parse() -> Result<Self, String>;
}

pub fn parse<T>() -> Result<T, String>
    where T: Parser
{
    T::parse()
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use std::net::IpAddr;
    use std::str::FromStr;
    use super::{Parser, parse, var};

    #[test]
    fn test_var() {
        std::env::set_var("a", "1");
        std::env::set_var("b", "1.0");
        std::env::set_var("c", "127.0.0.1");

        assert_eq!(1, var::<i32>("a").unwrap());
        assert_eq!(1.0, var("b").unwrap());
        assert_eq!(IpAddr::from_str("127.0.0.1").unwrap(), var::<IpAddr>("c").unwrap())
    }

    #[test]
    fn env_un_attr() {
        #[derive(Parser, Eq, PartialEq, Debug)]
        struct A<T, F>
            where F: Debug + FromStr,
                  T: Debug + FromStr,
        {
            name: String,
            age: i32,
            t: T,
            f: F,
        }

        std::env::set_var("name", "你好");
        std::env::set_var("age", "100");
        std::env::set_var("t", "1231312");
        std::env::set_var("f", "3432432");

        let a = parse().unwrap();
        assert_eq!(A { name: "你好".to_string(), age: 100, t: 1231312, f: 3432432 }, a);
    }
}