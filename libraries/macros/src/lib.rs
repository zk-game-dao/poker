#[macro_export]
macro_rules! impl_principal_traits {
    ($type:ty) => {
        impl PartialEq<Principal> for $type {
            fn eq(&self, other: &Principal) -> bool {
                self.0 == *other
            }
        }

        impl PartialEq<$type> for Principal {
            fn eq(&self, other: &$type) -> bool {
                *self == other.0
            }
        }

        impl Default for $type {
            fn default() -> Self {
                Self(Principal::anonymous()) // Use Self() instead of $type()
            }
        }
    };
}

#[macro_export]
macro_rules! impl_u64_comparisons {
    ($type:ty) => {
        impl PartialOrd<u64> for $type {
            fn partial_cmp(&self, other: &u64) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(other)
            }
        }

        impl PartialEq<u64> for $type {
            fn eq(&self, other: &u64) -> bool {
                self.0 == *other
            }
        }

        impl PartialOrd<$type> for u64 {
            fn partial_cmp(&self, other: &$type) -> Option<std::cmp::Ordering> {
                self.partial_cmp(&other.0)
            }
        }

        impl PartialEq<$type> for u64 {
            fn eq(&self, other: &$type) -> bool {
                *self == other.0
            }
        }
    };
}
