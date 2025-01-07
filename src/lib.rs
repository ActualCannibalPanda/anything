use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type RawMap<A> = HashMap<TypeId, Box<A>>;

pub trait ToBox<T: ?Sized + CastToT>: Any {
    fn to_box(self) -> Box<T>;
}

pub trait CastToT {
    fn type_id(&self) -> TypeId;

    unsafe fn downcast_ref<T>(&self) -> &T;
    unsafe fn downcast_mut<T>(&mut self) -> &mut T;
}

macro_rules! generate_implementation {
    ($t:ident $(+ $othert:ident)*) => {
        impl CastToT for dyn $t $(+ $othert)* {
            fn type_id(&self) -> TypeId {
                self.type_id()
            }

            unsafe fn downcast_ref<T>(&self) -> &T {
                &*(self as *const Self as *const T)
            }

            unsafe fn downcast_mut<T>(&mut self) -> &mut T {
                &mut *(self as *mut Self as *mut T)
            }
        }

        impl<T: $t $(+ $othert)*> ToBox<dyn $t $(+ $othert)*> for T {
            fn to_box(self) -> Box<dyn $t $(+ $othert)*> {
                Box::new(self)
            }
        }
    }
}

pub struct Map<A: ?Sized = dyn Any> {
    raw: RawMap<A>,
}

pub type Anything = Map<dyn Any>;

impl<A: ?Sized + CastToT> Map<A> {
    pub fn new() -> Map<A> {
        Map {
            raw: RawMap::with_hasher(Default::default()),
        }
    }

    pub fn get<T: ToBox<A>>(&self) -> Option<&T> {
        self.raw
            .get(&TypeId::of::<T>())
            .map(|any| unsafe { any.downcast_ref::<T>() })
    }

    pub fn get_mut<T: ToBox<A>>(&mut self) -> Option<&mut T> {
        self.raw
            .get_mut(&TypeId::of::<T>())
            .map(|any| unsafe { any.downcast_mut() })
    }

    pub fn insert<T: ToBox<A>>(&mut self, value: T) {
        self.raw.insert(TypeId::of::<T>(), value.to_box());
    }
}

#[macro_export]
macro_rules! add_multiple {
    ($anything:ident, $($x:expr),*) => {
        $(
            $anything.insert($x);
        )*
    };
}

#[macro_export]
macro_rules! create_anything {
    ($($x:expr),*) => {
        {
            let mut anything = Anything::new();
            $(
                anything.insert($x);
            )*
            anything
        }
    };
}

generate_implementation!(Any);
generate_implementation!(Any + Send);
generate_implementation!(Any + Send + Sync);

#[cfg(test)]
mod tests {
    use super::*;

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
    pub fn test_add_multiple() {
        let mut anything = Anything::new();
        add_multiple!(anything, 3.14f32, Foo(23));
        if let Some(val) = anything.get::<f32>() {
            assert_eq!(val, &3.14);
        }
        if let Some(val) = anything.get::<Foo>() {
            assert_eq!(val, &Foo(23));
        }
    }

    #[test]
    pub fn test_create_anything() {
        let anything = create_anything!(12i32, 3.14f32, Foo(23));
        if let Some(val) = anything.get::<i32>() {
            assert_eq!(val, &12i32);
        }
        if let Some(val) = anything.get::<f32>() {
            assert_eq!(val, &3.14);
        }
        if let Some(val) = anything.get::<Foo>() {
            assert_eq!(val, &Foo(23));
        }
    }
}
