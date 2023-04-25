#[cfg(test)]
mod tests;

use std::{
    mem::size_of,
    ops::{Index, IndexMut},
};

use std::fmt::{Debug, Formatter};

pub struct Vector<T: ?Sized + Default> {
    buffer: *mut T,
    len: usize,
    cap: usize,
}

const DEFAULT_CAPACITY: usize = 10;

impl<T: ?Sized + Default> Vector<T> {
    pub fn new() -> Vector<T> {
        let v = Vector {
            buffer: &mut T::default(),
            len: 0,
            cap: DEFAULT_CAPACITY,
        };

        for i in 0..v.cap {
            unsafe {
                v.buffer.add(i * size_of::<T>()).write(T::default());
            }
        }

        v
    }

    pub fn new_with_length(len: usize) -> Vector<T> {
        let v = Vector {
            buffer: &mut T::default(),
            len: len,
            cap: len * 2,
        };

        for i in 0..v.cap {
            unsafe {
                v.buffer.add(i * size_of::<T>()).write(T::default());
            }
        }

        v
    }

    pub fn new_with_capacity(cap: usize) -> Vector<T> {
        let v = Vector {
            buffer: &mut T::default(),
            len: 0,
            cap: cap,
        };

        for i in 0..v.cap {
            unsafe {
                v.buffer.add(i * size_of::<T>()).write(T::default());
            }
        }

        v
    }

    pub fn push(&mut self, val: T) {
        let ind = self.len;
        self.len += 1;

        if self.len < self.cap {
            unsafe {
                self.buffer.add(ind * size_of::<T>()).write(val);
            }
            return;
        }

        self.resize(self.len);
        unsafe {
            self.buffer.add(ind * size_of::<T>()).write(val);
        }
    }

    pub fn resize(&mut self, new_size: usize) {
        for i in self.cap..new_size {
            unsafe {
                self.buffer.add(i * size_of::<T>()).write(T::default());
            }
        }

        self.len = std::cmp::min(self.len, new_size);
        self.cap = new_size;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn cap(&self) -> usize {
        self.cap
    }
}

impl<T: Default> Index<usize> for Vector<T> {
    type Output = T;
    fn index(&self, ind: usize) -> &Self::Output {
        if ind < self.len {
            unsafe {
                return &*self.buffer.add(ind * size_of::<T>());
            }
        }

        panic!(
            "Index i: {} is outside the bounds of vector l: {}",
            ind, self.len,
        );
    }
}

impl<T: Default> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, ind: usize) -> &mut T {
        if ind < self.len {
            unsafe {
                return &mut *self.buffer.add(ind * size_of::<T>());
            }
        }

        panic!(
            "Index i: {} is outside the bounds of vector l: {}",
            ind, self.len,
        );
    }
}

impl<T: Debug + Default> ToString for Vector<T> {
    fn to_string(&self) -> String {
        let mut s = "[".to_string();
        for i in 0..self.len() - 1 {
            s += &format!("{:?}, ", self[i]);
        }

        s = s + &format!("{:?}", self[self.len() - 1]) + "]";

        s
    }
}

impl<T: Debug + Default> Debug for Vector<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vector")
            .field("Length:", &self.len())
            .field("Capacity:", &self.len())
            .field("Elements:", &self.to_string())
            .finish()
    }
}

#[macro_export]
macro_rules! vector {
    () => ({
        Vector::new()
    });

    ($t:ty) => ({
        Vector::<$t>::new()
    });

    ($elem:expr; $num:expr) => ({
        let mut vector = Vector::new();
        for _ in 0..num {
            vector.push($elem);
        }
        vector
    });

    ($($elem:expr),+ $(,)?) => ({
        let mut vector = Vector::new();
        $(
            vector.push($elem);
        )*
        vector
    });
}
