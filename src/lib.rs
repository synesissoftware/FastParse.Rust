/// Fast-parsing constructs and operations
pub mod fastparse {

    /// Fast-parsing types.
    pub mod types {

        use std::cmp::{
            self as std_cmp,
            Ordering,
        };


        /// A slice representation of offset and length.
        #[derive(Clone, Copy)]
        #[derive(Debug)]
        #[derive(Eq)]
        pub struct PositionalSlice {
            /// The slice offset in the source sequence.
            pub offset : usize,
            /// The length of the slice.
            pub length : usize,
        }

        // API functions
        impl PositionalSlice {
            /// Creates an empty instance.
            pub fn empty() -> Self {
                let offset = 0;
                let length = 0;

                Self {
                    offset,
                    length,
                }
            }

            /// Creates an instance with the given `off`set and `len`gth.
            ///
            /// Parameters:
            /// - `off` - The offset of the slice;
            /// - `len` - The length of the slice;
            pub fn new(
                off : usize,
                len : usize,
            ) -> Self {
                Self {
                    offset : off,
                    length : len,
                }
            }
        }

        // Mutating methods
        impl PositionalSlice {
        }

        // Non-mutating methods
        impl PositionalSlice {
            /// Indicates whether the slice is empty.
            pub fn is_empty(&self) -> bool {
                0 == self.length
            }

            /// Indicates the length of the slice.
            pub fn len(&self) -> usize {
                self.length
            }

            /// Obtains unchecked a copy of the slice moved by the given
            /// `d`elta.
            ///
            /// # Parameters:
            /// - `d` - The delta;
            ///
            /// # Return:
            /// New instance of [`PositionalSlice`] adjusted appropriately.
            ///
            /// # Preconditions:
            /// * `isize <= self.offset` - will panic (in debug) if false
            pub fn offset_unchecked(
                &self,
                d : isize,
            ) -> Self {
                // TODO: determine the right Rust way of doing addition with
                let new_off : usize = if d < 0 {
                    self.offset - (-d) as usize
                } else {
                    self.offset + d as usize
                };

                Self {
                    length : self.length,
                    offset : new_off,
                }
            }

            /// Obtains checked a copy of the slice moved by the given
            /// `d`elta.
            ///
            /// # Parameters:
            /// - `d` - The delta;
            ///
            /// # Return:
            /// `Option<PositionalSlice>`, where, if `Some`, it contains
            /// appropriately adjusted slice.
            pub fn offset_checked(
                &self,
                d : isize,
            ) -> Option<Self> {
                // Possibilities for failure:
                //
                // 1. d is -ve and _would_ move offset below 0;
                // 2. d is +ve and _would_ move offset above usize::MAX; or
                // 3. d is +ve and _would_ move offset such that offset+length > usize::MAX

                if 0 == d {
                    return Some(*self);
                }

                if d < 0 {
                    let a = (-d) as usize;

                    if a > self.offset {
                        // case 1.
                        return None;
                    }

                    Some(Self::new(self.offset - a, self.length))
                } else {
                    debug_assert!(d > 0);

                    let a = d as usize;

                    if a > usize::MAX - self.offset {
                        // case 2.
                        return None;
                    }

                    if a + self.length > usize::MAX - self.offset {
                        // case 3.
                        return None;
                    }

                    Some(Self::new(self.offset + a, self.length))
                }
            }

            /// Applies this positional slice to a slice of arbitrary type,
            /// obtaining a relative slice as a result.
            ///
            /// # Parameters:
            /// - `slice` - The slice of which to provide a subslice;
            ///
            /// # Return:
            /// An instance of a slice of `slice` according to the `offset`
            /// and `length` of the receiving instance.
            pub fn subslice_of<'a, T>(
                &self,
                slice : &'a [T],
            ) -> &'a [T] {
                &slice[self.offset..self.offset + self.length]
            }

            /// Applies this positional slice to a slice of `'str`,
            /// obtaining a relative slice as a result.
            ///
            /// # Parameters:
            /// - `slice` - The slice of which to provide a subslice;
            ///
            /// # Return:
            /// An instance of a slice of `slice` according to the `offset`
            /// and `length` of the receiving instance.
            pub fn substring_of<'a>(
                &self,
                slice : &'a str,
            ) -> &'a str {
                &slice[self.offset..self.offset + self.length]
            }
        }

        // Trait implementations

        impl std_cmp::PartialEq for PositionalSlice {
            fn eq(
                &self,
                other : &Self,
            ) -> bool {
                if self.offset != other.offset {
                    return false;
                }

                if self.length != other.length {
                    return false;
                }

                true
            }
        }

        impl std_cmp::PartialOrd for PositionalSlice {
            fn partial_cmp(
                &self,
                other : &Self,
            ) -> Option<Ordering> {
                if self.offset < other.offset {
                    return Some(Ordering::Less);
                }

                if other.offset < self.offset {
                    return Some(Ordering::Greater);
                }

                if self.length < other.length {
                    return Some(Ordering::Less);
                }

                if other.length < self.length {
                    return Some(Ordering::Greater);
                }

                Some(Ordering::Equal)
            }
        }
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::fastparse::types::PositionalSlice;


    #[test]
    fn PositionalSlice_empty() {
        // check empty() produces an empty slice
        {
            let ssi = PositionalSlice::empty();

            assert_eq!(0, ssi.offset);
            assert_eq!(0, ssi.length);

            assert!(ssi.is_empty());
        }
    }

    #[test]
    fn PositionalSlice_new() {
        // check new(0, 0) produces an empty slice
        {
            let ssi = PositionalSlice::new(0, 0);

            assert_eq!(0, ssi.offset);
            assert_eq!(0, ssi.length);

            assert!(ssi.is_empty());
        }

        // check new(1, 0) produces an empty slice
        {
            let ssi = PositionalSlice::new(1, 0);

            assert_eq!(1, ssi.offset);
            assert_eq!(0, ssi.length);

            assert!(ssi.is_empty());
        }

        // check new(0, 1) produces a non-empty slice
        {
            let ssi = PositionalSlice::new(0, 1);

            assert_eq!(0, ssi.offset);
            assert_eq!(1, ssi.length);

            assert!(!ssi.is_empty());
        }
    }

    #[test]
    fn PositionalSlice_clone() {
        let ssi1 = PositionalSlice::new(10, 13);
        let ssi2 = ssi1.clone();

        assert_eq!(ssi1, ssi2);
    }

    #[test]
    fn PositionalSlice_copy() {
        let ssi1 = PositionalSlice::new(10, 13);
        let ssi2 = ssi1;

        assert_eq!(ssi1, ssi2);
    }

    #[test]
    fn PositionalSlice_op_eq() {
        assert_eq!(PositionalSlice::new(0, 0), PositionalSlice::new(0, 0));

        assert_ne!(PositionalSlice::new(0, 0), PositionalSlice::new(1, 0));
        assert_ne!(PositionalSlice::new(0, 0), PositionalSlice::new(0, 1));
        assert_ne!(PositionalSlice::new(0, 0), PositionalSlice::new(1, 1));
    }

    #[test]
    fn PositionalSlice_op_lt() {
        assert!(!(PositionalSlice::new(0, 0) < PositionalSlice::new(0, 0)));
        assert!(!(PositionalSlice::new(0, 0) > PositionalSlice::new(0, 0)));

        assert!(PositionalSlice::new(0, 1) < PositionalSlice::new(1, 1));
        assert!(PositionalSlice::new(1, 1) > PositionalSlice::new(0, 1));

        assert!(PositionalSlice::new(0, 1) < PositionalSlice::new(0, 2));
        assert!(PositionalSlice::new(0, 2) > PositionalSlice::new(0, 1));
    }

    #[test]
    fn PositionalSlice_offset_unchecked() {
        {
            let ssi1 = PositionalSlice::new(0, 1);

            let ssi2 = ssi1.offset_unchecked(1);

            assert_eq!(PositionalSlice::new(1, 1), ssi2);
        }

        {
            let ssi1 = PositionalSlice::new(1, 1);

            let ssi2 = ssi1.offset_unchecked(-1);

            assert_eq!(PositionalSlice::new(0, 1), ssi2);
        }

        #[cfg(not(debug_assertions))]
        {
            let ssi1 = PositionalSlice::new(0, 1);

            let ssi2 = ssi1.offset_unchecked(-1);

            assert_eq!(PositionalSlice::new(std::usize::MAX, 1), ssi2);
        }
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn PositionalSlice_offset_unchecked_() {
        {
            let ssi1 = PositionalSlice::new(0, 1);

            let _ssi2 = ssi1.offset_unchecked(-1);

            panic!("should not get here");
        }
    }

    #[test]
    fn PositionalSlice_offset_checked() {
        {
            let ssi1 = PositionalSlice::new(0, 1);

            let ssi2 = ssi1.offset_checked(1);

            assert!(ssi2.is_some());
            assert_eq!(PositionalSlice::new(1, 1), ssi2.unwrap());
        }

        {
            let ssi1 = PositionalSlice::new(1, 1);

            let ssi2 = ssi1.offset_checked(-1);

            assert!(ssi2.is_some());
            assert_eq!(PositionalSlice::new(0, 1), ssi2.unwrap());
        }

        {
            let ssi1 = PositionalSlice::new(0, 1);

            let ssi2 = ssi1.offset_checked(-1);

            assert!(ssi2.is_none());
        }

        {
            let ssi1 = PositionalSlice::new(usize::MAX - 2, 1);

            let ssi2 = ssi1.offset_checked(1);

            assert!(ssi2.is_some());
            assert_eq!(PositionalSlice::new(usize::MAX - 1, 1), ssi2.unwrap());
        }

        {
            let ssi1 = PositionalSlice::new(usize::MAX - 2, 1);

            let ssi2 = ssi1.offset_checked(2);

            assert!(ssi2.is_none());
        }
    }

    #[test]
    fn PositionalSlice_subslice_of() {
        {
            let ps = PositionalSlice::new(2, 2);

            let source = vec![
                // insert list:
                0, 1, 2, 3, 4, 5, 6,
            ];

            let sub = ps.subslice_of(&source);

            assert_eq!(2, sub.len());
            assert_eq!(2, sub[0]);
            assert_eq!(3, sub[1]);
        }

        {
            let ps = PositionalSlice::new(2, 2);

            let source = vec![
                // insert list:
                0, 1, 2, 3, 4, 5, 6,
            ];

            let sub = ps.subslice_of(&source[1..]);

            assert_eq!(2, sub.len());
            assert_eq!(3, sub[0]);
            assert_eq!(4, sub[1]);
        }
    }

    #[test]
    fn PositionalSlice_substring_of() {
        {
            let ps = PositionalSlice::new(2, 2);

            let source = "abcdef".to_string();

            let sub = ps.substring_of(&source[..]);

            assert_eq!("cd", sub);
        }

        {
            let ps = PositionalSlice::new(2, 2);

            let source = "abcdef".to_string();

            let sub = ps.substring_of(&source[1..]);

            assert_eq!("de", sub);
        }
    }
}
