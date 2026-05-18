macro_rules! raw_option_set {
    ($name:ident) => {
        /// Wraps a raw `UserNotifications` option-set bitmask.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct $name(u64);

        impl $name {
            /// Creates the option set from raw framework bits.
            #[must_use]
            pub const fn from_bits(bits: u64) -> Self {
                Self(bits)
            }

            /// Returns the raw framework bits for this option set.
            #[must_use]
            pub const fn bits(self) -> u64 {
                self.0
            }

            /// Returns whether all bits in `other` are present in this option set.
            #[must_use]
            pub const fn contains(self, other: Self) -> bool {
                (self.0 & other.0) == other.0
            }
        }

        impl ::std::ops::BitOr for $name {
            type Output = Self;

            fn bitor(self, rhs: Self) -> Self::Output {
                Self(self.0 | rhs.0)
            }
        }

        impl ::std::ops::BitOrAssign for $name {
            fn bitor_assign(&mut self, rhs: Self) {
                self.0 |= rhs.0;
            }
        }
    };
}

pub(crate) use raw_option_set;
