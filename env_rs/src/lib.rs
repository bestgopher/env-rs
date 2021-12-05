extern crate env_derive;

pub use env_derive::FromEnv;

pub trait FromEnv: Sized {
    fn from_env() -> Result<Self, String>;
}

#[cfg(test)]
mod tests {
    use super::FromEnv;
    use std::fmt::Debug;
    use std::net::IpAddr;
    use std::str::FromStr;

    #[test]
    fn env_un_attr() {
        #[derive(FromEnv, Eq, PartialEq, Debug)]
        struct A<T, F>
        where
            F: Debug + FromStr,
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

        let a = A::from_env().unwrap();
        assert_eq!(
            A {
                name: "你好".to_string(),
                age: 100,
                t: 1231312,
                f: 3432432,
            },
            a
        );
    }

    #[test]
    fn env_rename_attr() {
        #[derive(FromEnv, Eq, PartialEq, Debug)]
        struct A<T, F>
        where
            F: Debug + FromStr,
            T: Debug + FromStr,
        {
            #[env(rename = "NAME")]
            name: String,
            age: i32,
            t: T,
            f: F,
        }

        std::env::set_var("name", "你好");
        std::env::set_var("NAME", "你好1");
        std::env::set_var("age", "100");
        std::env::set_var("t", "1231312");
        std::env::set_var("f", "3432432");

        let a = A::from_env().unwrap();
        assert_eq!(
            A {
                name: "你好1".to_string(),
                age: 100,
                t: 1231312,
                f: 3432432,
            },
            a
        );
    }

    #[test]
    fn env_default_val() {
        #[derive(FromEnv, Eq, PartialEq, Debug)]
        struct A<T, F>
        where
            F: Debug + FromStr,
            T: Debug + FromStr,
        {
            #[env(default = "bestgopher")]
            name: String,
            #[env(default = "842131")]
            age: i32,
            #[env(default = "127.0.0.1")]
            t: T,
            #[env(default = "12.32")]
            f: F,
        }

        let a = A::from_env().unwrap();
        assert_eq!(
            A {
                name: "bestgopher".to_string(),
                age: 842131,
                t: IpAddr::from_str("127.0.0.1").unwrap(),
                f: 12.32f64,
            },
            a
        );
    }
}
