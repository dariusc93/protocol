macro_rules! impl_list_type {
    ( $ty:ident => T: $( $ty_pred:ident ),* ) => {
        impl<T> $crate::Type for ::std::collections::$ty<T>
            where T: $crate::Type $( + $ty_pred )*
        {
            fn read(read: &mut ::std::io::Read) -> Result<Self, $crate::Error> {
                let elements: Vec<T> = Vec::read(read)?;
                Ok(elements.into_iter().collect())
            }

            fn write(&self, write: &mut ::std::io::Write) -> Result<(), $crate::Error> {
                let elements: Vec<_> = self.iter().cloned().collect();
                elements.write(write)?;
                Ok(())
            }
        }

        #[cfg(test)]
        mod test
        {
            pub use Type;
            pub use std::collections::$ty;

            describe! serialization {
                it "can be written and then read without changing" {
                    let original: $ty<u32> = [1, 2, 3, 4, 5].iter().cloned().collect();

                    let raw_bytes = original.raw_bytes().unwrap();
                    let read_deque = $ty::<u32>::from_raw_bytes(&raw_bytes).unwrap();

                    assert_eq!(original, read_deque);
                }
            }
        }
    }
}

pub mod linked_list { impl_list_type!(LinkedList => T: ); }
pub mod vec_deque   { impl_list_type!(VecDeque   => T: ); }

pub mod hash_set {
    use std::hash::Hash;
    impl_list_type!(HashSet => T: Hash, Eq);
}

