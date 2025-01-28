mod anything;
mod macros;

pub use anything::Anything;

#[cfg(test)]
mod tests {
    use super::*;

    use std::f32::consts::PI;

    #[derive(Debug, PartialEq)]
    struct Foo(i32);

    #[test]
    pub fn test_insert() {
        let mut anything = Anything::new();
        anything.insert(1i32);
        anything.insert(String::from("hello world"));
        if let Some(val) = anything.get::<i32>() {
            assert_eq!(val, &1i32);
        }
        if let Some(val) = anything.get::<String>() {
            assert_eq!(val, "hello world");
        }
    }

    #[test]
    pub fn test_add_anything() {
        let mut anything = Anything::new();
        add_anything!(anything, PI, Foo(23));
        if let Some(val) = anything.get::<f32>() {
            assert_eq!(val, &PI);
        }
        if let Some(val) = anything.get::<Foo>() {
            assert_eq!(val, &Foo(23));
        }
    }

    #[test]
    pub fn test_create_anything() {
        let anything = create_anything!(12i32, PI, Foo(23));
        if let Some(val) = anything.get::<i32>() {
            assert_eq!(val, &12i32);
        }
        if let Some(val) = anything.get::<f32>() {
            assert_eq!(val, &PI);
        }
        if let Some(val) = anything.get::<Foo>() {
            assert_eq!(val, &Foo(23));
        }
    }
}
