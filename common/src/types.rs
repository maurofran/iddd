#[macro_export]
macro_rules! declare_simple_type {
    ($type_name:ident,$max_length:literal) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            derive_more::Deref,
            derive_more::AsRef,
        )]
        pub struct $type_name(String);

        impl $type_name {
            const TYPE_NAME: &str = stringify!($type_name);

            pub fn new(value: &str) -> anyhow::Result<Self> {
                common::validate::not_empty(Self::TYPE_NAME, value)?;
                common::validate::max_length(Self::TYPE_NAME, value, $max_length)?;
                Ok(Self(value.into()))
            }

            pub unsafe fn new_unchecked(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> anyhow::Result<Self> {
                Self::new(value)
            }
        }

        impl TryFrom<String> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: String) -> anyhow::Result<Self> {
                Self::new(&value)
            }
        }

        impl Into<String> for &$type_name {
            fn into(self) -> String {
                self.0.clone()
            }
        }
    };
    ($type_name:ident,$max_length:literal,$pattern:literal) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            derive_more::Display,
            derive_more::Deref,
            derive_more::AsRef,
            derive_more::Into,
        )]
        pub struct $type_name(String);

        impl $type_name {
            const TYPE_NAME: &str = stringify!($type_name);

            pub fn new(value: &str) -> anyhow::Result<Self> {
                static PATTERN: std::sync::LazyLock<regex::Regex> =
                    std::sync::LazyLock::new(|| regex::Regex::new($pattern).unwrap());

                common::validate::not_empty(Self::TYPE_NAME, value)?;
                common::validate::max_length(Self::TYPE_NAME, value, $max_length)?;
                common::validate::matches(Self::TYPE_NAME, value, &*PATTERN)?;
                Ok(Self(value.into()))
            }

            pub unsafe fn new_unchecked(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> anyhow::Result<Self> {
                Self::new(value)
            }
        }
    };
    ($type_name:ident,$min_length:literal,$max_length:literal,$pattern:literal) => {
        #[derive(
            Debug,
            Clone,
            PartialEq,
            Eq,
            PartialOrd,
            Ord,
            Hash,
            derive_more::Display,
            derive_more::Deref,
            derive_more::AsRef,
            derive_more::Into,
        )]
        pub struct $type_name(String);

        impl $type_name {
            const TYPE_NAME: &str = stringify!($type_name);

            pub fn new(value: &str) -> anyhow::Result<Self> {
                static PATTERN: std::sync::LazyLock<regex::Regex> =
                    std::sync::LazyLock::new(|| regex::Regex::new($pattern).unwrap());

                common::validate::not_empty(Self::TYPE_NAME, value)?;
                common::validate::length_between(Self::TYPE_NAME, value, $min_length, $max_length)?;
                common::validate::matches(Self::TYPE_NAME, value, &*PATTERN)?;
                Ok(Self(value.into()))
            }

            pub unsafe fn new_unchecked(value: &str) -> Self {
                Self(value.into())
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> anyhow::Result<Self> {
                Self::new(value)
            }
        }
    };
}
