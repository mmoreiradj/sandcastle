#[macro_export]
macro_rules! declare_header {
    // String type (default)
    ( $name:expr => $struct:ident ) => {
        declare_header!($name => $struct: String);
    };
    
    // Typed version
    ( $name:expr => $struct:ident : $type:ty ) => {
        pub struct $struct(pub $type);

        impl $struct {
            pub fn into_inner(self) -> $type {
                self.0
            }
        }

        impl Header for $struct {
            fn name() -> &'static HeaderName {
                static NAME: HeaderName = HeaderName::from_static($name);
                &NAME
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
            where
                Self: Sized,
                I: Iterator<Item = &'i HeaderValue>,
            {
                let value = values.next().ok_or_else(headers::Error::invalid)?;
                let str_value = value
                    .to_str()
                    .map_err(|_| headers::Error::invalid())?;
                
                str_value
                    .parse::<$type>()
                    .map(Self)
                    .map_err(|_| headers::Error::invalid())
            }

            fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                let value_str = self.0.to_string();
                values.extend(std::iter::once(
                    HeaderValue::from_str(&value_str).expect("invalid header value"),
                ));
            }
        }
    };
    
    // Serde-based version for types that implement Serialize + Deserialize but not FromStr + Display
    ( $name:expr => $struct:ident : serde $type:ty ) => {
        pub struct $struct(pub $type);

        impl $struct {
            pub fn into_inner(self) -> $type {
                self.0
            }
        }

        impl Header for $struct {
            fn name() -> &'static HeaderName {
                static NAME: HeaderName = HeaderName::from_static($name);
                &NAME
            }

            fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
            where
                Self: Sized,
                I: Iterator<Item = &'i HeaderValue>,
            {
                let value = values.next().ok_or_else(headers::Error::invalid)?;
                let str_value = value
                    .to_str()
                    .map_err(|_| headers::Error::invalid())?;
                
                // Try to deserialize from JSON string first, then fall back to plain string
                serde_json::from_str::<$type>(str_value)
                    .or_else(|_| serde_json::from_str::<$type>(&format!("\"{}\"", str_value)))
                    .map(Self)
                    .map_err(|_| headers::Error::invalid())
            }

            fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                // Serialize to JSON and remove quotes if it's a simple string
                let json_str = serde_json::to_string(&self.0).unwrap_or_default();
                let value_str = if json_str.starts_with('"') && json_str.ends_with('"') {
                    json_str[1..json_str.len()-1].to_string()
                } else {
                    json_str
                };
                
                values.extend(std::iter::once(
                    HeaderValue::from_str(&value_str).expect("invalid header value"),
                ));
            }
        }
    };
}