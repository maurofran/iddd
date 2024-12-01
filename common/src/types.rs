#[macro_export]
macro_rules! constrained_string {
    ($type_name:ident,$max_length:literal) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $type_name(String);

        impl $type_name {
            const TYPE_NAME: &str = stringify!($type_name);

            pub fn new(value: &str) -> anyhow::Result<Self> {
                common::validate::not_empty(Self::TYPE_NAME, value)?;
                common::validate::max_length(Self::TYPE_NAME, value, $max_length)?;
                Ok($type_name(value.into()))
            }
        }

        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Into<String> for $type_name {
            fn into(self) -> String {
                self.0
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> anyhow::Result<Self> {
                $type_name::new(value)
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
    ($type_name:ident,$min_length:literal,$max_length:literal) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $type_name(String);

        impl $type_name {
            const TYPE_NAME: &str = stringify!($type_name);

            pub fn new(value: &str) -> anyhow::Result<Self> {
                common::validate::not_empty(Self::TYPE_NAME, value)?;
                common::validate::length_between(Self::TYPE_NAME, value, $min_length, $max_length)?;
                Ok($type_name(value.into()))
            }
        }

        impl std::fmt::Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Into<String> for $type_name {
            fn into(self) -> String {
                self.0
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> anyhow::Result<Self> {
                $type_name::new(value)
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
    ($type_name:ident,$min_length:literal,$max_length:literal,$pattern:literal) => {
        #[derive(Debug, PartialEq, Clone)]
        pub struct $type_name(String);

        impl $type_name {
            const TYPE_NAME: &str = stringify!($type_name);

            fn pattern() -> regex::Regex {
                regex::Regex::new($pattern).unwrap()
            }

            pub fn new(value: &str) -> anyhow::Result<Self> {
                common::validate::not_empty(Self::TYPE_NAME, value)?;
                common::validate::length_between(Self::TYPE_NAME, value, $min_length, $max_length)?;
                common::validate::matches(Self::TYPE_NAME, value, Self::pattern())?;
                Ok($type_name(value.into()))
            }
        }

        impl Display for $type_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Into<String> for $type_name {
            fn into(self) -> String {
                self.0
            }
        }

        impl TryFrom<&str> for $type_name {
            type Error = anyhow::Error;

            fn try_from(value: &str) -> anyhow::Result<Self> {
                $type_name::new(value)
            }
        }

        impl AsRef<str> for $type_name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}