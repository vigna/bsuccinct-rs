#![doc = include_str!("../README.md")]

/// Provides methods to get dynamic and total size of the variable.
pub trait GetSize {
    /// Returns approximate number of bytes occupied by dynamic part of `self`.
    /// Same as `self.size_bytes() - std::mem::size_of_val(self)`.
    fn size_bytes_dyn(&self) -> usize { 0 }

    /// Returns approximate, total (including heap memory) number of bytes occupied by `self`.
    fn size_bytes(&self) -> usize {
        std::mem::size_of_val(self) + self.size_bytes_dyn()
    }

    /// `true` if and only if the variables of this type can use dynamic (heap) memory.
    const USES_DYN_MEM: bool = false;
}

macro_rules! impl_nodyn_getsize_for {
    ($x:ty) => (impl GetSize for $x {});
    // `$x` followed by at least one `$y,`
    ($x:ty, $($y:ty),+) => (
        impl GetSize for $x {}
        impl_nodyn_getsize_for!($($y),+);
    )
}

impl_nodyn_getsize_for!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, char, ());

//impl<T: GetSize> GetSize for [T] {    // this works also with slices, but is this sound?
impl<T: GetSize, const N: usize> GetSize for [T; N] {
    fn size_bytes_dyn(&self) -> usize {
        if T::USES_DYN_MEM {
            self.iter().map(|i| i.size_bytes_dyn()).sum()
        } else {
            0
        }
    }
    //fn can_use_dyn_mem() -> bool { T::can_use_dyn_mem() }
    const USES_DYN_MEM: bool = T::USES_DYN_MEM;
}

macro_rules! impl_dyn_getsize_methods {
    ($T:ty) => (
        fn size_bytes_dyn(&self) -> usize {
            if <$T>::USES_DYN_MEM {
                self.iter().map(|i| i.size_bytes()).sum()
            } else {
                std::mem::size_of::<$T>() * self.len()
            }
        }
        const USES_DYN_MEM: bool = true;
    );
}

impl<T: GetSize> GetSize for Vec<T> {
    impl_dyn_getsize_methods!(T);
}

impl<T: GetSize> GetSize for Box<[T]> {
    impl_dyn_getsize_methods!(T);
}


#[cfg(test)]
mod tests {
    use super::*;

    fn test_primitive<T: GetSize>(v: T) {
        assert_eq!(v.size_bytes_dyn(), 0);
        assert_eq!(v.size_bytes(), std::mem::size_of_val(&v));
        assert!(!T::USES_DYN_MEM);
    }

    #[test]
    fn test_primitives() {
        test_primitive(1u32);
        test_primitive(1.0f32);
    }

    #[test]
    fn test_array() {
        assert_eq!([1u32, 2u32, 3u32].size_bytes(), 3*4);
        assert_eq!([[1u32, 2u32], [3u32, 4u32]].size_bytes(), 4*4);
        assert_eq!([vec![1u32, 2u32], vec![3u32, 4u32]].size_bytes_dyn(), 4*4);
    }

    #[test]
    fn test_vec() {
        assert_eq!(vec![1u32, 2u32, 3u32].size_bytes_dyn(), 3*4);
        assert_eq!(vec![[1u32, 2u32], [3u32, 4u32]].size_bytes_dyn(), 4*4);
        let v = vec![1u32, 2u32];
        assert_eq!(vec![v.clone(), v.clone()].size_bytes_dyn(), 2*v.size_bytes());
    }

    #[test]
    fn test_boxed_slice() {
        let bs = vec![1u32, 2u32, 3u32].into_boxed_slice();
        assert_eq!(bs.size_bytes_dyn(), 3*4);
        assert_eq!(bs.size_bytes(), 3*4 + std::mem::size_of_val(&bs));
    }
}