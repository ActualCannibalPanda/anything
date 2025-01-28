use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub type RawMap<A> = HashMap<TypeId, Box<A>>;

pub trait ToBox<T: ?Sized + Cast>: Any {
    fn to_box(self) -> Box<T>;
}

pub trait Cast {
    unsafe fn cast_ref<T>(&self) -> &T;
    unsafe fn cast_mut<T>(&mut self) -> &mut T;
}

pub struct Map<A: ?Sized = dyn Any> {
    raw: RawMap<A>,
}

pub type Anything = Map<dyn Any>;

impl<A: ?Sized + Cast> Default for Map<A> {
    fn default() -> Self {
        Self {
            raw: RawMap::with_hasher(Default::default()),
        }
    }
}

impl<A: ?Sized + Cast> Map<A> {
    pub fn new() -> Map<A> {
        Map {
            raw: RawMap::with_hasher(Default::default()),
        }
    }

    pub fn get<T: ToBox<A>>(&self) -> Option<&T> {
        self.raw
            .get(&TypeId::of::<T>())
            .map(|any| unsafe { any.cast_ref::<T>() })
    }

    pub fn get_mut<T: ToBox<A>>(&mut self) -> Option<&mut T> {
        self.raw
            .get_mut(&TypeId::of::<T>())
            .map(|any| unsafe { any.cast_mut() })
    }

    pub fn insert<T: ToBox<A>>(&mut self, value: T) {
        self.raw.insert(TypeId::of::<T>(), value.to_box());
    }
}

macro_rules! generate_implementation {
    ($t:ident $(+ $othert:ident)*) => {
        impl Cast for dyn $t $(+ $othert)* {
            unsafe fn cast_ref<T>(&self) -> &T {
                &*(self as *const Self as *const T)
            }

            unsafe fn cast_mut<T>(&mut self) -> &mut T {
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

generate_implementation!(Any);
generate_implementation!(Any + Send);
generate_implementation!(Any + Send + Sync);
