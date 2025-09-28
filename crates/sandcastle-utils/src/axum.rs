#[macro_export]
macro_rules! declare_header {
    // String type (default)
    ( $name:expr => $struct:ident ) => {
        declare_header!($name => $struct: String);
    };
    
    // Typed version
    ( $name:expr => $struct:ident : $type:ty ) => {
        pub struct $struct(pub $type);

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
}