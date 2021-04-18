
/// Fast-parsing constructs and operations
pub mod fastparse {

    /// Fast-parsing types
    pub mod types {

        use std::cmp::{

            Eq,
            Ordering,
            PartialEq,
            PartialOrd,
        };

        /// A slice representation of offset and length
        #[derive(Clone)]
        #[derive(Copy)]
        #[derive(Debug)]
        #[derive(Eq)]
        pub struct PositionalSlice {

            /// The slice offset in the source sequence
            pub offset  :   usize,
            /// The length of the slice
            pub length  :   usize,
        }

        impl PartialEq for PositionalSlice {

            fn eq(&self, other: &Self) -> bool {

                if self.offset != other.offset {

                    return false
                }

                if self.length != other.length {

                    return false
                }

                true
            }
        }

        impl PartialOrd for PositionalSlice {

            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

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

        impl PositionalSlice {

            /// Creates an empty instance
            pub fn empty() -> Self {

                Self {

                    offset  :   0,
                    length  :   0,
                }
            }

            /// Creates an instance with the given `off`set and `len`gth
            ///
            /// Parameters:
            /// * `off` - The offset of the slice
            /// * `len` - The length of the slice
            pub fn new(off : usize, len : usize) -> Self {

                Self {

                    offset  :   off,
                    length  :   len,
                }
            }

            /// Indicates the length of the slice
            pub fn len(&self) -> usize {

                self.length
            }

            /// Indicates whether the slice is empty
            pub fn is_empty(&self) -> bool {

                0 == self.length
            }

            /// Obtains unchecked a copy of the slice moved by the given
            /// `d`elta
            ///
            /// Parameters:
            /// * `d` - The delta
            ///
            /// Return:
            /// New instance of `PositionalSlice` adjusted appropriately
            ///
            /// Preconditions:
            /// * `isize <= self.offset` - will panic (in debug) if false
            pub fn move_unchecked(&self, d : isize) -> Self {

                // TODO: determine the rigth Rust way of doing addition with
                let new_off : usize = if d < 0 {

                    self.offset - (-d) as usize
                } else {

                    self.offset + d as usize
                };

                Self {

                    length  :   self.length,
                    offset  :   new_off,
                }
            }

            /// Obtains checked a copy of the slice moved by the given
            /// `d`elta
            ///
            /// Parameters:
            /// * `d` - The delta
            ///
            /// Return:
            /// `Option<PositionalSlice>`, where, if `Some`, it contains appropriately adjusted slice
            pub fn move_checked(&self, d : isize) -> Option<Self> {

                // Possibilities for failure:
                //
                // 1. d is -ve and _would_ move offset below 0;
                // 2. d is +ve and _would_ move offset above usize::MAX; or
                // 3. d is +ve and _would_ move offset such that offset+length > usize::MAX

                if 0 == d {

                    return Some(self.clone());
                }

                if d < 0 {

                    let a = (-d) as usize;

                    if a > self.offset {

                        // case 1.
                        return None;
                    }

                    return Some(Self::new(self.offset - a, self.length));
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

                    return Some(Self::new(self.offset + a, self.length));
                }
            }

            /// Applies this positional slice to a slice of arbitrary type, obtaining a relative slice as a result
            pub fn subslice_of<'a, T>(&self, slice : &'a [T]) -> &'a [T] {

                &slice[self.offset..self.offset+self.length]
            }

            /// Applies this positional slice to a slice of arbitrary type, obtaining a relative slice as a result
            pub fn substring_of<'a>(&self, slice : &'a str) -> &'a str {

                &slice[self.offset..self.offset+self.length]
            }
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_empty() {

    use fastparse::types::PositionalSlice;

    // check empty() produces an empty slice
    {
        let ssi = PositionalSlice::empty();

        assert_eq!(0, ssi.offset);
        assert_eq!(0, ssi.length);

        assert!(ssi.is_empty());
    }
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_new() {

    use fastparse::types::PositionalSlice;

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
#[allow(non_snake_case)]
fn PositionalSlice_op_eq() {

    use fastparse::types::PositionalSlice;

    assert_eq!(PositionalSlice::new(0, 0), PositionalSlice::new(0, 0));

    assert_ne!(PositionalSlice::new(0, 0), PositionalSlice::new(1, 0));
    assert_ne!(PositionalSlice::new(0, 0), PositionalSlice::new(0, 1));
    assert_ne!(PositionalSlice::new(0, 0), PositionalSlice::new(1, 1));
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_op_lt() {

    use fastparse::types::PositionalSlice;

    assert!(!(PositionalSlice::new(0, 0) < PositionalSlice::new(0, 0)));
    assert!(!(PositionalSlice::new(0, 0) > PositionalSlice::new(0, 0)));

    assert!(PositionalSlice::new(0, 1) < PositionalSlice::new(1, 1));
    assert!(PositionalSlice::new(1, 1) > PositionalSlice::new(0, 1));

    assert!(PositionalSlice::new(0, 1) < PositionalSlice::new(0, 2));
    assert!(PositionalSlice::new(0, 2) > PositionalSlice::new(0, 1));
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_move_unchecked() {

    use fastparse::types::PositionalSlice;

    {
        let ssi1 = PositionalSlice::new(0, 1);

        let ssi2 = ssi1.move_unchecked(1);

        assert_eq!(PositionalSlice::new(1, 1), ssi2);
    }

    {
        let ssi1 = PositionalSlice::new(1, 1);

        let ssi2 = ssi1.move_unchecked(-1);

        assert_eq!(PositionalSlice::new(0, 1), ssi2);
    }

    #[cfg(not(debug_assertions))]
    {
        let ssi1 = PositionalSlice::new(0, 1);

        let ssi2 = ssi1.move_unchecked(-1);

        assert_eq!(PositionalSlice::new(std::usize::MAX, 1), ssi2);
    }
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_move_checked() {

    use fastparse::types::PositionalSlice;

    {
        let ssi1 = PositionalSlice::new(0, 1);

        let ssi2 = ssi1.move_checked(1);

        assert!(ssi2.is_some());
        assert_eq!(PositionalSlice::new(1, 1), ssi2.unwrap());
    }

    {
        let ssi1 = PositionalSlice::new(1, 1);

        let ssi2 = ssi1.move_checked(-1);

        assert!(ssi2.is_some());
        assert_eq!(PositionalSlice::new(0, 1), ssi2.unwrap());
    }

    {
        let ssi1 = PositionalSlice::new(0, 1);

        let ssi2 = ssi1.move_checked(-1);

        assert!(ssi2.is_none());
    }

    {
        let ssi1 = PositionalSlice::new(usize::MAX - 2, 1);

        let ssi2 = ssi1.move_checked(1);

        assert!(ssi2.is_some());
        assert_eq!(PositionalSlice::new(usize::MAX - 1, 1), ssi2.unwrap());
    }

    {
        let ssi1 = PositionalSlice::new(usize::MAX - 2, 1);

        let ssi2 = ssi1.move_checked(2);

        assert!(ssi2.is_none());
    }
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_clone() {

    use fastparse::types::PositionalSlice;

    let ssi1 = PositionalSlice::new(10, 13);
    let ssi2 = ssi1.clone();

    assert_eq!(ssi1, ssi2);
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_copy() {

    use fastparse::types::PositionalSlice;

    let ssi1 = PositionalSlice::new(10, 13);
    let ssi2 = ssi1;

    assert_eq!(ssi1, ssi2);
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_subslice_of() {

    use fastparse::types::PositionalSlice;

    {
        let ps = PositionalSlice::new(2, 2);

        let source = vec![ 0, 1, 2, 3, 4, 5, 6 ];

        let sub = ps.subslice_of(&source);

        assert_eq!(2, sub.len());
        assert_eq!(2, sub[0]);
        assert_eq!(3, sub[1]);
    }

    {
        let ps = PositionalSlice::new(2, 2);

        let source = vec![ 0, 1, 2, 3, 4, 5, 6 ];

        let sub = ps.subslice_of(&source[1..]);

        assert_eq!(2, sub.len());
        assert_eq!(3, sub[0]);
        assert_eq!(4, sub[1]);
    }
}

#[test]
#[allow(non_snake_case)]
fn PositionalSlice_substring_of() {

    use fastparse::types::PositionalSlice;

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
