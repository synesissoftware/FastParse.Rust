
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
        pub struct SliceIndex {

            /// The slice offset in the source sequence
            pub offset  :   usize,
            /// The length of the slice
            pub length  :   usize,
        }

        impl PartialEq for SliceIndex {

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

        impl PartialOrd for SliceIndex {

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

        impl SliceIndex {

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
        }
    }
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_empty() {

    use fastparse::types::SliceIndex;

    // check empty() produces an empty slice
    {
        let ssi = SliceIndex::empty();

        assert_eq!(0, ssi.offset);
        assert_eq!(0, ssi.length);

        assert!(ssi.is_empty());
    }
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_new() {

    use fastparse::types::SliceIndex;

    // check new(0, 0) produces an empty slice
    {
        let ssi = SliceIndex::new(0, 0);

        assert_eq!(0, ssi.offset);
        assert_eq!(0, ssi.length);

        assert!(ssi.is_empty());
    }

    // check new(1, 0) produces an empty slice
    {
        let ssi = SliceIndex::new(1, 0);

        assert_eq!(1, ssi.offset);
        assert_eq!(0, ssi.length);

        assert!(ssi.is_empty());
    }

    // check new(0, 1) produces a non-empty slice
    {
        let ssi = SliceIndex::new(0, 1);

        assert_eq!(0, ssi.offset);
        assert_eq!(1, ssi.length);

        assert!(!ssi.is_empty());
    }
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_op_eq() {

    use fastparse::types::SliceIndex;

    assert_eq!(SliceIndex::new(0, 0), SliceIndex::new(0, 0));

    assert_ne!(SliceIndex::new(0, 0), SliceIndex::new(1, 0));
    assert_ne!(SliceIndex::new(0, 0), SliceIndex::new(0, 1));
    assert_ne!(SliceIndex::new(0, 0), SliceIndex::new(1, 1));
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_op_lt() {

    use fastparse::types::SliceIndex;

    assert!(!(SliceIndex::new(0, 0) < SliceIndex::new(0, 0)));
    assert!(!(SliceIndex::new(0, 0) > SliceIndex::new(0, 0)));

    assert!(SliceIndex::new(0, 1) < SliceIndex::new(1, 1));
    assert!(SliceIndex::new(1, 1) > SliceIndex::new(0, 1));

    assert!(SliceIndex::new(0, 1) < SliceIndex::new(0, 2));
    assert!(SliceIndex::new(0, 2) > SliceIndex::new(0, 1));
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_move_unchecked() {

    use fastparse::types::SliceIndex;

    {
        let ssi1 = SliceIndex::new(0, 1);

        let ssi2 = ssi1.move_unchecked(1);

        assert_eq!(SliceIndex::new(1, 1), ssi2);
    }

    {
        let ssi1 = SliceIndex::new(1, 1);

        let ssi2 = ssi1.move_unchecked(-1);

        assert_eq!(SliceIndex::new(0, 1), ssi2);
    }

    #[cfg(not(debug_assertions))]
    {
        let ssi1 = SliceIndex::new(0, 1);

        let ssi2 = ssi1.move_unchecked(-1);

        assert_eq!(SliceIndex::new(std::usize::MAX, 1), ssi2);
    }
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_clone() {

    use fastparse::types::SliceIndex;

    let ssi1 = SliceIndex::new(10, 13);
    let ssi2 = ssi1.clone();

    assert_eq!(ssi1, ssi2);
}

#[test]
#[allow(non_snake_case)]
fn SliceIndex_copy() {

    use fastparse::types::SliceIndex;

    let ssi1 = SliceIndex::new(10, 13);
    let ssi2 = ssi1;

    assert_eq!(ssi1, ssi2);
}
