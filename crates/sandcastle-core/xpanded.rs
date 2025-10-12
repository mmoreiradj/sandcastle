#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
use crate::error::SandcastleError;
pub mod application {
    use kube::{Client, Config};
    use crate::domain::repositories::services::{
        DefaultRepositoryConfigurationService, RepositoryConfigurations,
    };
    mod http {
        use axum_tracing_opentelemetry::middleware::{OtelAxumLayer, OtelInResponseLayer};
        pub mod webhook {
            use axum::Router;
            use axum_extra::routing::RouterExt;
            mod github {
                pub mod handler {
                    use axum::extract::Json;
                    use axum::http::{HeaderName, HeaderValue};
                    use axum_extra::{
                        TypedHeader, headers::{self, Header},
                        routing::TypedPath,
                    };
                    use axum_macros::FromRequestParts;
                    use octocrab::models::webhook_events::{
                        WebhookEvent, WebhookEventPayload, WebhookEventType,
                    };
                    use sandcastle_utils::declare_header;
                    use serde::Deserialize;
                    use serde_json::Value;
                    use tracing::info;
                    #[typed_path("/api/v1/github/webhook")]
                    pub struct HandleWebhookRoute;
                    #[automatically_derived]
                    impl ::axum_extra::routing::TypedPath for HandleWebhookRoute {
                        const PATH: &'static str = "/api/v1/github/webhook";
                    }
                    #[automatically_derived]
                    impl ::std::fmt::Display for HandleWebhookRoute {
                        fn fmt(
                            &self,
                            f: &mut ::std::fmt::Formatter<'_>,
                        ) -> ::std::fmt::Result {
                            f.write_fmt(format_args!("/api/v1/github/webhook"))
                        }
                    }
                    #[automatically_derived]
                    impl<S> ::axum::extract::FromRequestParts<S> for HandleWebhookRoute
                    where
                        S: Send + Sync,
                    {
                        type Rejection = ::axum::http::StatusCode;
                        async fn from_request_parts(
                            parts: &mut ::axum::http::request::Parts,
                            _state: &S,
                        ) -> ::std::result::Result<Self, Self::Rejection> {
                            if parts.uri.path()
                                == <Self as ::axum_extra::routing::TypedPath>::PATH
                            {
                                Ok(Self)
                            } else {
                                Err(::axum::http::StatusCode::NOT_FOUND)
                            }
                        }
                    }
                    #[doc(hidden)]
                    #[allow(
                        non_upper_case_globals,
                        unused_attributes,
                        unused_qualifications,
                        clippy::absolute_paths,
                    )]
                    const _: () = {
                        #[allow(unused_extern_crates, clippy::useless_attribute)]
                        extern crate serde as _serde;
                        #[automatically_derived]
                        impl<'de> _serde::Deserialize<'de> for HandleWebhookRoute {
                            fn deserialize<__D>(
                                __deserializer: __D,
                            ) -> _serde::__private225::Result<Self, __D::Error>
                            where
                                __D: _serde::Deserializer<'de>,
                            {
                                #[doc(hidden)]
                                struct __Visitor<'de> {
                                    marker: _serde::__private225::PhantomData<
                                        HandleWebhookRoute,
                                    >,
                                    lifetime: _serde::__private225::PhantomData<&'de ()>,
                                }
                                #[automatically_derived]
                                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                    type Value = HandleWebhookRoute;
                                    fn expecting(
                                        &self,
                                        __formatter: &mut _serde::__private225::Formatter,
                                    ) -> _serde::__private225::fmt::Result {
                                        _serde::__private225::Formatter::write_str(
                                            __formatter,
                                            "unit struct HandleWebhookRoute",
                                        )
                                    }
                                    #[inline]
                                    fn visit_unit<__E>(
                                        self,
                                    ) -> _serde::__private225::Result<Self::Value, __E>
                                    where
                                        __E: _serde::de::Error,
                                    {
                                        _serde::__private225::Ok(HandleWebhookRoute)
                                    }
                                }
                                _serde::Deserializer::deserialize_unit_struct(
                                    __deserializer,
                                    "HandleWebhookRoute",
                                    __Visitor {
                                        marker: _serde::__private225::PhantomData::<
                                            HandleWebhookRoute,
                                        >,
                                        lifetime: _serde::__private225::PhantomData,
                                    },
                                )
                            }
                        }
                    };
                    pub struct GithubDelivery(pub String);
                    impl GithubDelivery {
                        pub fn into_inner(self) -> String {
                            self.0
                        }
                    }
                    impl Header for GithubDelivery {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-github-delivery",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            str_value
                                .parse::<String>()
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let value_str = self.0.to_string();
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct HubSignature(pub String);
                    impl HubSignature {
                        pub fn into_inner(self) -> String {
                            self.0
                        }
                    }
                    impl Header for HubSignature {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-hub-signature",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            str_value
                                .parse::<String>()
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let value_str = self.0.to_string();
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct HubSignature256(pub String);
                    impl HubSignature256 {
                        pub fn into_inner(self) -> String {
                            self.0
                        }
                    }
                    impl Header for HubSignature256 {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-hub-signature-256",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            str_value
                                .parse::<String>()
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let value_str = self.0.to_string();
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct GithubWebhookEventType(pub WebhookEventType);
                    impl GithubWebhookEventType {
                        pub fn into_inner(self) -> WebhookEventType {
                            self.0
                        }
                    }
                    impl Header for GithubWebhookEventType {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-github-event",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            serde_json::from_str::<WebhookEventType>(str_value)
                                .or_else(|_| serde_json::from_str::<
                                    WebhookEventType,
                                >(
                                    &::alloc::__export::must_use({
                                        ::alloc::fmt::format(format_args!("\"{0}\"", str_value))
                                    }),
                                ))
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let json_str = serde_json::to_string(&self.0)
                                .unwrap_or_default();
                            let value_str = if json_str.starts_with('"')
                                && json_str.ends_with('"')
                            {
                                json_str[1..json_str.len() - 1].to_string()
                            } else {
                                json_str
                            };
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct GithubHookId(pub u64);
                    impl GithubHookId {
                        pub fn into_inner(self) -> u64 {
                            self.0
                        }
                    }
                    impl Header for GithubHookId {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-github-hook-id",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            str_value
                                .parse::<u64>()
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let value_str = self.0.to_string();
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct GithubHookInstallationTargetId(pub u64);
                    impl GithubHookInstallationTargetId {
                        pub fn into_inner(self) -> u64 {
                            self.0
                        }
                    }
                    impl Header for GithubHookInstallationTargetId {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-github-hook-installation-target-id",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            str_value
                                .parse::<u64>()
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let value_str = self.0.to_string();
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct GithubHookInstallationTargetType(pub String);
                    impl GithubHookInstallationTargetType {
                        pub fn into_inner(self) -> String {
                            self.0
                        }
                    }
                    impl Header for GithubHookInstallationTargetType {
                        fn name() -> &'static HeaderName {
                            static NAME: HeaderName = HeaderName::from_static(
                                "x-github-hook-installation-target-type",
                            );
                            &NAME
                        }
                        fn decode<'i, I>(values: &mut I) -> Result<Self, headers::Error>
                        where
                            Self: Sized,
                            I: Iterator<Item = &'i HeaderValue>,
                        {
                            let value = values
                                .next()
                                .ok_or_else(headers::Error::invalid)?;
                            let str_value = value
                                .to_str()
                                .map_err(|_| headers::Error::invalid())?;
                            str_value
                                .parse::<String>()
                                .map(Self)
                                .map_err(|_| headers::Error::invalid())
                        }
                        fn encode<E: Extend<HeaderValue>>(&self, values: &mut E) {
                            let value_str = self.0.to_string();
                            values
                                .extend(
                                    std::iter::once(
                                        HeaderValue::from_str(&value_str)
                                            .expect("invalid header value"),
                                    ),
                                );
                        }
                    }
                    pub struct GithubWebhookHeaders {
                        #[from_request(via(TypedHeader))]
                        pub delivery: GithubDelivery,
                        #[from_request(via(TypedHeader))]
                        pub signature: HubSignature,
                        #[from_request(via(TypedHeader))]
                        pub signature_256: HubSignature256,
                        #[from_request(via(TypedHeader))]
                        pub event: GithubWebhookEventType,
                        #[from_request(via(TypedHeader))]
                        pub hook_id: GithubHookId,
                        #[from_request(via(TypedHeader))]
                        pub installation_target_id: GithubHookInstallationTargetId,
                        #[from_request(via(TypedHeader))]
                        pub installation_target_type: GithubHookInstallationTargetType,
                    }
                    #[automatically_derived]
                    impl<S> ::axum::extract::FromRequestParts<S> for GithubWebhookHeaders
                    where
                        S: ::std::marker::Send + ::std::marker::Sync,
                    {
                        type Rejection = ::axum::response::Response;
                        async fn from_request_parts(
                            parts: &mut ::axum::http::request::Parts,
                            state: &S,
                        ) -> ::std::result::Result<Self, Self::Rejection> {
                            ::std::result::Result::Ok(Self {
                                delivery: {
                                    <TypedHeader<
                                        GithubDelivery,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                                signature: {
                                    <TypedHeader<
                                        HubSignature,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                                signature_256: {
                                    <TypedHeader<
                                        HubSignature256,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                                event: {
                                    <TypedHeader<
                                        GithubWebhookEventType,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                                hook_id: {
                                    <TypedHeader<
                                        GithubHookId,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                                installation_target_id: {
                                    <TypedHeader<
                                        GithubHookInstallationTargetId,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                                installation_target_type: {
                                    <TypedHeader<
                                        GithubHookInstallationTargetType,
                                    > as ::axum::extract::FromRequestParts<
                                        _,
                                    >>::from_request_parts(parts, state)
                                        .await
                                        .map(|TypedHeader(inner)| inner)
                                        .map_err(::axum::response::IntoResponse::into_response)?
                                },
                            })
                        }
                    }
                    /// Handle a GitHub webhook.
                    pub async fn handle_webhook(
                        _: HandleWebhookRoute,
                        headers: GithubWebhookHeaders,
                        Json(payload): Json<Value>,
                    ) -> () {
                        {
                            ::std::io::_print(
                                format_args!("Delivery ID: {0}\n", headers.delivery.0),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Hook ID: {0}\n", headers.hook_id.0),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Installation Target ID: {0}\n",
                                    headers.installation_target_id.0,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Installation Target Type: {0}\n",
                                    headers.installation_target_type.0,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!("Signature: {0}\n", headers.signature.0),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Signature 256: {0}\n",
                                    headers.signature_256.0,
                                ),
                            );
                        };
                        {
                            ::std::io::_print(
                                format_args!(
                                    "Payload: {0}\n",
                                    serde_json::to_string_pretty(&payload).unwrap(),
                                ),
                            );
                        };
                        let event_header = headers.event.into_inner();
                        let event_header_str = serde_json::to_string(&event_header)
                            .unwrap();
                        let webhook_event = WebhookEvent::try_from_header_and_body(
                                &event_header_str,
                                &payload.to_string(),
                            )
                            .unwrap();
                        let event_payload = event_header
                            .parse_specific_payload(payload)
                            .unwrap();
                        match event_payload {
                            WebhookEventPayload::IssueComment(payload) => {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:77",
                                                "sandcastle_core::application::http::webhook::github::handler",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(77u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!("repository: {0:?}", webhook_event.repository)
                                                                    as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::tracing::__macro_support::Option::Some(
                                                                                        &format_args!("repository: {0:?}", webhook_event.repository)
                                                                                            as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:78",
                                                "sandcastle_core::application::http::webhook::github::handler",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(78u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!("sender: {0:?}", webhook_event.sender)
                                                                    as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::tracing::__macro_support::Option::Some(
                                                                                        &format_args!("sender: {0:?}", webhook_event.sender)
                                                                                            as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:79",
                                                "sandcastle_core::application::http::webhook::github::handler",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(79u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "installation: {0:?}",
                                                                    webhook_event.installation,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::tracing::__macro_support::Option::Some(
                                                                                        &format_args!(
                                                                                            "installation: {0:?}",
                                                                                            webhook_event.installation,
                                                                                        ) as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            }
                            _ => {
                                {
                                    use ::tracing::__macro_support::Callsite as _;
                                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                        static META: ::tracing::Metadata<'static> = {
                                            ::tracing_core::metadata::Metadata::new(
                                                "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:82",
                                                "sandcastle_core::application::http::webhook::github::handler",
                                                ::tracing::Level::INFO,
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                ),
                                                ::tracing_core::__macro_support::Option::Some(82u32),
                                                ::tracing_core::__macro_support::Option::Some(
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                ),
                                                ::tracing_core::field::FieldSet::new(
                                                    &["message"],
                                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                ),
                                                ::tracing::metadata::Kind::EVENT,
                                            )
                                        };
                                        ::tracing::callsite::DefaultCallsite::new(&META)
                                    };
                                    let enabled = ::tracing::Level::INFO
                                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                        && ::tracing::Level::INFO
                                            <= ::tracing::level_filters::LevelFilter::current()
                                        && {
                                            let interest = __CALLSITE.interest();
                                            !interest.is_never()
                                                && ::tracing::__macro_support::__is_enabled(
                                                    __CALLSITE.metadata(),
                                                    interest,
                                                )
                                        };
                                    if enabled {
                                        (|value_set: ::tracing::field::ValueSet| {
                                            let meta = __CALLSITE.metadata();
                                            ::tracing::Event::dispatch(meta, &value_set);
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &value_set,
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        })({
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                            __CALLSITE
                                                .metadata()
                                                .fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &format_args!(
                                                                    "received unhandled event {0:?}",
                                                                    event_payload,
                                                                ) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        });
                                    } else {
                                        if match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        } <= ::tracing::log::STATIC_MAX_LEVEL
                                        {
                                            if !::tracing::dispatcher::has_been_set() {
                                                {
                                                    use ::tracing::log;
                                                    let level = match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    };
                                                    if level <= log::max_level() {
                                                        let meta = __CALLSITE.metadata();
                                                        let log_meta = log::Metadata::builder()
                                                            .level(level)
                                                            .target(meta.target())
                                                            .build();
                                                        let logger = log::logger();
                                                        if logger.enabled(&log_meta) {
                                                            ::tracing::__macro_support::__tracing_log(
                                                                meta,
                                                                logger,
                                                                log_meta,
                                                                &{
                                                                    #[allow(unused_imports)]
                                                                    use ::tracing::field::{debug, display, Value};
                                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                                    __CALLSITE
                                                                        .metadata()
                                                                        .fields()
                                                                        .value_set(
                                                                            &[
                                                                                (
                                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                                    ::tracing::__macro_support::Option::Some(
                                                                                        &format_args!(
                                                                                            "received unhandled event {0:?}",
                                                                                            event_payload,
                                                                                        ) as &dyn Value,
                                                                                    ),
                                                                                ),
                                                                            ],
                                                                        )
                                                                },
                                                            )
                                                        }
                                                    }
                                                }
                                            } else {
                                                {}
                                            }
                                        } else {
                                            {}
                                        };
                                    }
                                };
                            }
                        }
                    }
                    #[allow(warnings)]
                    #[allow(unreachable_code)]
                    #[doc(hidden)]
                    async fn __axum_macros_check_handle_webhook_into_response() {
                        #[allow(warnings)]
                        #[allow(unreachable_code)]
                        #[doc(hidden)]
                        async fn __axum_macros_check_handle_webhook_into_response_make_value() -> () {
                            let _: HandleWebhookRoute = {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                const fn panic_cold_explicit() -> ! {
                                    ::core::panicking::panic_explicit()
                                }
                                panic_cold_explicit();
                            };
                            let headers: GithubWebhookHeaders = {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                const fn panic_cold_explicit() -> ! {
                                    ::core::panicking::panic_explicit()
                                }
                                panic_cold_explicit();
                            };
                            let Json(payload): Json<Value> = {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                const fn panic_cold_explicit() -> ! {
                                    ::core::panicking::panic_explicit()
                                }
                                panic_cold_explicit();
                            };
                            {
                                {
                                    ::std::io::_print(
                                        format_args!("Delivery ID: {0}\n", headers.delivery.0),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!("Hook ID: {0}\n", headers.hook_id.0),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "Installation Target ID: {0}\n",
                                            headers.installation_target_id.0,
                                        ),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "Installation Target Type: {0}\n",
                                            headers.installation_target_type.0,
                                        ),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!("Signature: {0}\n", headers.signature.0),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "Signature 256: {0}\n",
                                            headers.signature_256.0,
                                        ),
                                    );
                                };
                                {
                                    ::std::io::_print(
                                        format_args!(
                                            "Payload: {0}\n",
                                            serde_json::to_string_pretty(&payload).unwrap(),
                                        ),
                                    );
                                };
                                let event_header = headers.event.into_inner();
                                let event_header_str = serde_json::to_string(&event_header)
                                    .unwrap();
                                let webhook_event = WebhookEvent::try_from_header_and_body(
                                        &event_header_str,
                                        &payload.to_string(),
                                    )
                                    .unwrap();
                                let event_payload = event_header
                                    .parse_specific_payload(payload)
                                    .unwrap();
                                match event_payload {
                                    WebhookEventPayload::IssueComment(payload) => {
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:77",
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                        ::tracing::Level::INFO,
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                        ),
                                                        ::tracing_core::__macro_support::Option::Some(77u32),
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "sandcastle_core::application::http::webhook::github::handler",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::INFO
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::INFO
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::INFO {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!("repository: {0:?}", webhook_event.repository)
                                                                            as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::tracing::__macro_support::Option::Some(
                                                                                                &format_args!("repository: {0:?}", webhook_event.repository)
                                                                                                    as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:78",
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                        ::tracing::Level::INFO,
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                        ),
                                                        ::tracing_core::__macro_support::Option::Some(78u32),
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "sandcastle_core::application::http::webhook::github::handler",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::INFO
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::INFO
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::INFO {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!("sender: {0:?}", webhook_event.sender)
                                                                            as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::tracing::__macro_support::Option::Some(
                                                                                                &format_args!("sender: {0:?}", webhook_event.sender)
                                                                                                    as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:79",
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                        ::tracing::Level::INFO,
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                        ),
                                                        ::tracing_core::__macro_support::Option::Some(79u32),
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "sandcastle_core::application::http::webhook::github::handler",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::INFO
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::INFO
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::INFO {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!(
                                                                            "installation: {0:?}",
                                                                            webhook_event.installation,
                                                                        ) as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::tracing::__macro_support::Option::Some(
                                                                                                &format_args!(
                                                                                                    "installation: {0:?}",
                                                                                                    webhook_event.installation,
                                                                                                ) as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                    }
                                    _ => {
                                        {
                                            use ::tracing::__macro_support::Callsite as _;
                                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                                static META: ::tracing::Metadata<'static> = {
                                                    ::tracing_core::metadata::Metadata::new(
                                                        "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:82",
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                        ::tracing::Level::INFO,
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                        ),
                                                        ::tracing_core::__macro_support::Option::Some(82u32),
                                                        ::tracing_core::__macro_support::Option::Some(
                                                            "sandcastle_core::application::http::webhook::github::handler",
                                                        ),
                                                        ::tracing_core::field::FieldSet::new(
                                                            &["message"],
                                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                        ),
                                                        ::tracing::metadata::Kind::EVENT,
                                                    )
                                                };
                                                ::tracing::callsite::DefaultCallsite::new(&META)
                                            };
                                            let enabled = ::tracing::Level::INFO
                                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                                && ::tracing::Level::INFO
                                                    <= ::tracing::level_filters::LevelFilter::current()
                                                && {
                                                    let interest = __CALLSITE.interest();
                                                    !interest.is_never()
                                                        && ::tracing::__macro_support::__is_enabled(
                                                            __CALLSITE.metadata(),
                                                            interest,
                                                        )
                                                };
                                            if enabled {
                                                (|value_set: ::tracing::field::ValueSet| {
                                                    let meta = __CALLSITE.metadata();
                                                    ::tracing::Event::dispatch(meta, &value_set);
                                                    if match ::tracing::Level::INFO {
                                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                        _ => ::tracing::log::Level::Trace,
                                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                                    {
                                                        if !::tracing::dispatcher::has_been_set() {
                                                            {
                                                                use ::tracing::log;
                                                                let level = match ::tracing::Level::INFO {
                                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                    _ => ::tracing::log::Level::Trace,
                                                                };
                                                                if level <= log::max_level() {
                                                                    let meta = __CALLSITE.metadata();
                                                                    let log_meta = log::Metadata::builder()
                                                                        .level(level)
                                                                        .target(meta.target())
                                                                        .build();
                                                                    let logger = log::logger();
                                                                    if logger.enabled(&log_meta) {
                                                                        ::tracing::__macro_support::__tracing_log(
                                                                            meta,
                                                                            logger,
                                                                            log_meta,
                                                                            &value_set,
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        } else {
                                                            {}
                                                        }
                                                    } else {
                                                        {}
                                                    };
                                                })({
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!(
                                                                            "received unhandled event {0:?}",
                                                                            event_payload,
                                                                        ) as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                });
                                            } else {
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &{
                                                                            #[allow(unused_imports)]
                                                                            use ::tracing::field::{debug, display, Value};
                                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                                            __CALLSITE
                                                                                .metadata()
                                                                                .fields()
                                                                                .value_set(
                                                                                    &[
                                                                                        (
                                                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                                            ::tracing::__macro_support::Option::Some(
                                                                                                &format_args!(
                                                                                                    "received unhandled event {0:?}",
                                                                                                    event_payload,
                                                                                                ) as &dyn Value,
                                                                                            ),
                                                                                        ),
                                                                                    ],
                                                                                )
                                                                        },
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            }
                                        };
                                    }
                                }
                            }
                        }
                        let value = __axum_macros_check_handle_webhook_into_response_make_value()
                            .await;
                        fn check<T>(_: T)
                        where
                            T: ::axum::response::IntoResponse,
                        {}
                        check(value);
                    }
                    #[allow(warnings)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_0_from_request_check()
                    where
                        HandleWebhookRoute: ::axum::extract::FromRequestParts<()> + Send,
                    {}
                    #[allow(warnings)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_0_from_request_call_check() {
                        __axum_macros_check_handle_webhook_0_from_request_check();
                    }
                    #[allow(warnings)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_1_from_request_check()
                    where
                        GithubWebhookHeaders: ::axum::extract::FromRequestParts<()>
                            + Send,
                    {}
                    #[allow(warnings)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_1_from_request_call_check() {
                        __axum_macros_check_handle_webhook_1_from_request_check();
                    }
                    #[allow(warnings)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_2_from_request_check()
                    where
                        Json<Value>: ::axum::extract::FromRequest<()> + Send,
                    {}
                    #[allow(warnings)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_2_from_request_call_check() {
                        __axum_macros_check_handle_webhook_2_from_request_check();
                    }
                    #[allow(warnings)]
                    #[allow(unreachable_code)]
                    #[doc(hidden)]
                    fn __axum_macros_check_handle_webhook_future() {
                        /// Handle a GitHub webhook.
                        pub async fn handle_webhook(
                            _: HandleWebhookRoute,
                            headers: GithubWebhookHeaders,
                            Json(payload): Json<Value>,
                        ) -> () {
                            {
                                ::std::io::_print(
                                    format_args!("Delivery ID: {0}\n", headers.delivery.0),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Hook ID: {0}\n", headers.hook_id.0),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Installation Target ID: {0}\n",
                                        headers.installation_target_id.0,
                                    ),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Installation Target Type: {0}\n",
                                        headers.installation_target_type.0,
                                    ),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!("Signature: {0}\n", headers.signature.0),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Signature 256: {0}\n",
                                        headers.signature_256.0,
                                    ),
                                );
                            };
                            {
                                ::std::io::_print(
                                    format_args!(
                                        "Payload: {0}\n",
                                        serde_json::to_string_pretty(&payload).unwrap(),
                                    ),
                                );
                            };
                            let event_header = headers.event.into_inner();
                            let event_header_str = serde_json::to_string(&event_header)
                                .unwrap();
                            let webhook_event = WebhookEvent::try_from_header_and_body(
                                    &event_header_str,
                                    &payload.to_string(),
                                )
                                .unwrap();
                            let event_payload = event_header
                                .parse_specific_payload(payload)
                                .unwrap();
                            match event_payload {
                                WebhookEventPayload::IssueComment(payload) => {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:77",
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(77u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("repository: {0:?}", webhook_event.repository)
                                                                        as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::tracing::__macro_support::Option::Some(
                                                                                            &format_args!("repository: {0:?}", webhook_event.repository)
                                                                                                as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:78",
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(78u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("sender: {0:?}", webhook_event.sender)
                                                                        as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::tracing::__macro_support::Option::Some(
                                                                                            &format_args!("sender: {0:?}", webhook_event.sender)
                                                                                                as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:79",
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(79u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!(
                                                                        "installation: {0:?}",
                                                                        webhook_event.installation,
                                                                    ) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::tracing::__macro_support::Option::Some(
                                                                                            &format_args!(
                                                                                                "installation: {0:?}",
                                                                                                webhook_event.installation,
                                                                                            ) as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                }
                                _ => {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/sandcastle-core/src/application/http/webhook/github/handler.rs:82",
                                                    "sandcastle_core::application::http::webhook::github::handler",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/sandcastle-core/src/application/http/webhook/github/handler.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(82u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "sandcastle_core::application::http::webhook::github::handler",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!(
                                                                        "received unhandled event {0:?}",
                                                                        event_payload,
                                                                    ) as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::tracing::__macro_support::Option::Some(
                                                                                            &format_args!(
                                                                                                "received unhandled event {0:?}",
                                                                                                event_payload,
                                                                                            ) as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                }
                            }
                        }
                        let future = handle_webhook(
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                const fn panic_cold_explicit() -> ! {
                                    ::core::panicking::panic_explicit()
                                }
                                panic_cold_explicit();
                            },
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                const fn panic_cold_explicit() -> ! {
                                    ::core::panicking::panic_explicit()
                                }
                                panic_cold_explicit();
                            },
                            {
                                #[cold]
                                #[track_caller]
                                #[inline(never)]
                                const fn panic_cold_explicit() -> ! {
                                    ::core::panicking::panic_explicit()
                                }
                                panic_cold_explicit();
                            },
                        );
                        fn check<T>(_: T)
                        where
                            T: ::std::future::Future + Send,
                        {}
                        check(future);
                    }
                }
            }
            pub fn router() -> Router {
                Router::new().typed_post(github::handler::handle_webhook)
            }
        }
        /**
 * Start the HTTP server
 */
        pub async fn start() {
            let _guard = init_tracing_opentelemetry::TracingConfig::development()
                .init_subscriber()
                .unwrap();
            let router = webhook::router()
                .layer(OtelInResponseLayer)
                .layer(OtelAxumLayer::default());
            let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
            axum::serve(listener, router).await.unwrap();
        }
    }
    mod operator {
        use std::{sync::Arc, time::Duration};
        use futures::StreamExt;
        use k8s_openapi::api::core::v1::Secret;
        use kube::{
            Api, Client, ResourceExt, api::ListParams,
            runtime::{Controller, controller::Action, finalizer},
        };
        use snafu::ResultExt;
        use tracing::instrument;
        use crate::{
            Result,
            domain::repositories::{
                models::RepositoryConfiguration, ports::RepositoryConfigurationService,
                services::RepositoryConfigurations,
            },
            error::{FinalizerSnafu, SandcastleError},
            infrastructure::repo_config_service::GithubAppSecretData,
        };
        pub struct OperatorContext {
            pub client: Client,
            pub repository_configuration_service: RepositoryConfigurations,
            pub namespace: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OperatorContext {
            #[inline]
            fn clone(&self) -> OperatorContext {
                OperatorContext {
                    client: ::core::clone::Clone::clone(&self.client),
                    repository_configuration_service: ::core::clone::Clone::clone(
                        &self.repository_configuration_service,
                    ),
                    namespace: ::core::clone::Clone::clone(&self.namespace),
                }
            }
        }
        const SANDCASTLE_FINALIZER: &str = "sandcastle.dev/finalizer";
        async fn apply(
            secret: Arc<Secret>,
            context: Arc<OperatorContext>,
        ) -> Result<Action> {
            let github_app_secret_data = GithubAppSecretData::from_secret(secret)?;
            let repository_config = RepositoryConfiguration::from(
                github_app_secret_data,
            );
            context
                .repository_configuration_service
                .upsert_repository_configuration(repository_config)
                .await?;
            Ok(Action::requeue(Duration::from_secs(5 * 60)))
        }
        async fn cleanup(
            secret: Arc<Secret>,
            context: Arc<OperatorContext>,
        ) -> Result<Action> {
            context
                .repository_configuration_service
                .delete_repository_configuration(&secret.name_any())
                .await?;
            Ok(Action::requeue(Duration::from_secs(5 * 60)))
        }
        async fn reconcile(
            secret: Arc<Secret>,
            context: Arc<OperatorContext>,
        ) -> Result<Action> {
            {}
            let __tracing_attr_span = {
                use ::tracing::__macro_support::Callsite as _;
                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "reconcile",
                            "sandcastle_core::application::operator",
                            ::tracing::Level::INFO,
                            ::tracing_core::__macro_support::Option::Some(
                                "crates/sandcastle-core/src/application/operator.rs",
                            ),
                            ::tracing_core::__macro_support::Option::Some(50u32),
                            ::tracing_core::__macro_support::Option::Some(
                                "sandcastle_core::application::operator",
                            ),
                            ::tracing_core::field::FieldSet::new(
                                &["_secret_name"],
                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = __CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        __CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = __CALLSITE.metadata();
                    ::tracing::Span::new(
                        meta,
                        &{
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = meta.fields().iter();
                            meta.fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &::tracing::field::Empty as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        },
                    )
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        __CALLSITE.metadata(),
                    );
                    if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            {
                                span.record_all(
                                    &{
                                        #[allow(unused_imports)]
                                        use ::tracing::field::{debug, display, Value};
                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                        __CALLSITE
                                            .metadata()
                                            .fields()
                                            .value_set(
                                                &[
                                                    (
                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                        ::tracing::__macro_support::Option::Some(
                                                            &::tracing::field::Empty as &dyn Value,
                                                        ),
                                                    ),
                                                ],
                                            )
                                    },
                                );
                            }
                        } else {
                            {}
                        }
                    } else {
                        {}
                    };
                    span
                }
            };
            let __tracing_instrument_future = async move {
                #[allow(
                    unknown_lints,
                    unreachable_code,
                    clippy::diverging_sub_expression,
                    clippy::empty_loop,
                    clippy::let_unit_value,
                    clippy::let_with_type_underscore,
                    clippy::needless_return,
                    clippy::unreachable
                )]
                if false {
                    let __tracing_attr_fake_return: Result<Action> = loop {};
                    return __tracing_attr_fake_return;
                }
                {
                    let _secret_name = &secret.name_any();
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/sandcastle-core/src/application/operator.rs:53",
                                    "sandcastle_core::application::operator",
                                    ::tracing::Level::INFO,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "crates/sandcastle-core/src/application/operator.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(53u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "sandcastle_core::application::operator",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                                if match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &value_set,
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!("Reconciling secret") as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                            if match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &{
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                        __CALLSITE
                                                            .metadata()
                                                            .fields()
                                                            .value_set(
                                                                &[
                                                                    (
                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                        ::tracing::__macro_support::Option::Some(
                                                                            &format_args!("Reconciling secret") as &dyn Value,
                                                                        ),
                                                                    ),
                                                                ],
                                                            )
                                                    },
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        }
                    };
                    let secrets = Api::<
                        Secret,
                    >::namespaced(context.client.clone(), &context.namespace);
                    finalizer(
                            &secrets,
                            SANDCASTLE_FINALIZER,
                            secret,
                            |event| async move {
                                match event {
                                    finalizer::Event::Apply(secret) => {
                                        apply(secret, context.clone()).await
                                    }
                                    finalizer::Event::Cleanup(secret) => {
                                        cleanup(secret, context.clone()).await
                                    }
                                }
                            },
                        )
                        .await
                        .context(FinalizerSnafu)
                }
            };
            if !__tracing_attr_span.is_disabled() {
                ::tracing::Instrument::instrument(
                        __tracing_instrument_future,
                        __tracing_attr_span,
                    )
                    .await
            } else {
                __tracing_instrument_future.await
            }
        }
        fn error_policy(
            secret: Arc<Secret>,
            error: &SandcastleError,
            _context: Arc<OperatorContext>,
        ) -> Action {
            {}
            #[allow(clippy::suspicious_else_formatting)]
            {
                let __tracing_attr_span;
                let __tracing_attr_guard;
                if ::tracing::Level::INFO <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::INFO
                        <= ::tracing::level_filters::LevelFilter::current()
                    || if match ::tracing::Level::INFO {
                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                        _ => ::tracing::log::Level::Trace,
                    } <= ::tracing::log::STATIC_MAX_LEVEL
                    {
                        if !::tracing::dispatcher::has_been_set() {
                            { true }
                        } else {
                            { false }
                        }
                    } else {
                        { false }
                    }
                {
                    __tracing_attr_span = {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "error_policy",
                                    "sandcastle_core::application::operator",
                                    ::tracing::Level::INFO,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "crates/sandcastle-core/src/application/operator.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(66u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "sandcastle_core::application::operator",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["_secret_name"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::SPAN,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let mut interest = ::tracing::subscriber::Interest::never();
                        if ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                interest = __CALLSITE.interest();
                                !interest.is_never()
                            }
                            && ::tracing::__macro_support::__is_enabled(
                                __CALLSITE.metadata(),
                                interest,
                            )
                        {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Span::new(
                                meta,
                                &{
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = meta.fields().iter();
                                    meta.fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &::tracing::field::Empty as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                },
                            )
                        } else {
                            let span = ::tracing::__macro_support::__disabled_span(
                                __CALLSITE.metadata(),
                            );
                            if match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        span.record_all(
                                            &{
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &::tracing::field::Empty as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            },
                                        );
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                            span
                        }
                    };
                    __tracing_attr_guard = __tracing_attr_span.enter();
                }
                #[warn(clippy::suspicious_else_formatting)]
                {
                    #[allow(
                        unknown_lints,
                        unreachable_code,
                        clippy::diverging_sub_expression,
                        clippy::empty_loop,
                        clippy::let_unit_value,
                        clippy::let_with_type_underscore,
                        clippy::needless_return,
                        clippy::unreachable
                    )]
                    if false {
                        let __tracing_attr_fake_return: Action = loop {};
                        return __tracing_attr_fake_return;
                    }
                    {
                        let _secret_name = secret.name_any();
                        {
                            use ::tracing::__macro_support::Callsite as _;
                            static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "event crates/sandcastle-core/src/application/operator.rs:73",
                                        "sandcastle_core::application::operator",
                                        ::tracing::Level::WARN,
                                        ::tracing_core::__macro_support::Option::Some(
                                            "crates/sandcastle-core/src/application/operator.rs",
                                        ),
                                        ::tracing_core::__macro_support::Option::Some(73u32),
                                        ::tracing_core::__macro_support::Option::Some(
                                            "sandcastle_core::application::operator",
                                        ),
                                        ::tracing_core::field::FieldSet::new(
                                            &["message"],
                                            ::tracing_core::callsite::Identifier(&__CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::EVENT,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let enabled = ::tracing::Level::WARN
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::WARN
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    let interest = __CALLSITE.interest();
                                    !interest.is_never()
                                        && ::tracing::__macro_support::__is_enabled(
                                            __CALLSITE.metadata(),
                                            interest,
                                        )
                                };
                            if enabled {
                                (|value_set: ::tracing::field::ValueSet| {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Event::dispatch(meta, &value_set);
                                    if match ::tracing::Level::WARN {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                use ::tracing::log;
                                                let level = match ::tracing::Level::WARN {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                };
                                                if level <= log::max_level() {
                                                    let meta = __CALLSITE.metadata();
                                                    let log_meta = log::Metadata::builder()
                                                        .level(level)
                                                        .target(meta.target())
                                                        .build();
                                                    let logger = log::logger();
                                                    if logger.enabled(&log_meta) {
                                                        ::tracing::__macro_support::__tracing_log(
                                                            meta,
                                                            logger,
                                                            log_meta,
                                                            &value_set,
                                                        )
                                                    }
                                                }
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                })({
                                    #[allow(unused_imports)]
                                    use ::tracing::field::{debug, display, Value};
                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                    __CALLSITE
                                        .metadata()
                                        .fields()
                                        .value_set(
                                            &[
                                                (
                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                    ::tracing::__macro_support::Option::Some(
                                                        &format_args!("Failed reconciling secret {0:?}", error)
                                                            as &dyn Value,
                                                    ),
                                                ),
                                            ],
                                        )
                                });
                            } else {
                                if match ::tracing::Level::WARN {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match ::tracing::Level::WARN {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &{
                                                            #[allow(unused_imports)]
                                                            use ::tracing::field::{debug, display, Value};
                                                            let mut iter = __CALLSITE.metadata().fields().iter();
                                                            __CALLSITE
                                                                .metadata()
                                                                .fields()
                                                                .value_set(
                                                                    &[
                                                                        (
                                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                                            ::tracing::__macro_support::Option::Some(
                                                                                &format_args!("Failed reconciling secret {0:?}", error)
                                                                                    as &dyn Value,
                                                                            ),
                                                                        ),
                                                                    ],
                                                                )
                                                        },
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            }
                        };
                        Action::requeue(Duration::from_secs(5 * 60))
                    }
                }
            }
        }
        pub async fn run(client: Client, context: OperatorContext) {
            let secrets = Api::<Secret>::namespaced(client.clone(), &context.namespace);
            if let Err(e) = secrets.list(&ListParams::default().limit(1)).await {
                {
                    use ::tracing::__macro_support::Callsite as _;
                    static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                        static META: ::tracing::Metadata<'static> = {
                            ::tracing_core::metadata::Metadata::new(
                                "event crates/sandcastle-core/src/application/operator.rs:81",
                                "sandcastle_core::application::operator",
                                ::tracing::Level::ERROR,
                                ::tracing_core::__macro_support::Option::Some(
                                    "crates/sandcastle-core/src/application/operator.rs",
                                ),
                                ::tracing_core::__macro_support::Option::Some(81u32),
                                ::tracing_core::__macro_support::Option::Some(
                                    "sandcastle_core::application::operator",
                                ),
                                ::tracing_core::field::FieldSet::new(
                                    &["message"],
                                    ::tracing_core::callsite::Identifier(&__CALLSITE),
                                ),
                                ::tracing::metadata::Kind::EVENT,
                            )
                        };
                        ::tracing::callsite::DefaultCallsite::new(&META)
                    };
                    let enabled = ::tracing::Level::ERROR
                        <= ::tracing::level_filters::STATIC_MAX_LEVEL
                        && ::tracing::Level::ERROR
                            <= ::tracing::level_filters::LevelFilter::current()
                        && {
                            let interest = __CALLSITE.interest();
                            !interest.is_never()
                                && ::tracing::__macro_support::__is_enabled(
                                    __CALLSITE.metadata(),
                                    interest,
                                )
                        };
                    if enabled {
                        (|value_set: ::tracing::field::ValueSet| {
                            let meta = __CALLSITE.metadata();
                            ::tracing::Event::dispatch(meta, &value_set);
                            if match ::tracing::Level::ERROR {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::ERROR {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &value_set,
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        })({
                            #[allow(unused_imports)]
                            use ::tracing::field::{debug, display, Value};
                            let mut iter = __CALLSITE.metadata().fields().iter();
                            __CALLSITE
                                .metadata()
                                .fields()
                                .value_set(
                                    &[
                                        (
                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                .expect("FieldSet corrupted (this is a bug)"),
                                            ::tracing::__macro_support::Option::Some(
                                                &format_args!(
                                                    "CRD is not queryable; {0:?}. Is the CRD installed?",
                                                    e,
                                                ) as &dyn Value,
                                            ),
                                        ),
                                    ],
                                )
                        });
                    } else {
                        if match ::tracing::Level::ERROR {
                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                            _ => ::tracing::log::Level::Trace,
                        } <= ::tracing::log::STATIC_MAX_LEVEL
                        {
                            if !::tracing::dispatcher::has_been_set() {
                                {
                                    use ::tracing::log;
                                    let level = match ::tracing::Level::ERROR {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    };
                                    if level <= log::max_level() {
                                        let meta = __CALLSITE.metadata();
                                        let log_meta = log::Metadata::builder()
                                            .level(level)
                                            .target(meta.target())
                                            .build();
                                        let logger = log::logger();
                                        if logger.enabled(&log_meta) {
                                            ::tracing::__macro_support::__tracing_log(
                                                meta,
                                                logger,
                                                log_meta,
                                                &{
                                                    #[allow(unused_imports)]
                                                    use ::tracing::field::{debug, display, Value};
                                                    let mut iter = __CALLSITE.metadata().fields().iter();
                                                    __CALLSITE
                                                        .metadata()
                                                        .fields()
                                                        .value_set(
                                                            &[
                                                                (
                                                                    &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                        .expect("FieldSet corrupted (this is a bug)"),
                                                                    ::tracing::__macro_support::Option::Some(
                                                                        &format_args!(
                                                                            "CRD is not queryable; {0:?}. Is the CRD installed?",
                                                                            e,
                                                                        ) as &dyn Value,
                                                                    ),
                                                                ),
                                                            ],
                                                        )
                                                },
                                            )
                                        }
                                    }
                                }
                            } else {
                                {}
                            }
                        } else {
                            {}
                        };
                    }
                };
                std::process::exit(1);
            }
            let watcher_config = kube::runtime::watcher::Config::default()
                .labels("sandcastle.dev/secret-type=repository");
            Controller::new(secrets, watcher_config.clone())
                .shutdown_on_signal()
                .run(reconcile, error_policy, Arc::new(context.clone()))
                .filter_map(|x| async move { std::result::Result::ok(x) })
                .for_each(|_| futures::future::ready(()))
                .await;
        }
    }
    /// State shared beetween the HTTP and Operator
    pub(crate) struct ApplicationState {
        pub(crate) kube_client: Client,
        pub(crate) namespace: String,
        pub(crate) repository_configuration_service: RepositoryConfigurations,
    }
    pub async fn start() -> Result<(), Box<dyn std::error::Error>> {
        let kube_client = Client::try_default().await?;
        let config = Config::infer().await?;
        let context = operator::OperatorContext {
            client: kube_client.clone(),
            repository_configuration_service: DefaultRepositoryConfigurationService::default()
                .into(),
            namespace: config.default_namespace,
        };
        {
            #[doc(hidden)]
            mod __tokio_select_util {
                pub(super) enum Out<_0, _1> {
                    _0(_0),
                    _1(_1),
                    Disabled,
                }
                pub(super) type Mask = u8;
            }
            use ::tokio::macros::support::Future;
            use ::tokio::macros::support::Pin;
            use ::tokio::macros::support::Poll::{Ready, Pending};
            const BRANCHES: u32 = 2;
            let mut disabled: __tokio_select_util::Mask = Default::default();
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 0;
                disabled |= mask;
            }
            if !true {
                let mask: __tokio_select_util::Mask = 1 << 1;
                disabled |= mask;
            }
            let mut output = {
                let futures_init = (http::start(), operator::run(kube_client, context));
                let mut futures = (
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.0),
                    ::tokio::macros::support::IntoFuture::into_future(futures_init.1),
                );
                let mut futures = &mut futures;
                ::tokio::macros::support::poll_fn(|cx| {
                        match ::tokio::macros::support::poll_budget_available(cx) {
                            ::core::task::Poll::Ready(t) => t,
                            ::core::task::Poll::Pending => {
                                return ::core::task::Poll::Pending;
                            }
                        };
                        let mut is_pending = false;
                        let start = { ::tokio::macros::support::thread_rng_n(BRANCHES) };
                        for i in 0..BRANCHES {
                            let branch;
                            #[allow(clippy::modulo_one)]
                            {
                                branch = (start + i) % BRANCHES;
                            }
                            match branch {
                                #[allow(unreachable_code)]
                                0 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        _ => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_0(out));
                                }
                                #[allow(unreachable_code)]
                                1 => {
                                    let mask = 1 << branch;
                                    if disabled & mask == mask {
                                        continue;
                                    }
                                    let (_, fut, ..) = &mut *futures;
                                    let mut fut = unsafe { Pin::new_unchecked(fut) };
                                    let out = match Future::poll(fut, cx) {
                                        Ready(out) => out,
                                        Pending => {
                                            is_pending = true;
                                            continue;
                                        }
                                    };
                                    disabled |= mask;
                                    #[allow(unused_variables)] #[allow(unused_mut)]
                                    match &out {
                                        _ => {}
                                        _ => continue,
                                    }
                                    return Ready(__tokio_select_util::Out::_1(out));
                                }
                                _ => {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "internal error: entered unreachable code: {0}",
                                            format_args!(
                                                "reaching this means there probably is an off by one bug",
                                            ),
                                        ),
                                    );
                                }
                            }
                        }
                        if is_pending {
                            Pending
                        } else {
                            Ready(__tokio_select_util::Out::Disabled)
                        }
                    })
                    .await
            };
            match output {
                __tokio_select_util::Out::_0(_) => {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/sandcastle-core/src/application.rs:27",
                                    "sandcastle_core::application",
                                    ::tracing::Level::INFO,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "crates/sandcastle-core/src/application.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(27u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "sandcastle_core::application",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                                if match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &value_set,
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!("HTTP server started") as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                            if match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &{
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                        __CALLSITE
                                                            .metadata()
                                                            .fields()
                                                            .value_set(
                                                                &[
                                                                    (
                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                        ::tracing::__macro_support::Option::Some(
                                                                            &format_args!("HTTP server started") as &dyn Value,
                                                                        ),
                                                                    ),
                                                                ],
                                                            )
                                                    },
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        }
                    };
                    Ok(())
                }
                __tokio_select_util::Out::_1(_) => {
                    {
                        use ::tracing::__macro_support::Callsite as _;
                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                            static META: ::tracing::Metadata<'static> = {
                                ::tracing_core::metadata::Metadata::new(
                                    "event crates/sandcastle-core/src/application.rs:31",
                                    "sandcastle_core::application",
                                    ::tracing::Level::INFO,
                                    ::tracing_core::__macro_support::Option::Some(
                                        "crates/sandcastle-core/src/application.rs",
                                    ),
                                    ::tracing_core::__macro_support::Option::Some(31u32),
                                    ::tracing_core::__macro_support::Option::Some(
                                        "sandcastle_core::application",
                                    ),
                                    ::tracing_core::field::FieldSet::new(
                                        &["message"],
                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                    ),
                                    ::tracing::metadata::Kind::EVENT,
                                )
                            };
                            ::tracing::callsite::DefaultCallsite::new(&META)
                        };
                        let enabled = ::tracing::Level::INFO
                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                            && ::tracing::Level::INFO
                                <= ::tracing::level_filters::LevelFilter::current()
                            && {
                                let interest = __CALLSITE.interest();
                                !interest.is_never()
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                            };
                        if enabled {
                            (|value_set: ::tracing::field::ValueSet| {
                                let meta = __CALLSITE.metadata();
                                ::tracing::Event::dispatch(meta, &value_set);
                                if match ::tracing::Level::INFO {
                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                    _ => ::tracing::log::Level::Trace,
                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                {
                                    if !::tracing::dispatcher::has_been_set() {
                                        {
                                            use ::tracing::log;
                                            let level = match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            };
                                            if level <= log::max_level() {
                                                let meta = __CALLSITE.metadata();
                                                let log_meta = log::Metadata::builder()
                                                    .level(level)
                                                    .target(meta.target())
                                                    .build();
                                                let logger = log::logger();
                                                if logger.enabled(&log_meta) {
                                                    ::tracing::__macro_support::__tracing_log(
                                                        meta,
                                                        logger,
                                                        log_meta,
                                                        &value_set,
                                                    )
                                                }
                                            }
                                        }
                                    } else {
                                        {}
                                    }
                                } else {
                                    {}
                                };
                            })({
                                #[allow(unused_imports)]
                                use ::tracing::field::{debug, display, Value};
                                let mut iter = __CALLSITE.metadata().fields().iter();
                                __CALLSITE
                                    .metadata()
                                    .fields()
                                    .value_set(
                                        &[
                                            (
                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                ::tracing::__macro_support::Option::Some(
                                                    &format_args!("Operator started") as &dyn Value,
                                                ),
                                            ),
                                        ],
                                    )
                            });
                        } else {
                            if match ::tracing::Level::INFO {
                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                _ => ::tracing::log::Level::Trace,
                            } <= ::tracing::log::STATIC_MAX_LEVEL
                            {
                                if !::tracing::dispatcher::has_been_set() {
                                    {
                                        use ::tracing::log;
                                        let level = match ::tracing::Level::INFO {
                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                            _ => ::tracing::log::Level::Trace,
                                        };
                                        if level <= log::max_level() {
                                            let meta = __CALLSITE.metadata();
                                            let log_meta = log::Metadata::builder()
                                                .level(level)
                                                .target(meta.target())
                                                .build();
                                            let logger = log::logger();
                                            if logger.enabled(&log_meta) {
                                                ::tracing::__macro_support::__tracing_log(
                                                    meta,
                                                    logger,
                                                    log_meta,
                                                    &{
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                        __CALLSITE
                                                            .metadata()
                                                            .fields()
                                                            .value_set(
                                                                &[
                                                                    (
                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                        ::tracing::__macro_support::Option::Some(
                                                                            &format_args!("Operator started") as &dyn Value,
                                                                        ),
                                                                    ),
                                                                ],
                                                            )
                                                    },
                                                )
                                            }
                                        }
                                    }
                                } else {
                                    {}
                                }
                            } else {
                                {}
                            };
                        }
                    };
                    Ok(())
                }
                __tokio_select_util::Out::Disabled => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "all branches are disabled and there is no else branch",
                        ),
                    );
                }
                _ => {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "internal error: entered unreachable code: {0}",
                            format_args!("failed to match bind"),
                        ),
                    );
                }
            }
        }
    }
}
mod domain {
    pub(crate) mod environment {
        pub mod models {
            mod config {
                use std::{
                    backtrace::Backtrace, ops::Deref, str::FromStr, sync::OnceLock,
                };
                use regex::Regex;
                use serde::{Deserialize, Serialize};
                use serde_yaml::Value;
                use crate::error::{SandcastleError, ServiceErrorCode};
                pub enum BuiltinConfigKey {
                    EnvironmentName,
                    RepoURL,
                    TargetRevision,
                    LastCommitSHA,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for BuiltinConfigKey {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::write_str(
                            f,
                            match self {
                                BuiltinConfigKey::EnvironmentName => "EnvironmentName",
                                BuiltinConfigKey::RepoURL => "RepoURL",
                                BuiltinConfigKey::TargetRevision => "TargetRevision",
                                BuiltinConfigKey::LastCommitSHA => "LastCommitSHA",
                            },
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for BuiltinConfigKey {
                    #[inline]
                    fn clone(&self) -> BuiltinConfigKey {
                        match self {
                            BuiltinConfigKey::EnvironmentName => {
                                BuiltinConfigKey::EnvironmentName
                            }
                            BuiltinConfigKey::RepoURL => BuiltinConfigKey::RepoURL,
                            BuiltinConfigKey::TargetRevision => {
                                BuiltinConfigKey::TargetRevision
                            }
                            BuiltinConfigKey::LastCommitSHA => {
                                BuiltinConfigKey::LastCommitSHA
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications,
                    clippy::absolute_paths,
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl _serde::Serialize for BuiltinConfigKey {
                        fn serialize<__S>(
                            &self,
                            __serializer: __S,
                        ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            match *self {
                                BuiltinConfigKey::EnvironmentName => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "BuiltinConfigKey",
                                        0u32,
                                        "EnvironmentName",
                                    )
                                }
                                BuiltinConfigKey::RepoURL => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "BuiltinConfigKey",
                                        1u32,
                                        "RepoURL",
                                    )
                                }
                                BuiltinConfigKey::TargetRevision => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "BuiltinConfigKey",
                                        2u32,
                                        "TargetRevision",
                                    )
                                }
                                BuiltinConfigKey::LastCommitSHA => {
                                    _serde::Serializer::serialize_unit_variant(
                                        __serializer,
                                        "BuiltinConfigKey",
                                        3u32,
                                        "LastCommitSHA",
                                    )
                                }
                            }
                        }
                    }
                };
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications,
                    clippy::absolute_paths,
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for BuiltinConfigKey {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private225::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __field1,
                                __field2,
                                __field3,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private225::Formatter,
                                ) -> _serde::__private225::fmt::Result {
                                    _serde::__private225::Formatter::write_str(
                                        __formatter,
                                        "variant identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private225::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private225::Ok(__Field::__field0),
                                        1u64 => _serde::__private225::Ok(__Field::__field1),
                                        2u64 => _serde::__private225::Ok(__Field::__field2),
                                        3u64 => _serde::__private225::Ok(__Field::__field3),
                                        _ => {
                                            _serde::__private225::Err(
                                                _serde::de::Error::invalid_value(
                                                    _serde::de::Unexpected::Unsigned(__value),
                                                    &"variant index 0 <= i < 4",
                                                ),
                                            )
                                        }
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private225::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "EnvironmentName" => {
                                            _serde::__private225::Ok(__Field::__field0)
                                        }
                                        "RepoURL" => _serde::__private225::Ok(__Field::__field1),
                                        "TargetRevision" => {
                                            _serde::__private225::Ok(__Field::__field2)
                                        }
                                        "LastCommitSHA" => {
                                            _serde::__private225::Ok(__Field::__field3)
                                        }
                                        _ => {
                                            _serde::__private225::Err(
                                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                                            )
                                        }
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private225::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"EnvironmentName" => {
                                            _serde::__private225::Ok(__Field::__field0)
                                        }
                                        b"RepoURL" => _serde::__private225::Ok(__Field::__field1),
                                        b"TargetRevision" => {
                                            _serde::__private225::Ok(__Field::__field2)
                                        }
                                        b"LastCommitSHA" => {
                                            _serde::__private225::Ok(__Field::__field3)
                                        }
                                        _ => {
                                            let __value = &_serde::__private225::from_utf8_lossy(
                                                __value,
                                            );
                                            _serde::__private225::Err(
                                                _serde::de::Error::unknown_variant(__value, VARIANTS),
                                            )
                                        }
                                    }
                                }
                            }
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private225::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private225::PhantomData<BuiltinConfigKey>,
                                lifetime: _serde::__private225::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = BuiltinConfigKey;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private225::Formatter,
                                ) -> _serde::__private225::fmt::Result {
                                    _serde::__private225::Formatter::write_str(
                                        __formatter,
                                        "enum BuiltinConfigKey",
                                    )
                                }
                                fn visit_enum<__A>(
                                    self,
                                    __data: __A,
                                ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::EnumAccess<'de>,
                                {
                                    match _serde::de::EnumAccess::variant(__data)? {
                                        (__Field::__field0, __variant) => {
                                            _serde::de::VariantAccess::unit_variant(__variant)?;
                                            _serde::__private225::Ok(BuiltinConfigKey::EnvironmentName)
                                        }
                                        (__Field::__field1, __variant) => {
                                            _serde::de::VariantAccess::unit_variant(__variant)?;
                                            _serde::__private225::Ok(BuiltinConfigKey::RepoURL)
                                        }
                                        (__Field::__field2, __variant) => {
                                            _serde::de::VariantAccess::unit_variant(__variant)?;
                                            _serde::__private225::Ok(BuiltinConfigKey::TargetRevision)
                                        }
                                        (__Field::__field3, __variant) => {
                                            _serde::de::VariantAccess::unit_variant(__variant)?;
                                            _serde::__private225::Ok(BuiltinConfigKey::LastCommitSHA)
                                        }
                                    }
                                }
                            }
                            #[doc(hidden)]
                            const VARIANTS: &'static [&'static str] = &[
                                "EnvironmentName",
                                "RepoURL",
                                "TargetRevision",
                                "LastCommitSHA",
                            ];
                            _serde::Deserializer::deserialize_enum(
                                __deserializer,
                                "BuiltinConfigKey",
                                VARIANTS,
                                __Visitor {
                                    marker: _serde::__private225::PhantomData::<
                                        BuiltinConfigKey,
                                    >,
                                    lifetime: _serde::__private225::PhantomData,
                                },
                            )
                        }
                    }
                };
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for BuiltinConfigKey {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for BuiltinConfigKey {
                    #[inline]
                    fn eq(&self, other: &BuiltinConfigKey) -> bool {
                        let __self_discr = ::core::intrinsics::discriminant_value(self);
                        let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                        __self_discr == __arg1_discr
                    }
                }
                impl BuiltinConfigKey {
                    pub fn from_key(key: &str) -> Option<Self> {
                        match key {
                            ".Sandcastle.EnvironmentName" => {
                                Some(BuiltinConfigKey::EnvironmentName)
                            }
                            ".Sandcastle.RepoURL" => Some(BuiltinConfigKey::RepoURL),
                            ".Sandcastle.TargetRevision" => {
                                Some(BuiltinConfigKey::TargetRevision)
                            }
                            ".Sandcastle.LastCommitSHA" => {
                                Some(BuiltinConfigKey::LastCommitSHA)
                            }
                            _ => None,
                        }
                    }
                }
                pub struct ConfigPath(String);
                #[automatically_derived]
                impl ::core::fmt::Debug for ConfigPath {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "ConfigPath",
                            &&self.0,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ConfigPath {
                    #[inline]
                    fn clone(&self) -> ConfigPath {
                        ConfigPath(::core::clone::Clone::clone(&self.0))
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ConfigPath {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ConfigPath {
                    #[inline]
                    fn eq(&self, other: &ConfigPath) -> bool {
                        self.0 == other.0
                    }
                }
                impl Deref for ConfigPath {
                    type Target = String;
                    fn deref(&self) -> &Self::Target {
                        &self.0
                    }
                }
                static VALIDATION_REGEX: OnceLock<Regex> = OnceLock::new();
                fn get_regex() -> &'static Regex {
                    VALIDATION_REGEX
                        .get_or_init(|| {
                            Regex::new(r#"^\.(?:Sandcastle|Custom)(?:\.[A-Za-z0-9]+)+$"#)
                                .expect("Failed to compile regex")
                        })
                }
                impl FromStr for ConfigPath {
                    type Err = SandcastleError;
                    fn from_str(s: &str) -> Result<Self, Self::Err> {
                        if !get_regex().is_match(s) {
                            return Err(SandcastleError::Service {
                                code: ServiceErrorCode::InvalidConfiguration,
                                message: "Invalid config path, must match .<Sandcastle|Custom>.<key>. Ex: .Sandcastle.EnvironmentName or .Custom.baseDomain"
                                    .to_string(),
                                reason: s.to_string(),
                                backtrace: Backtrace::capture(),
                            });
                        }
                        Ok(ConfigPath(s.to_string()))
                    }
                }
                /// Represents the sandcastle configuration from the application file
                /// found in the repository.
                pub struct SandcastleConfiguration {
                    pub custom: SandcastleCustomValues,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for SandcastleConfiguration {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "SandcastleConfiguration",
                            "custom",
                            &&self.custom,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for SandcastleConfiguration {
                    #[inline]
                    fn clone(&self) -> SandcastleConfiguration {
                        SandcastleConfiguration {
                            custom: ::core::clone::Clone::clone(&self.custom),
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications,
                    clippy::absolute_paths,
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl _serde::Serialize for SandcastleConfiguration {
                        fn serialize<__S>(
                            &self,
                            __serializer: __S,
                        ) -> _serde::__private225::Result<__S::Ok, __S::Error>
                        where
                            __S: _serde::Serializer,
                        {
                            let mut __serde_state = _serde::Serializer::serialize_struct(
                                __serializer,
                                "SandcastleConfiguration",
                                false as usize + 1,
                            )?;
                            _serde::ser::SerializeStruct::serialize_field(
                                &mut __serde_state,
                                "custom",
                                &self.custom,
                            )?;
                            _serde::ser::SerializeStruct::end(__serde_state)
                        }
                    }
                };
                #[doc(hidden)]
                #[allow(
                    non_upper_case_globals,
                    unused_attributes,
                    unused_qualifications,
                    clippy::absolute_paths,
                )]
                const _: () = {
                    #[allow(unused_extern_crates, clippy::useless_attribute)]
                    extern crate serde as _serde;
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for SandcastleConfiguration {
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private225::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            #[allow(non_camel_case_types)]
                            #[doc(hidden)]
                            enum __Field {
                                __field0,
                                __ignore,
                            }
                            #[doc(hidden)]
                            struct __FieldVisitor;
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                                type Value = __Field;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private225::Formatter,
                                ) -> _serde::__private225::fmt::Result {
                                    _serde::__private225::Formatter::write_str(
                                        __formatter,
                                        "field identifier",
                                    )
                                }
                                fn visit_u64<__E>(
                                    self,
                                    __value: u64,
                                ) -> _serde::__private225::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        0u64 => _serde::__private225::Ok(__Field::__field0),
                                        _ => _serde::__private225::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_str<__E>(
                                    self,
                                    __value: &str,
                                ) -> _serde::__private225::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        "custom" => _serde::__private225::Ok(__Field::__field0),
                                        _ => _serde::__private225::Ok(__Field::__ignore),
                                    }
                                }
                                fn visit_bytes<__E>(
                                    self,
                                    __value: &[u8],
                                ) -> _serde::__private225::Result<Self::Value, __E>
                                where
                                    __E: _serde::de::Error,
                                {
                                    match __value {
                                        b"custom" => _serde::__private225::Ok(__Field::__field0),
                                        _ => _serde::__private225::Ok(__Field::__ignore),
                                    }
                                }
                            }
                            #[automatically_derived]
                            impl<'de> _serde::Deserialize<'de> for __Field {
                                #[inline]
                                fn deserialize<__D>(
                                    __deserializer: __D,
                                ) -> _serde::__private225::Result<Self, __D::Error>
                                where
                                    __D: _serde::Deserializer<'de>,
                                {
                                    _serde::Deserializer::deserialize_identifier(
                                        __deserializer,
                                        __FieldVisitor,
                                    )
                                }
                            }
                            #[doc(hidden)]
                            struct __Visitor<'de> {
                                marker: _serde::__private225::PhantomData<
                                    SandcastleConfiguration,
                                >,
                                lifetime: _serde::__private225::PhantomData<&'de ()>,
                            }
                            #[automatically_derived]
                            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                                type Value = SandcastleConfiguration;
                                fn expecting(
                                    &self,
                                    __formatter: &mut _serde::__private225::Formatter,
                                ) -> _serde::__private225::fmt::Result {
                                    _serde::__private225::Formatter::write_str(
                                        __formatter,
                                        "struct SandcastleConfiguration",
                                    )
                                }
                                #[inline]
                                fn visit_seq<__A>(
                                    self,
                                    mut __seq: __A,
                                ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::SeqAccess<'de>,
                                {
                                    let __field0 = match _serde::de::SeqAccess::next_element::<
                                        SandcastleCustomValues,
                                    >(&mut __seq)? {
                                        _serde::__private225::Some(__value) => __value,
                                        _serde::__private225::None => {
                                            return _serde::__private225::Err(
                                                _serde::de::Error::invalid_length(
                                                    0usize,
                                                    &"struct SandcastleConfiguration with 1 element",
                                                ),
                                            );
                                        }
                                    };
                                    _serde::__private225::Ok(SandcastleConfiguration {
                                        custom: __field0,
                                    })
                                }
                                #[inline]
                                fn visit_map<__A>(
                                    self,
                                    mut __map: __A,
                                ) -> _serde::__private225::Result<Self::Value, __A::Error>
                                where
                                    __A: _serde::de::MapAccess<'de>,
                                {
                                    let mut __field0: _serde::__private225::Option<
                                        SandcastleCustomValues,
                                    > = _serde::__private225::None;
                                    while let _serde::__private225::Some(__key) = _serde::de::MapAccess::next_key::<
                                        __Field,
                                    >(&mut __map)? {
                                        match __key {
                                            __Field::__field0 => {
                                                if _serde::__private225::Option::is_some(&__field0) {
                                                    return _serde::__private225::Err(
                                                        <__A::Error as _serde::de::Error>::duplicate_field("custom"),
                                                    );
                                                }
                                                __field0 = _serde::__private225::Some(
                                                    _serde::de::MapAccess::next_value::<
                                                        SandcastleCustomValues,
                                                    >(&mut __map)?,
                                                );
                                            }
                                            _ => {
                                                let _ = _serde::de::MapAccess::next_value::<
                                                    _serde::de::IgnoredAny,
                                                >(&mut __map)?;
                                            }
                                        }
                                    }
                                    let __field0 = match __field0 {
                                        _serde::__private225::Some(__field0) => __field0,
                                        _serde::__private225::None => {
                                            _serde::__private225::de::missing_field("custom")?
                                        }
                                    };
                                    _serde::__private225::Ok(SandcastleConfiguration {
                                        custom: __field0,
                                    })
                                }
                            }
                            #[doc(hidden)]
                            const FIELDS: &'static [&'static str] = &["custom"];
                            _serde::Deserializer::deserialize_struct(
                                __deserializer,
                                "SandcastleConfiguration",
                                FIELDS,
                                __Visitor {
                                    marker: _serde::__private225::PhantomData::<
                                        SandcastleConfiguration,
                                    >,
                                    lifetime: _serde::__private225::PhantomData,
                                },
                            )
                        }
                    }
                };
                pub type SandcastleCustomValues = Value;
                impl SandcastleConfiguration {
                    pub fn from_string(string: &str) -> Result<Self, SandcastleError> {
                        let parts = string
                            .trim()
                            .split("---")
                            .filter_map(|s| {
                                if !s.is_empty() { Some(s.trim()) } else { None }
                            })
                            .collect::<Vec<&str>>();
                        match parts.first() {
                            Some(part) => {
                                let config = Self::from_yaml(part)?;
                                Ok(config)
                            }
                            None => {
                                return Err(SandcastleError::Service {
                                    code: ServiceErrorCode::InvalidConfiguration,
                                    message: "No configuration found in file".to_string(),
                                    reason: string.to_string(),
                                    backtrace: Backtrace::capture(),
                                });
                            }
                        }
                    }
                    fn from_yaml(yaml: &str) -> Result<Self, SandcastleError> {
                        let config: SandcastleConfiguration = serde_yaml::from_str(yaml)
                            .map_err(|e| SandcastleError::Service {
                                code: ServiceErrorCode::InvalidConfiguration,
                                message: e.to_string(),
                                reason: yaml.to_string(),
                                backtrace: Backtrace::capture(),
                            })?;
                        Ok(config)
                    }
                    pub fn get_custom_value(&self, path: &str) -> Option<String> {
                        let path_parts = path
                            .trim_start_matches(".Custom.")
                            .split(".")
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>();
                        let mut current = &self.custom;
                        for part in path_parts {
                            current = current.get(part.as_str())?;
                        }
                        current.as_str().map(|s| s.to_string())
                    }
                }
                mod tests {
                    use super::*;
                    extern crate test;
                    #[rustc_test_marker = "domain::environment::models::config::tests::test_get_custom_value"]
                    #[doc(hidden)]
                    pub const test_get_custom_value: test::TestDescAndFn = test::TestDescAndFn {
                        desc: test::TestDesc {
                            name: test::StaticTestName(
                                "domain::environment::models::config::tests::test_get_custom_value",
                            ),
                            ignore: false,
                            ignore_message: ::core::option::Option::None,
                            source_file: "crates/sandcastle-core/src/domain/environment/models/config.rs",
                            start_line: 131usize,
                            start_col: 8usize,
                            end_line: 131usize,
                            end_col: 29usize,
                            compile_fail: false,
                            no_run: false,
                            should_panic: test::ShouldPanic::No,
                            test_type: test::TestType::UnitTest,
                        },
                        testfn: test::StaticTestFn(
                            #[coverage(off)]
                            || test::assert_test_result(test_get_custom_value()),
                        ),
                    };
                    fn test_get_custom_value() {
                        let custom = r#"
        custom:
          baseDomain: sandcastle.dev
          whatever:
            key: value
        "#;
                        let config = SandcastleConfiguration::from_yaml(custom).unwrap();
                        let value = config.get_custom_value(".Custom.baseDomain");
                        match (&value, &Some("sandcastle.dev".to_string())) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        let value = config
                            .get_custom_value(".Custom.baseDomain.subDomain");
                        match (&value, &None) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        let value = config
                            .get_custom_value(".Custom.baseDomain.subDomain.subDomain");
                        match (&value, &None) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        let value = config.get_custom_value(".Custom.whatever.key");
                        match (&value, &Some("value".to_string())) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                    }
                    extern crate test;
                    #[rustc_test_marker = "domain::environment::models::config::tests::test_from_string"]
                    #[doc(hidden)]
                    pub const test_from_string: test::TestDescAndFn = test::TestDescAndFn {
                        desc: test::TestDesc {
                            name: test::StaticTestName(
                                "domain::environment::models::config::tests::test_from_string",
                            ),
                            ignore: false,
                            ignore_message: ::core::option::Option::None,
                            source_file: "crates/sandcastle-core/src/domain/environment/models/config.rs",
                            start_line: 153usize,
                            start_col: 8usize,
                            end_line: 153usize,
                            end_col: 24usize,
                            compile_fail: false,
                            no_run: false,
                            should_panic: test::ShouldPanic::No,
                            test_type: test::TestType::UnitTest,
                        },
                        testfn: test::StaticTestFn(
                            #[coverage(off)]
                            || test::assert_test_result(test_from_string()),
                        ),
                    };
                    fn test_from_string() {
                        let application_yaml = "---\ncustom:\n  baseDomain: \"sandcastle.dev\"\n  whatever:\n    key: \"value\"\nglobal:\n  images:\n    frontend:\n      # Image hint to find the images in:\n      # - github checks\n      # - container registry\n      hint: \"ghcr.io/mmoreiradj/sandcastle-monorepo-test/frontend\"\n---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nmetadata:\n  name: frontend-{{ .Sandcastle.EnvironmentName }}\n  namespace: argocd\nspec:\n  project: default\n  source:\n    path: \"charts/frontend\" \n    repoURL: \"{{ .Sandcastle.RepoURL }}\"\n    targetRevision: \"{{ .Sandcastle.TargetRevision }}\"\n    helm:\n      valuesObject:\n        ingress:\n          enabled: true\n          className: \"traefik\"\n          hosts:\n            - host: \"frontend-{{ .Sandcastle.EnvironmentName }}.{{ .Custom.baseDomain }}\"\n              paths:\n                - path: /\n                  pathType: ImplementationSpecific\n        image:\n          tag: \"{{ .Sandcastle.LastCommitSHA }}\"\n  destination:\n    server: \"https://kubernetes.default.svc\"\n    namespace: \"{{ .Sandcastle.EnvironmentName }}\"\n";
                        let config = SandcastleConfiguration::from_string(
                            application_yaml,
                        );
                        if !config.is_ok() {
                            ::core::panicking::panic("assertion failed: config.is_ok()")
                        }
                        let config = config.unwrap();
                        match (
                            &config.get_custom_value(".Custom.baseDomain"),
                            &Some("sandcastle.dev".to_string()),
                        ) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                        match (
                            &config.get_custom_value(".Custom.whatever.key"),
                            &Some("value".to_string()),
                        ) {
                            (left_val, right_val) => {
                                if !(*left_val == *right_val) {
                                    let kind = ::core::panicking::AssertKind::Eq;
                                    ::core::panicking::assert_failed(
                                        kind,
                                        &*left_val,
                                        &*right_val,
                                        ::core::option::Option::None,
                                    );
                                }
                            }
                        };
                    }
                }
            }
            mod environment {
                use std::{backtrace::Backtrace, str::FromStr};
                use crate::{
                    Result, application::ApplicationState,
                    domain::environment::{
                        models::{DownloadFileRequest, FetchPRLastCommitSHARequest},
                        services::GitHub,
                    },
                    error::ServiceErrorCode,
                };
                use crate::{
                    domain::environment::{
                        models::config::{
                            BuiltinConfigKey, ConfigPath, SandcastleConfiguration,
                        },
                        ports::{Reconcile, VCSService},
                        services::{GitOpsPlatform, VCS},
                    },
                    domain::repositories::ports::RepositoryConfigurationService,
                    error::SandcastleError,
                };
                use octocrab::{
                    Octocrab,
                    models::{
                        Repository, webhook_events::{WebhookEvent, WebhookEventPayload},
                    },
                };
                use regex::Regex;
                use serde_yaml::Value;
                pub struct ReconcileContext {
                    /// The ID of the reconcile context
                    pub id: String,
                    /// The VCS context
                    pub vcs: VcsContext,
                    /// The VCS service
                    pub vcs_service: VCS,
                    /// The GitOps platform service
                    pub gitops_platform_service: GitOpsPlatform,
                    /// Sandcastle configuration
                    pub config: SandcastleConfiguration,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ReconcileContext {
                    #[inline]
                    fn clone(&self) -> ReconcileContext {
                        ReconcileContext {
                            id: ::core::clone::Clone::clone(&self.id),
                            vcs: ::core::clone::Clone::clone(&self.vcs),
                            vcs_service: ::core::clone::Clone::clone(&self.vcs_service),
                            gitops_platform_service: ::core::clone::Clone::clone(
                                &self.gitops_platform_service,
                            ),
                            config: ::core::clone::Clone::clone(&self.config),
                        }
                    }
                }
                impl ReconcileContext {
                    pub fn template(
                        &self,
                        template: &str,
                    ) -> Result<String, SandcastleError> {
                        let mut result = template.to_string();
                        let r = Regex::new(r#"\{\{ (.*?) \}\}"#).unwrap();
                        let replacements: Vec<(String, String)> = r
                            .captures_iter(&result)
                            .map(|capture| -> Result<(String, String), SandcastleError> {
                                let full_match = capture
                                    .get(0)
                                    .unwrap()
                                    .as_str()
                                    .to_string();
                                let path = capture.get(1).unwrap().as_str().trim();
                                let value = self
                                    .get_config_value(path)
                                    .ok_or_else(|| SandcastleError::Service {
                                        code: ServiceErrorCode::InvalidConfiguration,
                                        message: ::alloc::__export::must_use({
                                            ::alloc::fmt::format(
                                                format_args!("Value not found for path: {0}", path),
                                            )
                                        }),
                                        reason: path.to_string(),
                                        backtrace: Backtrace::capture(),
                                    })?;
                                Ok((full_match, value))
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        for (pattern, replacement) in replacements {
                            result = result.replace(&pattern, &replacement);
                        }
                        Ok(result)
                    }
                    pub async fn from_github_event(
                        id: String,
                        event: WebhookEvent,
                        payload: WebhookEventPayload,
                        state: ApplicationState,
                    ) -> Result<Option<Self>> {
                        match payload {
                            WebhookEventPayload::IssueComment(payload) => {
                                let comment_body = if let Some(body) = payload.comment.body
                                {
                                    body
                                } else {
                                    return Ok(None);
                                };
                                let repository = event.repository.unwrap();
                                let repository_configuration = match state
                                    .repository_configuration_service
                                    .get_repository_configuration(repository.url.as_ref())
                                    .await?
                                {
                                    Some(repository_configuration) => repository_configuration,
                                    None => return Ok(None),
                                };
                                let vcs_service = VCS::try_from(repository_configuration)?;
                                let last_commit_sha = vcs_service
                                    .fetch_pr_last_commit_sha(FetchPRLastCommitSHARequest {
                                        repository_id: (*repository.id),
                                        pr_number: payload.issue.number,
                                    })
                                    .await?;
                                let refs_url = repository.git_refs_url.clone().unwrap();
                                let config_url = refs_url
                                    .to_string()
                                    .replace("{/sha}", &last_commit_sha);
                                let configuration_file_content = vcs_service
                                    .download_file(DownloadFileRequest {
                                        repository_id: (*repository.id),
                                        path: config_url,
                                        r#ref: last_commit_sha.clone(),
                                        content_type: "application/yaml".to_string(),
                                    })
                                    .await?;
                                let config = SandcastleConfiguration::from_string(
                                    &configuration_file_content,
                                )?;
                                Ok(
                                    Some(Self {
                                        id,
                                        vcs: VcsContext {
                                            comment: CommentContext {
                                                body: comment_body,
                                            },
                                            repository: RepositoryContext::from(&repository),
                                            pull_request: PullRequestContext {
                                                number: payload.issue.number,
                                                title: payload.issue.title.clone(),
                                                last_commit_sha,
                                            },
                                        },
                                        vcs_service: VCS::GitHub(
                                            crate::domain::environment::services::GitHub::from(
                                                octocrab::Octocrab::default(),
                                            ),
                                        ),
                                        gitops_platform_service: GitOpsPlatform::ArgoCD(
                                            crate::domain::environment::services::ArgoCD,
                                        ),
                                        config,
                                    }),
                                )
                            }
                            _ => Ok(None),
                        }
                    }
                    fn get_config_value(&self, path: &str) -> Option<String> {
                        if path.starts_with(".Sandcastle.") {
                            self.get_builtin_config_value(path)
                        } else {
                            self.config.get_custom_value(path)
                        }
                    }
                    fn get_builtin_config_value(&self, key: &str) -> Option<String> {
                        let key = BuiltinConfigKey::from_key(key)?;
                        match key {
                            BuiltinConfigKey::EnvironmentName => {
                                Some(self.vcs.repository.name.clone())
                            }
                            BuiltinConfigKey::RepoURL => {
                                Some(self.vcs.repository.url.clone())
                            }
                            BuiltinConfigKey::TargetRevision => {
                                Some(self.vcs.pull_request.last_commit_sha.clone())
                            }
                            BuiltinConfigKey::LastCommitSHA => {
                                Some(self.vcs.pull_request.last_commit_sha.clone())
                            }
                        }
                    }
                }
                pub struct VcsContext {
                    pub repository: RepositoryContext,
                    pub pull_request: PullRequestContext,
                    pub comment: CommentContext,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for VcsContext {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "VcsContext",
                            "repository",
                            &self.repository,
                            "pull_request",
                            &self.pull_request,
                            "comment",
                            &&self.comment,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for VcsContext {
                    #[inline]
                    fn clone(&self) -> VcsContext {
                        VcsContext {
                            repository: ::core::clone::Clone::clone(&self.repository),
                            pull_request: ::core::clone::Clone::clone(
                                &self.pull_request,
                            ),
                            comment: ::core::clone::Clone::clone(&self.comment),
                        }
                    }
                }
                pub struct RepositoryContext {
                    /// The name of the repository
                    pub name: String,
                    /// Whether the repository is private
                    pub private: bool,
                    /// The base URI of the repository
                    pub url: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for RepositoryContext {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "RepositoryContext",
                            "name",
                            &self.name,
                            "private",
                            &self.private,
                            "url",
                            &&self.url,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for RepositoryContext {
                    #[inline]
                    fn clone(&self) -> RepositoryContext {
                        RepositoryContext {
                            name: ::core::clone::Clone::clone(&self.name),
                            private: ::core::clone::Clone::clone(&self.private),
                            url: ::core::clone::Clone::clone(&self.url),
                        }
                    }
                }
                impl From<&Repository> for RepositoryContext {
                    fn from(value: &Repository) -> Self {
                        Self {
                            name: value.name.clone(),
                            private: value.private.unwrap_or(false),
                            url: value.url.to_string(),
                        }
                    }
                }
                pub struct PullRequestContext {
                    pub number: u64,
                    pub title: String,
                    pub last_commit_sha: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for PullRequestContext {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field3_finish(
                            f,
                            "PullRequestContext",
                            "number",
                            &self.number,
                            "title",
                            &self.title,
                            "last_commit_sha",
                            &&self.last_commit_sha,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for PullRequestContext {
                    #[inline]
                    fn clone(&self) -> PullRequestContext {
                        PullRequestContext {
                            number: ::core::clone::Clone::clone(&self.number),
                            title: ::core::clone::Clone::clone(&self.title),
                            last_commit_sha: ::core::clone::Clone::clone(
                                &self.last_commit_sha,
                            ),
                        }
                    }
                }
                pub struct CommentContext {
                    pub body: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for CommentContext {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "CommentContext",
                            "body",
                            &&self.body,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for CommentContext {
                    #[inline]
                    fn clone(&self) -> CommentContext {
                        CommentContext {
                            body: ::core::clone::Clone::clone(&self.body),
                        }
                    }
                }
                /// Action to create or update a GitOps Application
                pub struct CreateOrUpdateArgocdApplicationAction {
                    /// The GitOps File
                    pub application: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for CreateOrUpdateArgocdApplicationAction {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "CreateOrUpdateArgocdApplicationAction",
                            "application",
                            &&self.application,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for CreateOrUpdateArgocdApplicationAction {
                    #[inline]
                    fn clone(&self) -> CreateOrUpdateArgocdApplicationAction {
                        CreateOrUpdateArgocdApplicationAction {
                            application: ::core::clone::Clone::clone(&self.application),
                        }
                    }
                }
                /// Action to delete an Argocd Application
                pub struct DeleteArgocdApplicationAction {
                    /// The GitOps File
                    pub application: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for DeleteArgocdApplicationAction {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "DeleteArgocdApplicationAction",
                            "application",
                            &&self.application,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DeleteArgocdApplicationAction {
                    #[inline]
                    fn clone(&self) -> DeleteArgocdApplicationAction {
                        DeleteArgocdApplicationAction {
                            application: ::core::clone::Clone::clone(&self.application),
                        }
                    }
                }
                pub enum ReconcileActions {
                    CreateOrUpdateArgocdApplication(
                        CreateOrUpdateArgocdApplicationAction,
                    ),
                    DeleteArgocdApplication(DeleteArgocdApplicationAction),
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for ReconcileActions {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        match self {
                            ReconcileActions::CreateOrUpdateArgocdApplication(
                                __self_0,
                            ) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "CreateOrUpdateArgocdApplication",
                                    &__self_0,
                                )
                            }
                            ReconcileActions::DeleteArgocdApplication(__self_0) => {
                                ::core::fmt::Formatter::debug_tuple_field1_finish(
                                    f,
                                    "DeleteArgocdApplication",
                                    &__self_0,
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ReconcileActions {
                    #[inline]
                    fn clone(&self) -> ReconcileActions {
                        match self {
                            ReconcileActions::CreateOrUpdateArgocdApplication(
                                __self_0,
                            ) => {
                                ReconcileActions::CreateOrUpdateArgocdApplication(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                            ReconcileActions::DeleteArgocdApplication(__self_0) => {
                                ReconcileActions::DeleteArgocdApplication(
                                    ::core::clone::Clone::clone(__self_0),
                                )
                            }
                        }
                    }
                }
                impl ReconcileActions {
                    pub async fn reconcile(
                        &self,
                        context: ReconcileContext,
                    ) -> Result<(), SandcastleError> {
                        match self {
                            ReconcileActions::CreateOrUpdateArgocdApplication(action) => {
                                action.reconcile(context).await
                            }
                            ReconcileActions::DeleteArgocdApplication(action) => {
                                action.reconcile(context).await
                            }
                        }
                    }
                }
                mod tests {
                    use crate::domain::environment::services::ArgoCD;
                    use super::*;
                    async fn test_context() -> ReconcileContext {
                        let config = SandcastleConfiguration::from_string(
                                "---\ncustom:\n  baseDomain: \"sandcastle.dev\"\n  whatever:\n    key: \"value\"\nglobal:\n  images:\n    frontend:\n      # Image hint to find the images in:\n      # - github checks\n      # - container registry\n      hint: \"ghcr.io/mmoreiradj/sandcastle-monorepo-test/frontend\"\n---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nmetadata:\n  name: frontend-{{ .Sandcastle.EnvironmentName }}\n  namespace: argocd\nspec:\n  project: default\n  source:\n    path: \"charts/frontend\" \n    repoURL: \"{{ .Sandcastle.RepoURL }}\"\n    targetRevision: \"{{ .Sandcastle.TargetRevision }}\"\n    helm:\n      valuesObject:\n        ingress:\n          enabled: true\n          className: \"traefik\"\n          hosts:\n            - host: \"frontend-{{ .Sandcastle.EnvironmentName }}.{{ .Custom.baseDomain }}\"\n              paths:\n                - path: /\n                  pathType: ImplementationSpecific\n        image:\n          tag: \"{{ .Sandcastle.LastCommitSHA }}\"\n  destination:\n    server: \"https://kubernetes.default.svc\"\n    namespace: \"{{ .Sandcastle.EnvironmentName }}\"\n",
                            )
                            .unwrap();
                        let context = ReconcileContext {
                            id: "1".to_string(),
                            vcs: VcsContext {
                                repository: RepositoryContext {
                                    name: "test".to_string(),
                                    private: false,
                                    url: "https://github.com/test/test".to_string(),
                                },
                                pull_request: PullRequestContext {
                                    number: 1,
                                    title: "test".to_string(),
                                    last_commit_sha: "test".to_string(),
                                },
                                comment: CommentContext {
                                    body: "test".to_string(),
                                },
                            },
                            vcs_service: VCS::GitHub(GitHub::from(Octocrab::default())),
                            gitops_platform_service: GitOpsPlatform::ArgoCD(ArgoCD),
                            config: config,
                        };
                        context
                    }
                    extern crate test;
                    #[rustc_test_marker = "domain::environment::models::environment::tests::test_small_template"]
                    #[doc(hidden)]
                    pub const test_small_template: test::TestDescAndFn = test::TestDescAndFn {
                        desc: test::TestDesc {
                            name: test::StaticTestName(
                                "domain::environment::models::environment::tests::test_small_template",
                            ),
                            ignore: false,
                            ignore_message: ::core::option::Option::None,
                            source_file: "crates/sandcastle-core/src/domain/environment/models/environment.rs",
                            start_line: 264usize,
                            start_col: 14usize,
                            end_line: 264usize,
                            end_col: 33usize,
                            compile_fail: false,
                            no_run: false,
                            should_panic: test::ShouldPanic::No,
                            test_type: test::TestType::UnitTest,
                        },
                        testfn: test::StaticTestFn(
                            #[coverage(off)]
                            || test::assert_test_result(test_small_template()),
                        ),
                    };
                    fn test_small_template() {
                        let body = async {
                            let template = "{{ .Sandcastle.EnvironmentName }}";
                            let context = test_context().await;
                            let result = context.template(template).unwrap();
                            match (&result, &"test") {
                                (left_val, right_val) => {
                                    if !(*left_val == *right_val) {
                                        let kind = ::core::panicking::AssertKind::Eq;
                                        ::core::panicking::assert_failed(
                                            kind,
                                            &*left_val,
                                            &*right_val,
                                            ::core::option::Option::None,
                                        );
                                    }
                                }
                            };
                        };
                        let mut body = body;
                        #[allow(unused_mut)]
                        let mut body = unsafe {
                            ::tokio::macros::support::Pin::new_unchecked(&mut body)
                        };
                        let body: ::core::pin::Pin<
                            &mut dyn ::core::future::Future<Output = ()>,
                        > = body;
                        #[allow(
                            clippy::expect_used,
                            clippy::diverging_sub_expression,
                            clippy::needless_return
                        )]
                        {
                            return tokio::runtime::Builder::new_current_thread()
                                .enable_all()
                                .build()
                                .expect("Failed building the Runtime")
                                .block_on(body);
                        }
                    }
                    extern crate test;
                    #[rustc_test_marker = "domain::environment::models::environment::tests::test_large_template"]
                    #[doc(hidden)]
                    pub const test_large_template: test::TestDescAndFn = test::TestDescAndFn {
                        desc: test::TestDesc {
                            name: test::StaticTestName(
                                "domain::environment::models::environment::tests::test_large_template",
                            ),
                            ignore: false,
                            ignore_message: ::core::option::Option::None,
                            source_file: "crates/sandcastle-core/src/domain/environment/models/environment.rs",
                            start_line: 272usize,
                            start_col: 14usize,
                            end_line: 272usize,
                            end_col: 33usize,
                            compile_fail: false,
                            no_run: false,
                            should_panic: test::ShouldPanic::No,
                            test_type: test::TestType::UnitTest,
                        },
                        testfn: test::StaticTestFn(
                            #[coverage(off)]
                            || test::assert_test_result(test_large_template()),
                        ),
                    };
                    fn test_large_template() {
                        let body = async {
                            let template = "---\ncustom:\n  baseDomain: \"sandcastle.dev\"\n  whatever:\n    key: \"value\"\nglobal:\n  images:\n    frontend:\n      # Image hint to find the images in:\n      # - github checks\n      # - container registry\n      hint: \"ghcr.io/mmoreiradj/sandcastle-monorepo-test/frontend\"\n---\napiVersion: argoproj.io/v1alpha1\nkind: Application\nmetadata:\n  name: frontend-{{ .Sandcastle.EnvironmentName }}\n  namespace: argocd\nspec:\n  project: default\n  source:\n    path: \"charts/frontend\" \n    repoURL: \"{{ .Sandcastle.RepoURL }}\"\n    targetRevision: \"{{ .Sandcastle.TargetRevision }}\"\n    helm:\n      valuesObject:\n        ingress:\n          enabled: true\n          className: \"traefik\"\n          hosts:\n            - host: \"frontend-{{ .Sandcastle.EnvironmentName }}.{{ .Custom.baseDomain }}\"\n              paths:\n                - path: /\n                  pathType: ImplementationSpecific\n        image:\n          tag: \"{{ .Sandcastle.LastCommitSHA }}\"\n  destination:\n    server: \"https://kubernetes.default.svc\"\n    namespace: \"{{ .Sandcastle.EnvironmentName }}\"\n";
                            let context = test_context().await;
                            let result = context.template(template).unwrap();
                            ::insta::_macro_support::assert_snapshot(
                                    (
                                        ::insta::_macro_support::AutoName,
                                        #[allow(clippy::redundant_closure_call)]
                                        (|v| ::alloc::__export::must_use({
                                            ::alloc::fmt::format(format_args!("{0}", v))
                                        }))(&result)
                                            .as_str(),
                                    )
                                        .into(),
                                    {
                                        use ::insta::_macro_support::{env, option_env};
                                        const WORKSPACE_ROOT: ::insta::_macro_support::Workspace = if let Some(
                                            root,
                                        ) = ::core::option::Option::None::<&'static str> {
                                            ::insta::_macro_support::Workspace::UseAsIs(root)
                                        } else {
                                            ::insta::_macro_support::Workspace::DetectWithCargo(
                                                "/home/mmoreiradj/projects/mmoreiradj/sandcastle/crates/sandcastle-core",
                                            )
                                        };
                                        ::insta::_macro_support::get_cargo_workspace(WORKSPACE_ROOT)
                                    }
                                        .as_path(),
                                    {
                                        fn f() {}
                                        fn type_name_of_val<T>(_: T) -> &'static str {
                                            ::insta::_macro_support::any::type_name::<T>()
                                        }
                                        let mut name = type_name_of_val(f)
                                            .strip_suffix("::f")
                                            .unwrap_or("");
                                        while let Some(rest) = name.strip_suffix("::{{closure}}") {
                                            name = rest;
                                        }
                                        name
                                    },
                                    "sandcastle_core::domain::environment::models::environment::tests",
                                    "crates/sandcastle-core/src/domain/environment/models/environment.rs",
                                    276u32,
                                    "result",
                                )
                                .unwrap();
                        };
                        let mut body = body;
                        #[allow(unused_mut)]
                        let mut body = unsafe {
                            ::tokio::macros::support::Pin::new_unchecked(&mut body)
                        };
                        let body: ::core::pin::Pin<
                            &mut dyn ::core::future::Future<Output = ()>,
                        > = body;
                        #[allow(
                            clippy::expect_used,
                            clippy::diverging_sub_expression,
                            clippy::needless_return
                        )]
                        {
                            return tokio::runtime::Builder::new_current_thread()
                                .enable_all()
                                .build()
                                .expect("Failed building the Runtime")
                                .block_on(body);
                        }
                    }
                }
            }
            mod gitops {}
            mod vcs {
                pub struct DownloadFileRequest {
                    pub repository_id: u64,
                    pub path: String,
                    pub content_type: String,
                    pub r#ref: String,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for DownloadFileRequest {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field4_finish(
                            f,
                            "DownloadFileRequest",
                            "repository_id",
                            &self.repository_id,
                            "path",
                            &self.path,
                            "content_type",
                            &self.content_type,
                            "ref",
                            &&self.r#ref,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for DownloadFileRequest {
                    #[inline]
                    fn clone(&self) -> DownloadFileRequest {
                        DownloadFileRequest {
                            repository_id: ::core::clone::Clone::clone(
                                &self.repository_id,
                            ),
                            path: ::core::clone::Clone::clone(&self.path),
                            content_type: ::core::clone::Clone::clone(
                                &self.content_type,
                            ),
                            r#ref: ::core::clone::Clone::clone(&self.r#ref),
                        }
                    }
                }
                pub struct FetchPRLastCommitSHARequest {
                    pub repository_id: u64,
                    pub pr_number: u64,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for FetchPRLastCommitSHARequest {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field2_finish(
                            f,
                            "FetchPRLastCommitSHARequest",
                            "repository_id",
                            &self.repository_id,
                            "pr_number",
                            &&self.pr_number,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for FetchPRLastCommitSHARequest {
                    #[inline]
                    fn clone(&self) -> FetchPRLastCommitSHARequest {
                        FetchPRLastCommitSHARequest {
                            repository_id: ::core::clone::Clone::clone(
                                &self.repository_id,
                            ),
                            pr_number: ::core::clone::Clone::clone(&self.pr_number),
                        }
                    }
                }
            }
            pub use environment::*;
            pub use vcs::*;
        }
        pub mod ports {
            use async_trait::async_trait;
            use enum_dispatch::enum_dispatch;
            use mockall::mock;
            use crate::{
                domain::environment::models::{
                    DownloadFileRequest, FetchPRLastCommitSHARequest, ReconcileContext,
                },
                error::SandcastleError,
            };
            /// Reconcile the environment
            pub trait Reconcile: Clone + Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn reconcile<'life0, 'async_trait>(
                    &'life0 self,
                    context: ReconcileContext,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<(), SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
            }
            #[allow(non_snake_case)]
            #[allow(missing_docs)]
            pub mod __mock_MockVCSService {
                use super::*;
                #[allow(missing_docs)]
                #[allow(clippy::too_many_arguments, clippy::indexing_slicing)]
                pub mod __download_file {
                    use super::*;
                    use ::mockall::CaseTreeExt;
                    use ::std::{
                        boxed::Box, mem, ops::{DerefMut, Range},
                        sync::Mutex, vec::Vec,
                    };
                    #[allow(clippy::unused_unit)]
                    enum Rfunc {
                        Default,
                        Expired,
                        Mut(
                            Box<
                                dyn FnMut(
                                    DownloadFileRequest,
                                ) -> Result<String, SandcastleError> + Send,
                            >,
                        ),
                        MutSt(
                            ::mockall::Fragile<
                                Box<
                                    dyn FnMut(
                                        DownloadFileRequest,
                                    ) -> Result<String, SandcastleError>,
                                >,
                            >,
                        ),
                        Once(
                            Box<
                                dyn FnOnce(
                                    DownloadFileRequest,
                                ) -> Result<String, SandcastleError> + Send,
                            >,
                        ),
                        OnceSt(
                            ::mockall::Fragile<
                                Box<
                                    dyn FnOnce(
                                        DownloadFileRequest,
                                    ) -> Result<String, SandcastleError>,
                                >,
                            >,
                        ),
                        _Phantom(Box<dyn Fn() + Send>),
                    }
                    impl Rfunc {
                        fn call_mut(
                            &mut self,
                            request: DownloadFileRequest,
                        ) -> std::result::Result<
                            Result<String, SandcastleError>,
                            &'static str,
                        > {
                            match self {
                                Rfunc::Default => {
                                    use ::mockall::ReturnDefault;
                                    ::mockall::DefaultReturner::<
                                        Result<String, SandcastleError>,
                                    >::return_default()
                                }
                                Rfunc::Expired => {
                                    Err("called twice, but it returns by move")
                                }
                                Rfunc::Mut(__mockall_f) => {
                                    ::std::result::Result::Ok(__mockall_f(request))
                                }
                                Rfunc::MutSt(__mockall_f) => {
                                    ::std::result::Result::Ok((__mockall_f.get_mut())(request))
                                }
                                Rfunc::Once(_) => {
                                    if let Rfunc::Once(mut __mockall_f) = mem::replace(
                                        self,
                                        Rfunc::Expired,
                                    ) {
                                        ::std::result::Result::Ok(__mockall_f(request))
                                    } else {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                                Rfunc::OnceSt(_) => {
                                    if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                                        self,
                                        Rfunc::Expired,
                                    ) {
                                        ::std::result::Result::Ok(
                                            (__mockall_f.into_inner())(request),
                                        )
                                    } else {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                                Rfunc::_Phantom(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    impl std::default::Default for Rfunc {
                        fn default() -> Self {
                            Rfunc::Default
                        }
                    }
                    enum Matcher {
                        Always,
                        Func(Box<dyn Fn(&DownloadFileRequest) -> bool + Send>),
                        FuncSt(
                            ::mockall::Fragile<Box<dyn Fn(&DownloadFileRequest) -> bool>>,
                        ),
                        Pred(
                            Box<
                                (Box<dyn ::mockall::Predicate<DownloadFileRequest> + Send>,),
                            >,
                        ),
                        _Phantom(Box<dyn Fn() + Send>),
                    }
                    impl Matcher {
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(&self, request: &DownloadFileRequest) -> bool {
                            match self {
                                Matcher::Always => true,
                                Matcher::Func(__mockall_f) => __mockall_f(request),
                                Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(request),
                                Matcher::Pred(__mockall_pred) => {
                                    [__mockall_pred.0.eval(request)]
                                        .iter()
                                        .all(|__mockall_x| *__mockall_x)
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    impl Default for Matcher {
                        #[allow(unused_variables)]
                        fn default() -> Self {
                            Matcher::Always
                        }
                    }
                    impl ::std::fmt::Display for Matcher {
                        fn fmt(
                            &self,
                            __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
                        ) -> ::std::fmt::Result {
                            match self {
                                Matcher::Always => {
                                    __mockall_fmt.write_fmt(format_args!("<anything>"))
                                }
                                Matcher::Func(_) => {
                                    __mockall_fmt.write_fmt(format_args!("<function>"))
                                }
                                Matcher::FuncSt(_) => {
                                    __mockall_fmt
                                        .write_fmt(format_args!("<single threaded function>"))
                                }
                                Matcher::Pred(__mockall_p) => {
                                    __mockall_fmt.write_fmt(format_args!("{0}", __mockall_p.0))
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    /// Holds the stuff that is independent of the output type
                    struct Common {
                        matcher: Mutex<Matcher>,
                        seq_handle: Option<::mockall::SeqHandle>,
                        times: ::mockall::Times,
                    }
                    impl std::default::Default for Common {
                        fn default() -> Self {
                            Common {
                                matcher: Mutex::new(Matcher::default()),
                                seq_handle: None,
                                times: ::mockall::Times::default(),
                            }
                        }
                    }
                    impl Common {
                        fn call(&self, desc: &str) {
                            self.times
                                .call()
                                .unwrap_or_else(|m| {
                                    let desc = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("{0}", self.matcher.lock().unwrap()),
                                        )
                                    });
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "{0}: Expectation({1}) {2}",
                                                "MockVCSService::download_file",
                                                desc,
                                                m,
                                            ),
                                        );
                                    };
                                });
                            self.verify_sequence(desc);
                            if ::mockall::ExpectedCalls::TooFew
                                != self.times.is_satisfied()
                            {
                                self.satisfy_sequence()
                            }
                        }
                        fn in_sequence(
                            &mut self,
                            __mockall_seq: &mut ::mockall::Sequence,
                        ) -> &mut Self {
                            if !self.times.is_exact() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Only Expectations with an exact call count have sequences",
                                        ),
                                    );
                                }
                            }
                            self.seq_handle = Some(__mockall_seq.next_handle());
                            self
                        }
                        fn is_done(&self) -> bool {
                            self.times.is_done()
                        }
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(&self, request: &DownloadFileRequest) -> bool {
                            self.matcher.lock().unwrap().matches(request)
                        }
                        /// Forbid this expectation from ever being called.
                        fn never(&mut self) {
                            self.times.never();
                        }
                        fn satisfy_sequence(&self) {
                            if let Some(__mockall_handle) = &self.seq_handle {
                                __mockall_handle.satisfy()
                            }
                        }
                        /// Expect this expectation to be called any number of times
                        /// contained with the given range.
                        fn times<MockallR>(&mut self, __mockall_r: MockallR)
                        where
                            MockallR: Into<::mockall::TimesRange>,
                        {
                            self.times.times(__mockall_r)
                        }
                        fn with<
                            MockallMatcher0: ::mockall::Predicate<DownloadFileRequest>
                                + Send + 'static,
                        >(&mut self, request: MockallMatcher0) {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::Pred(
                                Box::new((Box::new(request),)),
                            );
                        }
                        fn withf<MockallF>(&mut self, __mockall_f: MockallF)
                        where
                            MockallF: Fn(&DownloadFileRequest) -> bool + Send + 'static,
                        {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::Func(
                                Box::new(__mockall_f),
                            );
                        }
                        fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
                        where
                            MockallF: Fn(&DownloadFileRequest) -> bool + 'static,
                        {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::FuncSt(
                                ::mockall::Fragile::new(Box::new(__mockall_f)),
                            );
                        }
                        fn verify_sequence(&self, desc: &str) {
                            if let Some(__mockall_handle) = &self.seq_handle {
                                __mockall_handle.verify(desc)
                            }
                        }
                    }
                    impl Drop for Common {
                        fn drop(&mut self) {
                            if !::std::thread::panicking() {
                                let desc = ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("{0}", self.matcher.lock().unwrap()),
                                    )
                                });
                                match self.times.is_satisfied() {
                                    ::mockall::ExpectedCalls::TooFew => {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                                    "MockVCSService::download_file",
                                                    desc,
                                                    self.times.count(),
                                                    self.times.minimum(),
                                                ),
                                            );
                                        };
                                    }
                                    ::mockall::ExpectedCalls::TooMany => {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                                    "MockVCSService::download_file",
                                                    desc,
                                                    self.times.count(),
                                                    self.times.maximum(),
                                                ),
                                            );
                                        };
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    /// Expectation type for methods that return a `'static` type.
                    /// This is the type returned by the `expect_*` methods.
                    pub(in super::super) struct Expectation {
                        common: Common,
                        rfunc: Mutex<Rfunc>,
                    }
                    #[allow(clippy::unused_unit)]
                    impl Expectation {
                        /// Call this [`Expectation`] as if it were the real method.
                        #[doc(hidden)]
                        pub(in super::super) fn call(
                            &self,
                            request: DownloadFileRequest,
                        ) -> Result<String, SandcastleError> {
                            use ::mockall::{ViaDebug, ViaNothing};
                            self.common
                                .call(
                                    &::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "MockVCSService::download_file({0:?})",
                                                (&&::mockall::ArgPrinter(&request)).debug_string(),
                                            ),
                                        )
                                    }),
                                );
                            self.rfunc
                                .lock()
                                .unwrap()
                                .call_mut(request)
                                .unwrap_or_else(|message| {
                                    let desc = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("{0}", self.common.matcher.lock().unwrap()),
                                        )
                                    });
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "{0}: Expectation({1}) {2}",
                                                "MockVCSService::download_file",
                                                desc,
                                                message,
                                            ),
                                        );
                                    };
                                })
                        }
                        /// Return a constant value from the `Expectation`
                        ///
                        /// The output type must be `Clone`.  The compiler can't always
                        /// infer the proper type to use with this method; you will
                        /// usually need to specify it explicitly.  i.e.
                        /// `return_const(42i32)` instead of `return_const(42)`.
                        #[allow(unused_variables)]
                        pub(in super::super) fn return_const<MockallOutput>(
                            &mut self,
                            __mockall_c: MockallOutput,
                        ) -> &mut Self
                        where
                            MockallOutput: Clone + Into<Result<String, SandcastleError>>
                                + Send + 'static,
                        {
                            self.returning(move |request| __mockall_c.clone().into())
                        }
                        /// Single-threaded version of
                        /// [`return_const`](#method.return_const).  This is useful for
                        /// return types that are not `Send`.
                        ///
                        /// The output type must be `Clone`.  The compiler can't always
                        /// infer the proper type to use with this method; you will
                        /// usually need to specify it explicitly.  i.e.
                        /// `return_const(42i32)` instead of `return_const(42)`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.
                        #[allow(unused_variables)]
                        pub(in super::super) fn return_const_st<MockallOutput>(
                            &mut self,
                            __mockall_c: MockallOutput,
                        ) -> &mut Self
                        where
                            MockallOutput: Clone + Into<Result<String, SandcastleError>>
                                + 'static,
                        {
                            self.returning_st(move |request| __mockall_c.clone().into())
                        }
                        /// Supply an `FnOnce` closure that will provide the return
                        /// value for this Expectation.  This is useful for return types
                        /// that aren't `Clone`.  It will be an error to call this
                        /// method multiple times.
                        pub(in super::super) fn return_once<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnOnce(
                                    DownloadFileRequest,
                                ) -> Result<String, SandcastleError> + Send + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::Once(
                                    Box::new(__mockall_f),
                                );
                            }
                            self
                        }
                        /// Single-threaded version of
                        /// [`return_once`](#method.return_once).  This is useful for
                        /// return types that are neither `Send` nor `Clone`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.  It is also a runtime error to call the method more
                        /// than once.
                        pub(in super::super) fn return_once_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnOnce(
                                    DownloadFileRequest,
                                ) -> Result<String, SandcastleError> + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::OnceSt(
                                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                                );
                            }
                            self
                        }
                        /// Supply a closure that will provide the return value for this
                        /// `Expectation`.  The method's arguments are passed to the
                        /// closure by value.
                        pub(in super::super) fn returning<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnMut(
                                    DownloadFileRequest,
                                ) -> Result<String, SandcastleError> + Send + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::Mut(
                                    Box::new(__mockall_f),
                                );
                            }
                            self
                        }
                        /// Single-threaded version of [`returning`](#method.returning).
                        /// Can be used when the argument or return type isn't `Send`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.
                        pub(in super::super) fn returning_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnMut(
                                    DownloadFileRequest,
                                ) -> Result<String, SandcastleError> + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::MutSt(
                                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                                );
                            }
                            self
                        }
                        /// Add this expectation to a
                        /// [`Sequence`](../../../mockall/struct.Sequence.html).
                        pub(in super::super) fn in_sequence(
                            &mut self,
                            __mockall_seq: &mut ::mockall::Sequence,
                        ) -> &mut Self {
                            self.common.in_sequence(__mockall_seq);
                            self
                        }
                        fn is_done(&self) -> bool {
                            self.common.is_done()
                        }
                        /// Validate this expectation's matcher.
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(&self, request: &DownloadFileRequest) -> bool {
                            self.common.matches(request)
                        }
                        /// Forbid this expectation from ever being called.
                        pub(in super::super) fn never(&mut self) -> &mut Self {
                            self.common.never();
                            self
                        }
                        /// Create a new, default, [`Expectation`](struct.Expectation.html)
                        pub(in super::super) fn new() -> Self {
                            Self::default()
                        }
                        /// Expect this expectation to be called exactly once.  Shortcut for
                        /// [`times(1)`](#method.times).
                        pub(in super::super) fn once(&mut self) -> &mut Self {
                            self.times(1)
                        }
                        /// Restrict the number of times that that this method may be called.
                        ///
                        /// The argument may be:
                        /// * A fixed number: `.times(4)`
                        /// * Various types of range:
                        ///   - `.times(5..10)`
                        ///   - `.times(..10)`
                        ///   - `.times(5..)`
                        ///   - `.times(5..=10)`
                        ///   - `.times(..=10)`
                        /// * The wildcard: `.times(..)`
                        pub(in super::super) fn times<MockallR>(
                            &mut self,
                            __mockall_r: MockallR,
                        ) -> &mut Self
                        where
                            MockallR: Into<::mockall::TimesRange>,
                        {
                            self.common.times(__mockall_r);
                            self
                        }
                        /// Set matching criteria for this Expectation.
                        ///
                        /// The matching predicate can be anything implemening the
                        /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
                        /// one matcher can be set per `Expectation` at a time.
                        pub(in super::super) fn with<
                            MockallMatcher0: ::mockall::Predicate<DownloadFileRequest>
                                + Send + 'static,
                        >(&mut self, request: MockallMatcher0) -> &mut Self {
                            self.common.with(request);
                            self
                        }
                        /// Set a matching function for this Expectation.
                        ///
                        /// This is equivalent to calling [`with`](#method.with) with a
                        /// function argument, like `with(predicate::function(f))`.
                        pub(in super::super) fn withf<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: Fn(&DownloadFileRequest) -> bool + Send + 'static,
                        {
                            self.common.withf(__mockall_f);
                            self
                        }
                        /// Single-threaded version of [`withf`](#method.withf).
                        /// Can be used when the argument type isn't `Send`.
                        pub(in super::super) fn withf_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: Fn(&DownloadFileRequest) -> bool + 'static,
                        {
                            self.common.withf_st(__mockall_f);
                            self
                        }
                    }
                    impl Default for Expectation {
                        fn default() -> Self {
                            Expectation {
                                common: Common::default(),
                                rfunc: Mutex::new(Rfunc::default()),
                            }
                        }
                    }
                    /// A collection of [`Expectation`](struct.Expectations.html)
                    /// objects.  Users will rarely if ever use this struct directly.
                    #[doc(hidden)]
                    pub(in super::super) struct Expectations(Vec<Expectation>);
                    impl Expectations {
                        /// Verify that all current expectations are satisfied and clear
                        /// them.
                        pub(in super::super) fn checkpoint(
                            &mut self,
                        ) -> std::vec::Drain<Expectation> {
                            self.0.drain(..)
                        }
                        /// Create a new expectation for this method.
                        pub(in super::super) fn expect(&mut self) -> &mut Expectation {
                            self.0.push(Expectation::default());
                            let __mockall_l = self.0.len();
                            &mut self.0[__mockall_l - 1]
                        }
                        pub(in super::super) const fn new() -> Self {
                            Self(Vec::new())
                        }
                    }
                    impl Default for Expectations {
                        fn default() -> Self {
                            Expectations::new()
                        }
                    }
                    impl Expectations {
                        /// Simulate calling the real method.  Every current expectation
                        /// will be checked in FIFO order and the first one with
                        /// matching arguments will be used.
                        pub(in super::super) fn call(
                            &self,
                            request: DownloadFileRequest,
                        ) -> Option<Result<String, SandcastleError>> {
                            self.0
                                .iter()
                                .find(|__mockall_e| {
                                    __mockall_e.matches(&request)
                                        && (!__mockall_e.is_done() || self.0.len() == 1)
                                })
                                .map(move |__mockall_e| __mockall_e.call(request))
                        }
                    }
                }
                #[allow(missing_docs)]
                #[allow(clippy::too_many_arguments, clippy::indexing_slicing)]
                pub mod __fetch_pr_last_commit_sha {
                    use super::*;
                    use ::mockall::CaseTreeExt;
                    use ::std::{
                        boxed::Box, mem, ops::{DerefMut, Range},
                        sync::Mutex, vec::Vec,
                    };
                    #[allow(clippy::unused_unit)]
                    enum Rfunc {
                        Default,
                        Expired,
                        Mut(
                            Box<
                                dyn FnMut(
                                    FetchPRLastCommitSHARequest,
                                ) -> Result<String, SandcastleError> + Send,
                            >,
                        ),
                        MutSt(
                            ::mockall::Fragile<
                                Box<
                                    dyn FnMut(
                                        FetchPRLastCommitSHARequest,
                                    ) -> Result<String, SandcastleError>,
                                >,
                            >,
                        ),
                        Once(
                            Box<
                                dyn FnOnce(
                                    FetchPRLastCommitSHARequest,
                                ) -> Result<String, SandcastleError> + Send,
                            >,
                        ),
                        OnceSt(
                            ::mockall::Fragile<
                                Box<
                                    dyn FnOnce(
                                        FetchPRLastCommitSHARequest,
                                    ) -> Result<String, SandcastleError>,
                                >,
                            >,
                        ),
                        _Phantom(Box<dyn Fn() + Send>),
                    }
                    impl Rfunc {
                        fn call_mut(
                            &mut self,
                            request: FetchPRLastCommitSHARequest,
                        ) -> std::result::Result<
                            Result<String, SandcastleError>,
                            &'static str,
                        > {
                            match self {
                                Rfunc::Default => {
                                    use ::mockall::ReturnDefault;
                                    ::mockall::DefaultReturner::<
                                        Result<String, SandcastleError>,
                                    >::return_default()
                                }
                                Rfunc::Expired => {
                                    Err("called twice, but it returns by move")
                                }
                                Rfunc::Mut(__mockall_f) => {
                                    ::std::result::Result::Ok(__mockall_f(request))
                                }
                                Rfunc::MutSt(__mockall_f) => {
                                    ::std::result::Result::Ok((__mockall_f.get_mut())(request))
                                }
                                Rfunc::Once(_) => {
                                    if let Rfunc::Once(mut __mockall_f) = mem::replace(
                                        self,
                                        Rfunc::Expired,
                                    ) {
                                        ::std::result::Result::Ok(__mockall_f(request))
                                    } else {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                                Rfunc::OnceSt(_) => {
                                    if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                                        self,
                                        Rfunc::Expired,
                                    ) {
                                        ::std::result::Result::Ok(
                                            (__mockall_f.into_inner())(request),
                                        )
                                    } else {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                                Rfunc::_Phantom(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    impl std::default::Default for Rfunc {
                        fn default() -> Self {
                            Rfunc::Default
                        }
                    }
                    enum Matcher {
                        Always,
                        Func(Box<dyn Fn(&FetchPRLastCommitSHARequest) -> bool + Send>),
                        FuncSt(
                            ::mockall::Fragile<
                                Box<dyn Fn(&FetchPRLastCommitSHARequest) -> bool>,
                            >,
                        ),
                        Pred(
                            Box<
                                (
                                    Box<
                                        dyn ::mockall::Predicate<FetchPRLastCommitSHARequest> + Send,
                                    >,
                                ),
                            >,
                        ),
                        _Phantom(Box<dyn Fn() + Send>),
                    }
                    impl Matcher {
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(
                            &self,
                            request: &FetchPRLastCommitSHARequest,
                        ) -> bool {
                            match self {
                                Matcher::Always => true,
                                Matcher::Func(__mockall_f) => __mockall_f(request),
                                Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(request),
                                Matcher::Pred(__mockall_pred) => {
                                    [__mockall_pred.0.eval(request)]
                                        .iter()
                                        .all(|__mockall_x| *__mockall_x)
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    impl Default for Matcher {
                        #[allow(unused_variables)]
                        fn default() -> Self {
                            Matcher::Always
                        }
                    }
                    impl ::std::fmt::Display for Matcher {
                        fn fmt(
                            &self,
                            __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
                        ) -> ::std::fmt::Result {
                            match self {
                                Matcher::Always => {
                                    __mockall_fmt.write_fmt(format_args!("<anything>"))
                                }
                                Matcher::Func(_) => {
                                    __mockall_fmt.write_fmt(format_args!("<function>"))
                                }
                                Matcher::FuncSt(_) => {
                                    __mockall_fmt
                                        .write_fmt(format_args!("<single threaded function>"))
                                }
                                Matcher::Pred(__mockall_p) => {
                                    __mockall_fmt.write_fmt(format_args!("{0}", __mockall_p.0))
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    /// Holds the stuff that is independent of the output type
                    struct Common {
                        matcher: Mutex<Matcher>,
                        seq_handle: Option<::mockall::SeqHandle>,
                        times: ::mockall::Times,
                    }
                    impl std::default::Default for Common {
                        fn default() -> Self {
                            Common {
                                matcher: Mutex::new(Matcher::default()),
                                seq_handle: None,
                                times: ::mockall::Times::default(),
                            }
                        }
                    }
                    impl Common {
                        fn call(&self, desc: &str) {
                            self.times
                                .call()
                                .unwrap_or_else(|m| {
                                    let desc = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("{0}", self.matcher.lock().unwrap()),
                                        )
                                    });
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "{0}: Expectation({1}) {2}",
                                                "MockVCSService::fetch_pr_last_commit_sha",
                                                desc,
                                                m,
                                            ),
                                        );
                                    };
                                });
                            self.verify_sequence(desc);
                            if ::mockall::ExpectedCalls::TooFew
                                != self.times.is_satisfied()
                            {
                                self.satisfy_sequence()
                            }
                        }
                        fn in_sequence(
                            &mut self,
                            __mockall_seq: &mut ::mockall::Sequence,
                        ) -> &mut Self {
                            if !self.times.is_exact() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Only Expectations with an exact call count have sequences",
                                        ),
                                    );
                                }
                            }
                            self.seq_handle = Some(__mockall_seq.next_handle());
                            self
                        }
                        fn is_done(&self) -> bool {
                            self.times.is_done()
                        }
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(
                            &self,
                            request: &FetchPRLastCommitSHARequest,
                        ) -> bool {
                            self.matcher.lock().unwrap().matches(request)
                        }
                        /// Forbid this expectation from ever being called.
                        fn never(&mut self) {
                            self.times.never();
                        }
                        fn satisfy_sequence(&self) {
                            if let Some(__mockall_handle) = &self.seq_handle {
                                __mockall_handle.satisfy()
                            }
                        }
                        /// Expect this expectation to be called any number of times
                        /// contained with the given range.
                        fn times<MockallR>(&mut self, __mockall_r: MockallR)
                        where
                            MockallR: Into<::mockall::TimesRange>,
                        {
                            self.times.times(__mockall_r)
                        }
                        fn with<
                            MockallMatcher0: ::mockall::Predicate<
                                    FetchPRLastCommitSHARequest,
                                > + Send + 'static,
                        >(&mut self, request: MockallMatcher0) {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::Pred(
                                Box::new((Box::new(request),)),
                            );
                        }
                        fn withf<MockallF>(&mut self, __mockall_f: MockallF)
                        where
                            MockallF: Fn(&FetchPRLastCommitSHARequest) -> bool + Send
                                + 'static,
                        {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::Func(
                                Box::new(__mockall_f),
                            );
                        }
                        fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
                        where
                            MockallF: Fn(&FetchPRLastCommitSHARequest) -> bool + 'static,
                        {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::FuncSt(
                                ::mockall::Fragile::new(Box::new(__mockall_f)),
                            );
                        }
                        fn verify_sequence(&self, desc: &str) {
                            if let Some(__mockall_handle) = &self.seq_handle {
                                __mockall_handle.verify(desc)
                            }
                        }
                    }
                    impl Drop for Common {
                        fn drop(&mut self) {
                            if !::std::thread::panicking() {
                                let desc = ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("{0}", self.matcher.lock().unwrap()),
                                    )
                                });
                                match self.times.is_satisfied() {
                                    ::mockall::ExpectedCalls::TooFew => {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                                    "MockVCSService::fetch_pr_last_commit_sha",
                                                    desc,
                                                    self.times.count(),
                                                    self.times.minimum(),
                                                ),
                                            );
                                        };
                                    }
                                    ::mockall::ExpectedCalls::TooMany => {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                                    "MockVCSService::fetch_pr_last_commit_sha",
                                                    desc,
                                                    self.times.count(),
                                                    self.times.maximum(),
                                                ),
                                            );
                                        };
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    /// Expectation type for methods that return a `'static` type.
                    /// This is the type returned by the `expect_*` methods.
                    pub(in super::super) struct Expectation {
                        common: Common,
                        rfunc: Mutex<Rfunc>,
                    }
                    #[allow(clippy::unused_unit)]
                    impl Expectation {
                        /// Call this [`Expectation`] as if it were the real method.
                        #[doc(hidden)]
                        pub(in super::super) fn call(
                            &self,
                            request: FetchPRLastCommitSHARequest,
                        ) -> Result<String, SandcastleError> {
                            use ::mockall::{ViaDebug, ViaNothing};
                            self.common
                                .call(
                                    &::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "MockVCSService::fetch_pr_last_commit_sha({0:?})",
                                                (&&::mockall::ArgPrinter(&request)).debug_string(),
                                            ),
                                        )
                                    }),
                                );
                            self.rfunc
                                .lock()
                                .unwrap()
                                .call_mut(request)
                                .unwrap_or_else(|message| {
                                    let desc = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("{0}", self.common.matcher.lock().unwrap()),
                                        )
                                    });
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "{0}: Expectation({1}) {2}",
                                                "MockVCSService::fetch_pr_last_commit_sha",
                                                desc,
                                                message,
                                            ),
                                        );
                                    };
                                })
                        }
                        /// Return a constant value from the `Expectation`
                        ///
                        /// The output type must be `Clone`.  The compiler can't always
                        /// infer the proper type to use with this method; you will
                        /// usually need to specify it explicitly.  i.e.
                        /// `return_const(42i32)` instead of `return_const(42)`.
                        #[allow(unused_variables)]
                        pub(in super::super) fn return_const<MockallOutput>(
                            &mut self,
                            __mockall_c: MockallOutput,
                        ) -> &mut Self
                        where
                            MockallOutput: Clone + Into<Result<String, SandcastleError>>
                                + Send + 'static,
                        {
                            self.returning(move |request| __mockall_c.clone().into())
                        }
                        /// Single-threaded version of
                        /// [`return_const`](#method.return_const).  This is useful for
                        /// return types that are not `Send`.
                        ///
                        /// The output type must be `Clone`.  The compiler can't always
                        /// infer the proper type to use with this method; you will
                        /// usually need to specify it explicitly.  i.e.
                        /// `return_const(42i32)` instead of `return_const(42)`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.
                        #[allow(unused_variables)]
                        pub(in super::super) fn return_const_st<MockallOutput>(
                            &mut self,
                            __mockall_c: MockallOutput,
                        ) -> &mut Self
                        where
                            MockallOutput: Clone + Into<Result<String, SandcastleError>>
                                + 'static,
                        {
                            self.returning_st(move |request| __mockall_c.clone().into())
                        }
                        /// Supply an `FnOnce` closure that will provide the return
                        /// value for this Expectation.  This is useful for return types
                        /// that aren't `Clone`.  It will be an error to call this
                        /// method multiple times.
                        pub(in super::super) fn return_once<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnOnce(
                                    FetchPRLastCommitSHARequest,
                                ) -> Result<String, SandcastleError> + Send + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::Once(
                                    Box::new(__mockall_f),
                                );
                            }
                            self
                        }
                        /// Single-threaded version of
                        /// [`return_once`](#method.return_once).  This is useful for
                        /// return types that are neither `Send` nor `Clone`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.  It is also a runtime error to call the method more
                        /// than once.
                        pub(in super::super) fn return_once_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnOnce(
                                    FetchPRLastCommitSHARequest,
                                ) -> Result<String, SandcastleError> + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::OnceSt(
                                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                                );
                            }
                            self
                        }
                        /// Supply a closure that will provide the return value for this
                        /// `Expectation`.  The method's arguments are passed to the
                        /// closure by value.
                        pub(in super::super) fn returning<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnMut(
                                    FetchPRLastCommitSHARequest,
                                ) -> Result<String, SandcastleError> + Send + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::Mut(
                                    Box::new(__mockall_f),
                                );
                            }
                            self
                        }
                        /// Single-threaded version of [`returning`](#method.returning).
                        /// Can be used when the argument or return type isn't `Send`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.
                        pub(in super::super) fn returning_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnMut(
                                    FetchPRLastCommitSHARequest,
                                ) -> Result<String, SandcastleError> + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::MutSt(
                                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                                );
                            }
                            self
                        }
                        /// Add this expectation to a
                        /// [`Sequence`](../../../mockall/struct.Sequence.html).
                        pub(in super::super) fn in_sequence(
                            &mut self,
                            __mockall_seq: &mut ::mockall::Sequence,
                        ) -> &mut Self {
                            self.common.in_sequence(__mockall_seq);
                            self
                        }
                        fn is_done(&self) -> bool {
                            self.common.is_done()
                        }
                        /// Validate this expectation's matcher.
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(
                            &self,
                            request: &FetchPRLastCommitSHARequest,
                        ) -> bool {
                            self.common.matches(request)
                        }
                        /// Forbid this expectation from ever being called.
                        pub(in super::super) fn never(&mut self) -> &mut Self {
                            self.common.never();
                            self
                        }
                        /// Create a new, default, [`Expectation`](struct.Expectation.html)
                        pub(in super::super) fn new() -> Self {
                            Self::default()
                        }
                        /// Expect this expectation to be called exactly once.  Shortcut for
                        /// [`times(1)`](#method.times).
                        pub(in super::super) fn once(&mut self) -> &mut Self {
                            self.times(1)
                        }
                        /// Restrict the number of times that that this method may be called.
                        ///
                        /// The argument may be:
                        /// * A fixed number: `.times(4)`
                        /// * Various types of range:
                        ///   - `.times(5..10)`
                        ///   - `.times(..10)`
                        ///   - `.times(5..)`
                        ///   - `.times(5..=10)`
                        ///   - `.times(..=10)`
                        /// * The wildcard: `.times(..)`
                        pub(in super::super) fn times<MockallR>(
                            &mut self,
                            __mockall_r: MockallR,
                        ) -> &mut Self
                        where
                            MockallR: Into<::mockall::TimesRange>,
                        {
                            self.common.times(__mockall_r);
                            self
                        }
                        /// Set matching criteria for this Expectation.
                        ///
                        /// The matching predicate can be anything implemening the
                        /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
                        /// one matcher can be set per `Expectation` at a time.
                        pub(in super::super) fn with<
                            MockallMatcher0: ::mockall::Predicate<
                                    FetchPRLastCommitSHARequest,
                                > + Send + 'static,
                        >(&mut self, request: MockallMatcher0) -> &mut Self {
                            self.common.with(request);
                            self
                        }
                        /// Set a matching function for this Expectation.
                        ///
                        /// This is equivalent to calling [`with`](#method.with) with a
                        /// function argument, like `with(predicate::function(f))`.
                        pub(in super::super) fn withf<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: Fn(&FetchPRLastCommitSHARequest) -> bool + Send
                                + 'static,
                        {
                            self.common.withf(__mockall_f);
                            self
                        }
                        /// Single-threaded version of [`withf`](#method.withf).
                        /// Can be used when the argument type isn't `Send`.
                        pub(in super::super) fn withf_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: Fn(&FetchPRLastCommitSHARequest) -> bool + 'static,
                        {
                            self.common.withf_st(__mockall_f);
                            self
                        }
                    }
                    impl Default for Expectation {
                        fn default() -> Self {
                            Expectation {
                                common: Common::default(),
                                rfunc: Mutex::new(Rfunc::default()),
                            }
                        }
                    }
                    /// A collection of [`Expectation`](struct.Expectations.html)
                    /// objects.  Users will rarely if ever use this struct directly.
                    #[doc(hidden)]
                    pub(in super::super) struct Expectations(Vec<Expectation>);
                    impl Expectations {
                        /// Verify that all current expectations are satisfied and clear
                        /// them.
                        pub(in super::super) fn checkpoint(
                            &mut self,
                        ) -> std::vec::Drain<Expectation> {
                            self.0.drain(..)
                        }
                        /// Create a new expectation for this method.
                        pub(in super::super) fn expect(&mut self) -> &mut Expectation {
                            self.0.push(Expectation::default());
                            let __mockall_l = self.0.len();
                            &mut self.0[__mockall_l - 1]
                        }
                        pub(in super::super) const fn new() -> Self {
                            Self(Vec::new())
                        }
                    }
                    impl Default for Expectations {
                        fn default() -> Self {
                            Expectations::new()
                        }
                    }
                    impl Expectations {
                        /// Simulate calling the real method.  Every current expectation
                        /// will be checked in FIFO order and the first one with
                        /// matching arguments will be used.
                        pub(in super::super) fn call(
                            &self,
                            request: FetchPRLastCommitSHARequest,
                        ) -> Option<Result<String, SandcastleError>> {
                            self.0
                                .iter()
                                .find(|__mockall_e| {
                                    __mockall_e.matches(&request)
                                        && (!__mockall_e.is_done() || self.0.len() == 1)
                                })
                                .map(move |__mockall_e| __mockall_e.call(request))
                        }
                    }
                }
            }
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            #[allow(missing_docs)]
            pub struct MockVCSService {
                Clone_expectations: MockVCSService_Clone,
                download_file: __mock_MockVCSService::__download_file::Expectations,
                fetch_pr_last_commit_sha: __mock_MockVCSService::__fetch_pr_last_commit_sha::Expectations,
            }
            impl ::std::default::Default for MockVCSService {
                #[allow(clippy::default_trait_access)]
                fn default() -> Self {
                    Self {
                        Clone_expectations: Default::default(),
                        download_file: Default::default(),
                        fetch_pr_last_commit_sha: Default::default(),
                    }
                }
            }
            #[allow(non_snake_case)]
            #[allow(missing_docs)]
            pub mod __mock_MockVCSService_Clone {
                use super::*;
                #[allow(missing_docs)]
                #[allow(clippy::too_many_arguments, clippy::indexing_slicing)]
                pub mod __clone {
                    use super::*;
                    use ::mockall::CaseTreeExt;
                    use ::std::{
                        boxed::Box, mem, ops::{DerefMut, Range},
                        sync::Mutex, vec::Vec,
                    };
                    #[allow(clippy::unused_unit)]
                    enum Rfunc {
                        Default,
                        Expired,
                        Mut(Box<dyn FnMut() -> MockVCSService + Send>),
                        MutSt(::mockall::Fragile<Box<dyn FnMut() -> MockVCSService>>),
                        Once(Box<dyn FnOnce() -> MockVCSService + Send>),
                        OnceSt(::mockall::Fragile<Box<dyn FnOnce() -> MockVCSService>>),
                        _Phantom(Box<dyn Fn() + Send>),
                    }
                    impl Rfunc {
                        fn call_mut(
                            &mut self,
                        ) -> std::result::Result<MockVCSService, &'static str> {
                            match self {
                                Rfunc::Default => {
                                    use ::mockall::ReturnDefault;
                                    ::mockall::DefaultReturner::<
                                        MockVCSService,
                                    >::return_default()
                                }
                                Rfunc::Expired => {
                                    Err("called twice, but it returns by move")
                                }
                                Rfunc::Mut(__mockall_f) => {
                                    ::std::result::Result::Ok(__mockall_f())
                                }
                                Rfunc::MutSt(__mockall_f) => {
                                    ::std::result::Result::Ok((__mockall_f.get_mut())())
                                }
                                Rfunc::Once(_) => {
                                    if let Rfunc::Once(mut __mockall_f) = mem::replace(
                                        self,
                                        Rfunc::Expired,
                                    ) {
                                        ::std::result::Result::Ok(__mockall_f())
                                    } else {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                                Rfunc::OnceSt(_) => {
                                    if let Rfunc::OnceSt(mut __mockall_f) = mem::replace(
                                        self,
                                        Rfunc::Expired,
                                    ) {
                                        ::std::result::Result::Ok((__mockall_f.into_inner())())
                                    } else {
                                        ::core::panicking::panic(
                                            "internal error: entered unreachable code",
                                        )
                                    }
                                }
                                Rfunc::_Phantom(_) => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    impl std::default::Default for Rfunc {
                        fn default() -> Self {
                            Rfunc::Default
                        }
                    }
                    enum Matcher {
                        Always,
                        Func(Box<dyn Fn() -> bool + Send>),
                        FuncSt(::mockall::Fragile<Box<dyn Fn() -> bool>>),
                        Pred(Box<()>),
                        _Phantom(Box<dyn Fn() + Send>),
                    }
                    impl Matcher {
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(&self) -> bool {
                            match self {
                                Matcher::Always => true,
                                Matcher::Func(__mockall_f) => __mockall_f(),
                                Matcher::FuncSt(__mockall_f) => (__mockall_f.get())(),
                                Matcher::Pred(__mockall_pred) => {
                                    [].iter().all(|__mockall_x| *__mockall_x)
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    impl Default for Matcher {
                        #[allow(unused_variables)]
                        fn default() -> Self {
                            Matcher::Always
                        }
                    }
                    impl ::std::fmt::Display for Matcher {
                        fn fmt(
                            &self,
                            __mockall_fmt: &mut ::std::fmt::Formatter<'_>,
                        ) -> ::std::fmt::Result {
                            match self {
                                Matcher::Always => {
                                    __mockall_fmt.write_fmt(format_args!("<anything>"))
                                }
                                Matcher::Func(_) => {
                                    __mockall_fmt.write_fmt(format_args!("<function>"))
                                }
                                Matcher::FuncSt(_) => {
                                    __mockall_fmt
                                        .write_fmt(format_args!("<single threaded function>"))
                                }
                                Matcher::Pred(__mockall_p) => {
                                    __mockall_fmt.write_fmt(format_args!(""))
                                }
                                _ => {
                                    ::core::panicking::panic(
                                        "internal error: entered unreachable code",
                                    )
                                }
                            }
                        }
                    }
                    /// Holds the stuff that is independent of the output type
                    struct Common {
                        matcher: Mutex<Matcher>,
                        seq_handle: Option<::mockall::SeqHandle>,
                        times: ::mockall::Times,
                    }
                    impl std::default::Default for Common {
                        fn default() -> Self {
                            Common {
                                matcher: Mutex::new(Matcher::default()),
                                seq_handle: None,
                                times: ::mockall::Times::default(),
                            }
                        }
                    }
                    impl Common {
                        fn call(&self, desc: &str) {
                            self.times
                                .call()
                                .unwrap_or_else(|m| {
                                    let desc = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("{0}", self.matcher.lock().unwrap()),
                                        )
                                    });
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "{0}: Expectation({1}) {2}",
                                                "MockVCSService::clone",
                                                desc,
                                                m,
                                            ),
                                        );
                                    };
                                });
                            self.verify_sequence(desc);
                            if ::mockall::ExpectedCalls::TooFew
                                != self.times.is_satisfied()
                            {
                                self.satisfy_sequence()
                            }
                        }
                        fn in_sequence(
                            &mut self,
                            __mockall_seq: &mut ::mockall::Sequence,
                        ) -> &mut Self {
                            if !self.times.is_exact() {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "Only Expectations with an exact call count have sequences",
                                        ),
                                    );
                                }
                            }
                            self.seq_handle = Some(__mockall_seq.next_handle());
                            self
                        }
                        fn is_done(&self) -> bool {
                            self.times.is_done()
                        }
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(&self) -> bool {
                            self.matcher.lock().unwrap().matches()
                        }
                        /// Forbid this expectation from ever being called.
                        fn never(&mut self) {
                            self.times.never();
                        }
                        fn satisfy_sequence(&self) {
                            if let Some(__mockall_handle) = &self.seq_handle {
                                __mockall_handle.satisfy()
                            }
                        }
                        /// Expect this expectation to be called any number of times
                        /// contained with the given range.
                        fn times<MockallR>(&mut self, __mockall_r: MockallR)
                        where
                            MockallR: Into<::mockall::TimesRange>,
                        {
                            self.times.times(__mockall_r)
                        }
                        fn with(&mut self) {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::Pred(Box::new(()));
                        }
                        fn withf<MockallF>(&mut self, __mockall_f: MockallF)
                        where
                            MockallF: Fn() -> bool + Send + 'static,
                        {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::Func(
                                Box::new(__mockall_f),
                            );
                        }
                        fn withf_st<MockallF>(&mut self, __mockall_f: MockallF)
                        where
                            MockallF: Fn() -> bool + 'static,
                        {
                            let mut __mockall_guard = self.matcher.lock().unwrap();
                            *__mockall_guard.deref_mut() = Matcher::FuncSt(
                                ::mockall::Fragile::new(Box::new(__mockall_f)),
                            );
                        }
                        fn verify_sequence(&self, desc: &str) {
                            if let Some(__mockall_handle) = &self.seq_handle {
                                __mockall_handle.verify(desc)
                            }
                        }
                    }
                    impl Drop for Common {
                        fn drop(&mut self) {
                            if !::std::thread::panicking() {
                                let desc = ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("{0}", self.matcher.lock().unwrap()),
                                    )
                                });
                                match self.times.is_satisfied() {
                                    ::mockall::ExpectedCalls::TooFew => {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "{0}: Expectation({1}) called {2} time(s) which is fewer than expected {3}",
                                                    "MockVCSService::clone",
                                                    desc,
                                                    self.times.count(),
                                                    self.times.minimum(),
                                                ),
                                            );
                                        };
                                    }
                                    ::mockall::ExpectedCalls::TooMany => {
                                        {
                                            ::core::panicking::panic_fmt(
                                                format_args!(
                                                    "{0}: Expectation({1}) called {2} time(s) which is more than expected {3}",
                                                    "MockVCSService::clone",
                                                    desc,
                                                    self.times.count(),
                                                    self.times.maximum(),
                                                ),
                                            );
                                        };
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                    /// Expectation type for methods that return a `'static` type.
                    /// This is the type returned by the `expect_*` methods.
                    pub struct Expectation {
                        common: Common,
                        rfunc: Mutex<Rfunc>,
                    }
                    #[allow(clippy::unused_unit)]
                    impl Expectation {
                        /// Call this [`Expectation`] as if it were the real method.
                        #[doc(hidden)]
                        pub fn call(&self) -> MockVCSService {
                            use ::mockall::{ViaDebug, ViaNothing};
                            self.common
                                .call(
                                    &::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("MockVCSService::clone()"),
                                        )
                                    }),
                                );
                            self.rfunc
                                .lock()
                                .unwrap()
                                .call_mut()
                                .unwrap_or_else(|message| {
                                    let desc = ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!("{0}", self.common.matcher.lock().unwrap()),
                                        )
                                    });
                                    {
                                        ::core::panicking::panic_fmt(
                                            format_args!(
                                                "{0}: Expectation({1}) {2}",
                                                "MockVCSService::clone",
                                                desc,
                                                message,
                                            ),
                                        );
                                    };
                                })
                        }
                        /// Return a constant value from the `Expectation`
                        ///
                        /// The output type must be `Clone`.  The compiler can't always
                        /// infer the proper type to use with this method; you will
                        /// usually need to specify it explicitly.  i.e.
                        /// `return_const(42i32)` instead of `return_const(42)`.
                        #[allow(unused_variables)]
                        pub fn return_const<MockallOutput>(
                            &mut self,
                            __mockall_c: MockallOutput,
                        ) -> &mut Self
                        where
                            MockallOutput: Clone + Into<MockVCSService> + Send + 'static,
                        {
                            self.returning(move || __mockall_c.clone().into())
                        }
                        /// Single-threaded version of
                        /// [`return_const`](#method.return_const).  This is useful for
                        /// return types that are not `Send`.
                        ///
                        /// The output type must be `Clone`.  The compiler can't always
                        /// infer the proper type to use with this method; you will
                        /// usually need to specify it explicitly.  i.e.
                        /// `return_const(42i32)` instead of `return_const(42)`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.
                        #[allow(unused_variables)]
                        pub fn return_const_st<MockallOutput>(
                            &mut self,
                            __mockall_c: MockallOutput,
                        ) -> &mut Self
                        where
                            MockallOutput: Clone + Into<MockVCSService> + 'static,
                        {
                            self.returning_st(move || __mockall_c.clone().into())
                        }
                        /// Supply an `FnOnce` closure that will provide the return
                        /// value for this Expectation.  This is useful for return types
                        /// that aren't `Clone`.  It will be an error to call this
                        /// method multiple times.
                        pub fn return_once<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnOnce() -> MockVCSService + Send + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::Once(
                                    Box::new(__mockall_f),
                                );
                            }
                            self
                        }
                        /// Single-threaded version of
                        /// [`return_once`](#method.return_once).  This is useful for
                        /// return types that are neither `Send` nor `Clone`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.  It is also a runtime error to call the method more
                        /// than once.
                        pub fn return_once_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnOnce() -> MockVCSService + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::OnceSt(
                                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                                );
                            }
                            self
                        }
                        /// Supply a closure that will provide the return value for this
                        /// `Expectation`.  The method's arguments are passed to the
                        /// closure by value.
                        pub fn returning<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnMut() -> MockVCSService + Send + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::Mut(
                                    Box::new(__mockall_f),
                                );
                            }
                            self
                        }
                        /// Single-threaded version of [`returning`](#method.returning).
                        /// Can be used when the argument or return type isn't `Send`.
                        ///
                        /// It is a runtime error to call the mock method from a
                        /// different thread than the one that originally called this
                        /// method.
                        pub fn returning_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: FnMut() -> MockVCSService + 'static,
                        {
                            {
                                let mut __mockall_guard = self.rfunc.lock().unwrap();
                                *__mockall_guard.deref_mut() = Rfunc::MutSt(
                                    ::mockall::Fragile::new(Box::new(__mockall_f)),
                                );
                            }
                            self
                        }
                        /// Add this expectation to a
                        /// [`Sequence`](../../../mockall/struct.Sequence.html).
                        pub fn in_sequence(
                            &mut self,
                            __mockall_seq: &mut ::mockall::Sequence,
                        ) -> &mut Self {
                            self.common.in_sequence(__mockall_seq);
                            self
                        }
                        fn is_done(&self) -> bool {
                            self.common.is_done()
                        }
                        /// Validate this expectation's matcher.
                        #[allow(clippy::ptr_arg)]
                        #[allow(clippy::ref_option)]
                        fn matches(&self) -> bool {
                            self.common.matches()
                        }
                        /// Forbid this expectation from ever being called.
                        pub fn never(&mut self) -> &mut Self {
                            self.common.never();
                            self
                        }
                        /// Create a new, default, [`Expectation`](struct.Expectation.html)
                        pub fn new() -> Self {
                            Self::default()
                        }
                        /// Expect this expectation to be called exactly once.  Shortcut for
                        /// [`times(1)`](#method.times).
                        pub fn once(&mut self) -> &mut Self {
                            self.times(1)
                        }
                        /// Restrict the number of times that that this method may be called.
                        ///
                        /// The argument may be:
                        /// * A fixed number: `.times(4)`
                        /// * Various types of range:
                        ///   - `.times(5..10)`
                        ///   - `.times(..10)`
                        ///   - `.times(5..)`
                        ///   - `.times(5..=10)`
                        ///   - `.times(..=10)`
                        /// * The wildcard: `.times(..)`
                        pub fn times<MockallR>(
                            &mut self,
                            __mockall_r: MockallR,
                        ) -> &mut Self
                        where
                            MockallR: Into<::mockall::TimesRange>,
                        {
                            self.common.times(__mockall_r);
                            self
                        }
                        /// Set matching criteria for this Expectation.
                        ///
                        /// The matching predicate can be anything implemening the
                        /// [`Predicate`](../../../mockall/trait.Predicate.html) trait.  Only
                        /// one matcher can be set per `Expectation` at a time.
                        pub fn with(&mut self) -> &mut Self {
                            self.common.with();
                            self
                        }
                        /// Set a matching function for this Expectation.
                        ///
                        /// This is equivalent to calling [`with`](#method.with) with a
                        /// function argument, like `with(predicate::function(f))`.
                        pub fn withf<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: Fn() -> bool + Send + 'static,
                        {
                            self.common.withf(__mockall_f);
                            self
                        }
                        /// Single-threaded version of [`withf`](#method.withf).
                        /// Can be used when the argument type isn't `Send`.
                        pub fn withf_st<MockallF>(
                            &mut self,
                            __mockall_f: MockallF,
                        ) -> &mut Self
                        where
                            MockallF: Fn() -> bool + 'static,
                        {
                            self.common.withf_st(__mockall_f);
                            self
                        }
                    }
                    impl Default for Expectation {
                        fn default() -> Self {
                            Expectation {
                                common: Common::default(),
                                rfunc: Mutex::new(Rfunc::default()),
                            }
                        }
                    }
                    /// A collection of [`Expectation`](struct.Expectations.html)
                    /// objects.  Users will rarely if ever use this struct directly.
                    #[doc(hidden)]
                    pub struct Expectations(Vec<Expectation>);
                    impl Expectations {
                        /// Verify that all current expectations are satisfied and clear
                        /// them.
                        pub fn checkpoint(&mut self) -> std::vec::Drain<Expectation> {
                            self.0.drain(..)
                        }
                        /// Create a new expectation for this method.
                        pub fn expect(&mut self) -> &mut Expectation {
                            self.0.push(Expectation::default());
                            let __mockall_l = self.0.len();
                            &mut self.0[__mockall_l - 1]
                        }
                        pub const fn new() -> Self {
                            Self(Vec::new())
                        }
                    }
                    impl Default for Expectations {
                        fn default() -> Self {
                            Expectations::new()
                        }
                    }
                    impl Expectations {
                        /// Simulate calling the real method.  Every current expectation
                        /// will be checked in FIFO order and the first one with
                        /// matching arguments will be used.
                        pub fn call(&self) -> Option<MockVCSService> {
                            self.0
                                .iter()
                                .find(|__mockall_e| {
                                    __mockall_e.matches()
                                        && (!__mockall_e.is_done() || self.0.len() == 1)
                                })
                                .map(move |__mockall_e| __mockall_e.call())
                        }
                    }
                }
            }
            #[allow(non_camel_case_types)]
            #[allow(non_snake_case)]
            #[allow(missing_docs)]
            struct MockVCSService_Clone {
                clone: __mock_MockVCSService_Clone::__clone::Expectations,
            }
            impl ::std::default::Default for MockVCSService_Clone {
                fn default() -> Self {
                    Self { clone: Default::default() }
                }
            }
            impl MockVCSService_Clone {
                /// Validate that all current expectations for all methods have
                /// been satisfied, and discard them.
                pub fn checkpoint(&mut self) {
                    {
                        self.clone.checkpoint();
                    }
                }
            }
            impl MockVCSService {
                #[allow(dead_code)]
                async fn download_file(
                    &self,
                    request: DownloadFileRequest,
                ) -> Result<String, SandcastleError> {
                    use ::mockall::{ViaDebug, ViaNothing};
                    let no_match_msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{0}: No matching expectation found",
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "MockVCSService::download_file({0:?})",
                                            (&&::mockall::ArgPrinter(&request)).debug_string(),
                                        ),
                                    )
                                }),
                            ),
                        )
                    });
                    self.download_file.call(request).expect(&no_match_msg)
                }
                #[allow(dead_code)]
                async fn fetch_pr_last_commit_sha(
                    &self,
                    request: FetchPRLastCommitSHARequest,
                ) -> Result<String, SandcastleError> {
                    use ::mockall::{ViaDebug, ViaNothing};
                    let no_match_msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{0}: No matching expectation found",
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "MockVCSService::fetch_pr_last_commit_sha({0:?})",
                                            (&&::mockall::ArgPrinter(&request)).debug_string(),
                                        ),
                                    )
                                }),
                            ),
                        )
                    });
                    self.fetch_pr_last_commit_sha.call(request).expect(&no_match_msg)
                }
                #[must_use = "Must set return value when not using the \"nightly\" feature"]
                ///Create an [`Expectation`](__mock_MockVCSService/__download_file/struct.Expectation.html) for mocking the `download_file` method
                fn expect_download_file(
                    &mut self,
                ) -> &mut __mock_MockVCSService::__download_file::Expectation {
                    self.download_file.expect()
                }
                #[must_use = "Must set return value when not using the \"nightly\" feature"]
                ///Create an [`Expectation`](__mock_MockVCSService/__fetch_pr_last_commit_sha/struct.Expectation.html) for mocking the `fetch_pr_last_commit_sha` method
                fn expect_fetch_pr_last_commit_sha(
                    &mut self,
                ) -> &mut __mock_MockVCSService::__fetch_pr_last_commit_sha::Expectation {
                    self.fetch_pr_last_commit_sha.expect()
                }
                /// Validate that all current expectations for all methods have
                /// been satisfied, and discard them.
                pub fn checkpoint(&mut self) {
                    self.Clone_expectations.checkpoint();
                    {
                        self.download_file.checkpoint();
                    }
                    {
                        self.fetch_pr_last_commit_sha.checkpoint();
                    }
                }
                /// Create a new mock object with no expectations.
                ///
                /// This method will not be generated if the real struct
                /// already has a `new` method.  However, it *will* be
                /// generated if the struct implements a trait with a `new`
                /// method.  The trait's `new` method can still be called
                /// like `<MockX as TraitY>::new`
                pub fn new() -> Self {
                    Self::default()
                }
            }
            impl Clone for MockVCSService {
                fn clone(&self) -> MockVCSService {
                    use ::mockall::{ViaDebug, ViaNothing};
                    let no_match_msg = ::alloc::__export::must_use({
                        ::alloc::fmt::format(
                            format_args!(
                                "{0}: No matching expectation found",
                                ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!("MockVCSService::clone()"),
                                    )
                                }),
                            ),
                        )
                    });
                    self.Clone_expectations.clone.call().expect(&no_match_msg)
                }
            }
            impl MockVCSService {
                #[must_use = "Must set return value when not using the \"nightly\" feature"]
                ///Create an [`Expectation`](__mock_MockVCSService_Clone/__clone/struct.Expectation.html) for mocking the `clone` method
                pub fn expect_clone(
                    &mut self,
                ) -> &mut __mock_MockVCSService_Clone::__clone::Expectation {
                    self.Clone_expectations.clone.expect()
                }
            }
            /// A trait for a GitOps platform services
            /// This is supposed to be argocd, flux, etc.
            pub trait GitOpsPlatformService: Clone + Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn create_or_update_application<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    application: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<(), SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn delete_application<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    application: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<(), SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait;
            }
            /// A trait for a VCS service
            /// This is supposed to be github, gitlab, etc.
            pub trait VCSService: Clone + Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn download_file<'life0, 'async_trait>(
                    &'life0 self,
                    request: DownloadFileRequest,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<String, SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn fetch_pr_last_commit_sha<'life0, 'async_trait>(
                    &'life0 self,
                    request: FetchPRLastCommitSHARequest,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<String, SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
            }
        }
        pub mod services {
            mod environment {
                use async_trait::async_trait;
                use tracing::instrument;
                use crate::{
                    domain::environment::{
                        models::{
                            CreateOrUpdateArgocdApplicationAction,
                            DeleteArgocdApplicationAction, ReconcileContext,
                        },
                        ports::Reconcile,
                    },
                    error::SandcastleError,
                };
                impl Reconcile for CreateOrUpdateArgocdApplicationAction {
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn reconcile<'life0, 'async_trait>(
                        &'life0 self,
                        context: ReconcileContext,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), SandcastleError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        ::std::boxed::Box::pin(async move {
                            let __tracing_attr_span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "reconcile",
                                            "sandcastle_core::domain::environment::services::environment",
                                            ::tracing::Level::INFO,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/sandcastle-core/src/domain/environment/services/environment.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(16u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "sandcastle_core::domain::environment::services::environment",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &[],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{ meta.fields().value_set(&[]) },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    if match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                span.record_all(
                                                    &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                                );
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                    span
                                }
                            };
                            let __tracing_instrument_future = async move {
                                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                    Result<(), SandcastleError>,
                                > {
                                    #[allow(unreachable_code)] return __ret;
                                }
                                let __self = self;
                                let context = context;
                                let __ret: Result<(), SandcastleError> = { Ok(()) };
                                #[allow(unreachable_code)] __ret
                            };
                            if !__tracing_attr_span.is_disabled() {
                                ::tracing::Instrument::instrument(
                                        __tracing_instrument_future,
                                        __tracing_attr_span,
                                    )
                                    .await
                            } else {
                                __tracing_instrument_future.await
                            }
                        })
                    }
                }
                impl Reconcile for DeleteArgocdApplicationAction {
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn reconcile<'life0, 'async_trait>(
                        &'life0 self,
                        context: ReconcileContext,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), SandcastleError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        ::std::boxed::Box::pin(async move {
                            let __tracing_attr_span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "reconcile",
                                            "sandcastle_core::domain::environment::services::environment",
                                            ::tracing::Level::INFO,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/sandcastle-core/src/domain/environment/services/environment.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(24u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "sandcastle_core::domain::environment::services::environment",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &[],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{ meta.fields().value_set(&[]) },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    if match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                span.record_all(
                                                    &{ __CALLSITE.metadata().fields().value_set(&[]) },
                                                );
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                    span
                                }
                            };
                            let __tracing_instrument_future = async move {
                                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                    Result<(), SandcastleError>,
                                > {
                                    #[allow(unreachable_code)] return __ret;
                                }
                                let __self = self;
                                let context = context;
                                let __ret: Result<(), SandcastleError> = { Ok(()) };
                                #[allow(unreachable_code)] __ret
                            };
                            if !__tracing_attr_span.is_disabled() {
                                ::tracing::Instrument::instrument(
                                        __tracing_instrument_future,
                                        __tracing_attr_span,
                                    )
                                    .await
                            } else {
                                __tracing_instrument_future.await
                            }
                        })
                    }
                }
            }
            mod gitops {
                use async_trait::async_trait;
                use crate::{
                    domain::environment::ports::GitOpsPlatformService,
                    error::SandcastleError,
                };
                pub struct ArgoCD;
                #[automatically_derived]
                impl ::core::clone::Clone for ArgoCD {
                    #[inline]
                    fn clone(&self) -> ArgoCD {
                        ArgoCD
                    }
                }
                impl GitOpsPlatformService for ArgoCD {
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn create_or_update_application<'life0, 'life1, 'async_trait>(
                        &'life0 self,
                        application: &'life1 str,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), SandcastleError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        'life1: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), SandcastleError>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let __self = self;
                            let __ret: Result<(), SandcastleError> = { Ok(()) };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn delete_application<'life0, 'life1, 'async_trait>(
                        &'life0 self,
                        application: &'life1 str,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<(), SandcastleError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        'life1: 'async_trait,
                        Self: 'async_trait,
                    {
                        Box::pin(async move {
                            if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                Result<(), SandcastleError>,
                            > {
                                #[allow(unreachable_code)] return __ret;
                            }
                            let __self = self;
                            let __ret: Result<(), SandcastleError> = { Ok(()) };
                            #[allow(unreachable_code)] __ret
                        })
                    }
                }
            }
            mod github {
                use std::backtrace::Backtrace;
                use async_trait::async_trait;
                use octocrab::Octocrab;
                use tracing::instrument;
                use crate::{
                    domain::environment::{
                        models::{DownloadFileRequest, FetchPRLastCommitSHARequest},
                        ports::VCSService,
                    },
                    error::{SandcastleError, ServiceErrorCode},
                };
                pub struct GitHub {
                    client: Octocrab,
                }
                #[automatically_derived]
                impl ::core::fmt::Debug for GitHub {
                    #[inline]
                    fn fmt(
                        &self,
                        f: &mut ::core::fmt::Formatter,
                    ) -> ::core::fmt::Result {
                        ::core::fmt::Formatter::debug_struct_field1_finish(
                            f,
                            "GitHub",
                            "client",
                            &&self.client,
                        )
                    }
                }
                #[automatically_derived]
                impl ::core::clone::Clone for GitHub {
                    #[inline]
                    fn clone(&self) -> GitHub {
                        GitHub {
                            client: ::core::clone::Clone::clone(&self.client),
                        }
                    }
                }
                impl From<Octocrab> for GitHub {
                    fn from(client: Octocrab) -> Self {
                        Self { client }
                    }
                }
                impl VCSService for GitHub {
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn download_file<'life0, 'async_trait>(
                        &'life0 self,
                        request: DownloadFileRequest,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<String, SandcastleError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        ::std::boxed::Box::pin(async move {
                            let __tracing_attr_span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "download_file",
                                            "sandcastle_core::domain::environment::services::github",
                                            ::tracing::Level::INFO,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/sandcastle-core/src/domain/environment/services/github.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(28u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "sandcastle_core::domain::environment::services::github",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["request"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &::tracing::field::debug(&request) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    if match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                span.record_all(
                                                    &{
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                        __CALLSITE
                                                            .metadata()
                                                            .fields()
                                                            .value_set(
                                                                &[
                                                                    (
                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                        ::tracing::__macro_support::Option::Some(
                                                                            &::tracing::field::debug(&request) as &dyn Value,
                                                                        ),
                                                                    ),
                                                                ],
                                                            )
                                                    },
                                                );
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                    span
                                }
                            };
                            let __tracing_instrument_future = async move {
                                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                    Result<String, SandcastleError>,
                                > {
                                    #[allow(unreachable_code)] return __ret;
                                }
                                let __self = self;
                                let request = request;
                                let __ret: Result<String, SandcastleError> = {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/sandcastle-core/src/domain/environment/services/github.rs:30",
                                                    "sandcastle_core::domain::environment::services::github",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/sandcastle-core/src/domain/environment/services/github.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(30u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "sandcastle_core::domain::environment::services::github",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("Downloading file from GitHub") as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::tracing::__macro_support::Option::Some(
                                                                                            &format_args!("Downloading file from GitHub") as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                    let mut file = __self
                                        .client
                                        .repos_by_id(request.repository_id)
                                        .get_content()
                                        .path(request.path.clone())
                                        .r#ref(request.r#ref)
                                        .send()
                                        .await
                                        .map_err(|e| SandcastleError::Service {
                                            code: ServiceErrorCode::VCSFileDownloadFailed,
                                            message: e.to_string(),
                                            reason: request.path.clone(),
                                            backtrace: Backtrace::capture(),
                                        })?;
                                    let file = file
                                        .take_items()
                                        .first()
                                        .ok_or(SandcastleError::Service {
                                            code: ServiceErrorCode::VCSFileNotFound,
                                            message: "File not found".to_string(),
                                            reason: request.path.clone(),
                                            backtrace: Backtrace::capture(),
                                        })?
                                        .decoded_content()
                                        .ok_or(SandcastleError::Service {
                                            code: ServiceErrorCode::VCSFileDownloadFailed,
                                            message: "Failure to decode file content".to_string(),
                                            reason: request.path.clone(),
                                            backtrace: Backtrace::capture(),
                                        })?
                                        .to_string();
                                    Ok(file)
                                };
                                #[allow(unreachable_code)] __ret
                            };
                            if !__tracing_attr_span.is_disabled() {
                                ::tracing::Instrument::instrument(
                                        __tracing_instrument_future,
                                        __tracing_attr_span,
                                    )
                                    .await
                            } else {
                                __tracing_instrument_future.await
                            }
                        })
                    }
                    #[allow(
                        elided_named_lifetimes,
                        clippy::async_yields_async,
                        clippy::diverging_sub_expression,
                        clippy::let_unit_value,
                        clippy::needless_arbitrary_self_type,
                        clippy::no_effect_underscore_binding,
                        clippy::shadow_same,
                        clippy::type_complexity,
                        clippy::type_repetition_in_bounds,
                        clippy::used_underscore_binding
                    )]
                    fn fetch_pr_last_commit_sha<'life0, 'async_trait>(
                        &'life0 self,
                        request: FetchPRLastCommitSHARequest,
                    ) -> ::core::pin::Pin<
                        Box<
                            dyn ::core::future::Future<
                                Output = Result<String, SandcastleError>,
                            > + ::core::marker::Send + 'async_trait,
                        >,
                    >
                    where
                        'life0: 'async_trait,
                        Self: 'async_trait,
                    {
                        ::std::boxed::Box::pin(async move {
                            let __tracing_attr_span = {
                                use ::tracing::__macro_support::Callsite as _;
                                static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                    static META: ::tracing::Metadata<'static> = {
                                        ::tracing_core::metadata::Metadata::new(
                                            "fetch_pr_last_commit_sha",
                                            "sandcastle_core::domain::environment::services::github",
                                            ::tracing::Level::INFO,
                                            ::tracing_core::__macro_support::Option::Some(
                                                "crates/sandcastle-core/src/domain/environment/services/github.rs",
                                            ),
                                            ::tracing_core::__macro_support::Option::Some(67u32),
                                            ::tracing_core::__macro_support::Option::Some(
                                                "sandcastle_core::domain::environment::services::github",
                                            ),
                                            ::tracing_core::field::FieldSet::new(
                                                &["request"],
                                                ::tracing_core::callsite::Identifier(&__CALLSITE),
                                            ),
                                            ::tracing::metadata::Kind::SPAN,
                                        )
                                    };
                                    ::tracing::callsite::DefaultCallsite::new(&META)
                                };
                                let mut interest = ::tracing::subscriber::Interest::never();
                                if ::tracing::Level::INFO
                                    <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                    && ::tracing::Level::INFO
                                        <= ::tracing::level_filters::LevelFilter::current()
                                    && {
                                        interest = __CALLSITE.interest();
                                        !interest.is_never()
                                    }
                                    && ::tracing::__macro_support::__is_enabled(
                                        __CALLSITE.metadata(),
                                        interest,
                                    )
                                {
                                    let meta = __CALLSITE.metadata();
                                    ::tracing::Span::new(
                                        meta,
                                        &{
                                            #[allow(unused_imports)]
                                            use ::tracing::field::{debug, display, Value};
                                            let mut iter = meta.fields().iter();
                                            meta.fields()
                                                .value_set(
                                                    &[
                                                        (
                                                            &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                .expect("FieldSet corrupted (this is a bug)"),
                                                            ::tracing::__macro_support::Option::Some(
                                                                &::tracing::field::debug(&request) as &dyn Value,
                                                            ),
                                                        ),
                                                    ],
                                                )
                                        },
                                    )
                                } else {
                                    let span = ::tracing::__macro_support::__disabled_span(
                                        __CALLSITE.metadata(),
                                    );
                                    if match ::tracing::Level::INFO {
                                        ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                        ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                        ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                        ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                        _ => ::tracing::log::Level::Trace,
                                    } <= ::tracing::log::STATIC_MAX_LEVEL
                                    {
                                        if !::tracing::dispatcher::has_been_set() {
                                            {
                                                span.record_all(
                                                    &{
                                                        #[allow(unused_imports)]
                                                        use ::tracing::field::{debug, display, Value};
                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                        __CALLSITE
                                                            .metadata()
                                                            .fields()
                                                            .value_set(
                                                                &[
                                                                    (
                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                        ::tracing::__macro_support::Option::Some(
                                                                            &::tracing::field::debug(&request) as &dyn Value,
                                                                        ),
                                                                    ),
                                                                ],
                                                            )
                                                    },
                                                );
                                            }
                                        } else {
                                            {}
                                        }
                                    } else {
                                        {}
                                    };
                                    span
                                }
                            };
                            let __tracing_instrument_future = async move {
                                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                                    Result<String, SandcastleError>,
                                > {
                                    #[allow(unreachable_code)] return __ret;
                                }
                                let __self = self;
                                let request = request;
                                let __ret: Result<String, SandcastleError> = {
                                    {
                                        use ::tracing::__macro_support::Callsite as _;
                                        static __CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                            static META: ::tracing::Metadata<'static> = {
                                                ::tracing_core::metadata::Metadata::new(
                                                    "event crates/sandcastle-core/src/domain/environment/services/github.rs:72",
                                                    "sandcastle_core::domain::environment::services::github",
                                                    ::tracing::Level::INFO,
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "crates/sandcastle-core/src/domain/environment/services/github.rs",
                                                    ),
                                                    ::tracing_core::__macro_support::Option::Some(72u32),
                                                    ::tracing_core::__macro_support::Option::Some(
                                                        "sandcastle_core::domain::environment::services::github",
                                                    ),
                                                    ::tracing_core::field::FieldSet::new(
                                                        &["message"],
                                                        ::tracing_core::callsite::Identifier(&__CALLSITE),
                                                    ),
                                                    ::tracing::metadata::Kind::EVENT,
                                                )
                                            };
                                            ::tracing::callsite::DefaultCallsite::new(&META)
                                        };
                                        let enabled = ::tracing::Level::INFO
                                            <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                            && ::tracing::Level::INFO
                                                <= ::tracing::level_filters::LevelFilter::current()
                                            && {
                                                let interest = __CALLSITE.interest();
                                                !interest.is_never()
                                                    && ::tracing::__macro_support::__is_enabled(
                                                        __CALLSITE.metadata(),
                                                        interest,
                                                    )
                                            };
                                        if enabled {
                                            (|value_set: ::tracing::field::ValueSet| {
                                                let meta = __CALLSITE.metadata();
                                                ::tracing::Event::dispatch(meta, &value_set);
                                                if match ::tracing::Level::INFO {
                                                    ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                    ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                    ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                    ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                    _ => ::tracing::log::Level::Trace,
                                                } <= ::tracing::log::STATIC_MAX_LEVEL
                                                {
                                                    if !::tracing::dispatcher::has_been_set() {
                                                        {
                                                            use ::tracing::log;
                                                            let level = match ::tracing::Level::INFO {
                                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                                _ => ::tracing::log::Level::Trace,
                                                            };
                                                            if level <= log::max_level() {
                                                                let meta = __CALLSITE.metadata();
                                                                let log_meta = log::Metadata::builder()
                                                                    .level(level)
                                                                    .target(meta.target())
                                                                    .build();
                                                                let logger = log::logger();
                                                                if logger.enabled(&log_meta) {
                                                                    ::tracing::__macro_support::__tracing_log(
                                                                        meta,
                                                                        logger,
                                                                        log_meta,
                                                                        &value_set,
                                                                    )
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        {}
                                                    }
                                                } else {
                                                    {}
                                                };
                                            })({
                                                #[allow(unused_imports)]
                                                use ::tracing::field::{debug, display, Value};
                                                let mut iter = __CALLSITE.metadata().fields().iter();
                                                __CALLSITE
                                                    .metadata()
                                                    .fields()
                                                    .value_set(
                                                        &[
                                                            (
                                                                &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                    .expect("FieldSet corrupted (this is a bug)"),
                                                                ::tracing::__macro_support::Option::Some(
                                                                    &format_args!("Fetching last commit SHA from GitHub")
                                                                        as &dyn Value,
                                                                ),
                                                            ),
                                                        ],
                                                    )
                                            });
                                        } else {
                                            if match ::tracing::Level::INFO {
                                                ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                _ => ::tracing::log::Level::Trace,
                                            } <= ::tracing::log::STATIC_MAX_LEVEL
                                            {
                                                if !::tracing::dispatcher::has_been_set() {
                                                    {
                                                        use ::tracing::log;
                                                        let level = match ::tracing::Level::INFO {
                                                            ::tracing::Level::ERROR => ::tracing::log::Level::Error,
                                                            ::tracing::Level::WARN => ::tracing::log::Level::Warn,
                                                            ::tracing::Level::INFO => ::tracing::log::Level::Info,
                                                            ::tracing::Level::DEBUG => ::tracing::log::Level::Debug,
                                                            _ => ::tracing::log::Level::Trace,
                                                        };
                                                        if level <= log::max_level() {
                                                            let meta = __CALLSITE.metadata();
                                                            let log_meta = log::Metadata::builder()
                                                                .level(level)
                                                                .target(meta.target())
                                                                .build();
                                                            let logger = log::logger();
                                                            if logger.enabled(&log_meta) {
                                                                ::tracing::__macro_support::__tracing_log(
                                                                    meta,
                                                                    logger,
                                                                    log_meta,
                                                                    &{
                                                                        #[allow(unused_imports)]
                                                                        use ::tracing::field::{debug, display, Value};
                                                                        let mut iter = __CALLSITE.metadata().fields().iter();
                                                                        __CALLSITE
                                                                            .metadata()
                                                                            .fields()
                                                                            .value_set(
                                                                                &[
                                                                                    (
                                                                                        &::tracing::__macro_support::Iterator::next(&mut iter)
                                                                                            .expect("FieldSet corrupted (this is a bug)"),
                                                                                        ::tracing::__macro_support::Option::Some(
                                                                                            &format_args!("Fetching last commit SHA from GitHub")
                                                                                                as &dyn Value,
                                                                                        ),
                                                                                    ),
                                                                                ],
                                                                            )
                                                                    },
                                                                )
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    {}
                                                }
                                            } else {
                                                {}
                                            };
                                        }
                                    };
                                    let repository = __self
                                        .client
                                        .repos_by_id(request.repository_id)
                                        .get()
                                        .await
                                        .map_err(|e| SandcastleError::Service {
                                            code: ServiceErrorCode::VCSFetchPRLastCommitSHARequest,
                                            message: e.to_string(),
                                            reason: request.repository_id.to_string(),
                                            backtrace: Backtrace::capture(),
                                        })?;
                                    let pr = __self
                                        .client
                                        .pulls(repository.owner.unwrap().login, repository.name)
                                        .get(request.pr_number)
                                        .await
                                        .map_err(|e| SandcastleError::Service {
                                            code: ServiceErrorCode::VCSFetchPRLastCommitSHARequest,
                                            message: e.to_string(),
                                            reason: ::alloc::__export::must_use({
                                                ::alloc::fmt::format(format_args!("{0:?}", e))
                                            }),
                                            backtrace: Backtrace::capture(),
                                        })?;
                                    Ok(pr.head.sha)
                                };
                                #[allow(unreachable_code)] __ret
                            };
                            if !__tracing_attr_span.is_disabled() {
                                ::tracing::Instrument::instrument(
                                        __tracing_instrument_future,
                                        __tracing_attr_span,
                                    )
                                    .await
                            } else {
                                __tracing_instrument_future.await
                            }
                        })
                    }
                }
            }
            use enum_dispatch::enum_dispatch;
            pub use gitops::*;
            use octocrab::Octocrab;
            pub use github::*;
            use crate::domain::environment::models::*;
            use crate::domain::environment::ports::*;
            use crate::domain::repositories::models::Authentication;
            use crate::domain::repositories::models::RepositoryConfiguration;
            use crate::error::SandcastleError;
            use crate::domain::environment::ports::MockVCSService as MockVCS;
            pub enum VCS {
                GitHub(GitHub),
                MockVCS(MockVCS),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for VCS {
                #[inline]
                fn clone(&self) -> VCS {
                    match self {
                        VCS::GitHub(__self_0) => {
                            VCS::GitHub(::core::clone::Clone::clone(__self_0))
                        }
                        VCS::MockVCS(__self_0) => {
                            VCS::MockVCS(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            impl ::core::convert::From<GitHub> for VCS {
                fn from(v: GitHub) -> VCS {
                    VCS::GitHub(v)
                }
            }
            impl ::core::convert::From<MockVCS> for VCS {
                fn from(v: MockVCS) -> VCS {
                    VCS::MockVCS(v)
                }
            }
            impl ::core::convert::TryInto<GitHub> for VCS {
                type Error = &'static str;
                fn try_into(
                    self,
                ) -> ::core::result::Result<
                    GitHub,
                    <Self as ::core::convert::TryInto<GitHub>>::Error,
                > {
                    match self {
                        VCS::GitHub(v) => Ok(v),
                        VCS::MockVCS(v) => {
                            Err("Tried to convert variant MockVCS to GitHub")
                        }
                    }
                }
            }
            impl ::core::convert::TryInto<MockVCS> for VCS {
                type Error = &'static str;
                fn try_into(
                    self,
                ) -> ::core::result::Result<
                    MockVCS,
                    <Self as ::core::convert::TryInto<MockVCS>>::Error,
                > {
                    match self {
                        VCS::MockVCS(v) => Ok(v),
                        VCS::GitHub(v) => {
                            Err("Tried to convert variant GitHub to MockVCS")
                        }
                    }
                }
            }
            impl VCSService for VCS {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn download_file<'life0, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: DownloadFileRequest,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<String, SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        VCS::GitHub(inner) => {
                            VCSService::download_file(inner, __enum_dispatch_arg_0)
                        }
                        VCS::MockVCS(inner) => {
                            VCSService::download_file(inner, __enum_dispatch_arg_0)
                        }
                    }
                }
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn fetch_pr_last_commit_sha<'life0, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: FetchPRLastCommitSHARequest,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<String, SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        VCS::GitHub(inner) => {
                            VCSService::fetch_pr_last_commit_sha(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                        VCS::MockVCS(inner) => {
                            VCSService::fetch_pr_last_commit_sha(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                    }
                }
            }
            impl TryFrom<RepositoryConfiguration> for VCS {
                type Error = SandcastleError;
                fn try_from(
                    value: RepositoryConfiguration,
                ) -> Result<Self, Self::Error> {
                    match &value.authentication {
                        Authentication::GitHubApp(_) => {
                            let octocrab = Octocrab::try_from(&value)?;
                            Ok(VCS::GitHub(GitHub::from(octocrab)))
                        }
                    }
                }
            }
            pub enum GitOpsPlatform {
                ArgoCD(ArgoCD),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for GitOpsPlatform {
                #[inline]
                fn clone(&self) -> GitOpsPlatform {
                    match self {
                        GitOpsPlatform::ArgoCD(__self_0) => {
                            GitOpsPlatform::ArgoCD(::core::clone::Clone::clone(__self_0))
                        }
                    }
                }
            }
            impl ::core::convert::From<ArgoCD> for GitOpsPlatform {
                fn from(v: ArgoCD) -> GitOpsPlatform {
                    GitOpsPlatform::ArgoCD(v)
                }
            }
            impl ::core::convert::TryInto<ArgoCD> for GitOpsPlatform {
                type Error = &'static str;
                fn try_into(
                    self,
                ) -> ::core::result::Result<
                    ArgoCD,
                    <Self as ::core::convert::TryInto<ArgoCD>>::Error,
                > {
                    match self {
                        GitOpsPlatform::ArgoCD(v) => Ok(v),
                    }
                }
            }
            impl GitOpsPlatformService for GitOpsPlatform {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn create_or_update_application<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<(), SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        GitOpsPlatform::ArgoCD(inner) => {
                            GitOpsPlatformService::create_or_update_application(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                    }
                }
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn delete_application<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<(), SandcastleError>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        GitOpsPlatform::ArgoCD(inner) => {
                            GitOpsPlatformService::delete_application(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                    }
                }
            }
        }
    }
    pub(crate) mod repositories {
        pub mod models {
            use std::backtrace::Backtrace;
            use octocrab::Octocrab;
            use snafu::ResultExt;
            use crate::error::{SandcastleError, ServiceErrorCode};
            pub struct RepositoryConfiguration {
                pub repository_url: String,
                pub authentication: Authentication,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for RepositoryConfiguration {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "RepositoryConfiguration",
                        "repository_url",
                        &self.repository_url,
                        "authentication",
                        &&self.authentication,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for RepositoryConfiguration {
                #[inline]
                fn clone(&self) -> RepositoryConfiguration {
                    RepositoryConfiguration {
                        repository_url: ::core::clone::Clone::clone(
                            &self.repository_url,
                        ),
                        authentication: ::core::clone::Clone::clone(&self.authentication),
                    }
                }
            }
            impl TryFrom<&RepositoryConfiguration> for Octocrab {
                type Error = SandcastleError;
                fn try_from(
                    value: &RepositoryConfiguration,
                ) -> Result<Self, Self::Error> {
                    match value.authentication.clone() {
                        Authentication::GitHubApp(auth) => {
                            let key = jsonwebtoken::EncodingKey::from_rsa_pem(
                                    auth.private_key.as_bytes(),
                                )
                                .whatever_context(
                                    ::alloc::__export::must_use({
                                        ::alloc::fmt::format(
                                            format_args!(
                                                "Failed to encode private key for GitHub app {0}",
                                                auth.app_id,
                                            ),
                                        )
                                    }),
                                )?;
                            let octocrab = Octocrab::builder()
                                .app(auth.app_id.into(), key)
                                .build()
                                .map_err(|e| SandcastleError::Service {
                                    code: ServiceErrorCode::GitHubAppAuthentication,
                                    message: e.to_string(),
                                    reason: auth.app_id.to_string(),
                                    backtrace: Backtrace::capture(),
                                })?;
                            let octocrab = octocrab
                                .installation(auth.installation_id.into())
                                .map_err(|e| SandcastleError::Service {
                                    code: ServiceErrorCode::GitHubAppAuthentication,
                                    message: e.to_string(),
                                    reason: auth.app_id.to_string(),
                                    backtrace: Backtrace::capture(),
                                })?;
                            Ok(octocrab)
                        }
                    }
                }
            }
            pub enum Authentication {
                GitHubApp(GitHubAppAuthentication),
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for Authentication {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match self {
                        Authentication::GitHubApp(__self_0) => {
                            ::core::fmt::Formatter::debug_tuple_field1_finish(
                                f,
                                "GitHubApp",
                                &__self_0,
                            )
                        }
                    }
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for Authentication {
                #[inline]
                fn clone(&self) -> Authentication {
                    match self {
                        Authentication::GitHubApp(__self_0) => {
                            Authentication::GitHubApp(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            pub struct GitHubAppAuthentication {
                pub app_id: u64,
                pub installation_id: u64,
                pub private_key: String,
            }
            #[automatically_derived]
            impl ::core::fmt::Debug for GitHubAppAuthentication {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "GitHubAppAuthentication",
                        "app_id",
                        &self.app_id,
                        "installation_id",
                        &self.installation_id,
                        "private_key",
                        &&self.private_key,
                    )
                }
            }
            #[automatically_derived]
            impl ::core::clone::Clone for GitHubAppAuthentication {
                #[inline]
                fn clone(&self) -> GitHubAppAuthentication {
                    GitHubAppAuthentication {
                        app_id: ::core::clone::Clone::clone(&self.app_id),
                        installation_id: ::core::clone::Clone::clone(
                            &self.installation_id,
                        ),
                        private_key: ::core::clone::Clone::clone(&self.private_key),
                    }
                }
            }
        }
        pub mod ports {
            use async_trait::async_trait;
            use enum_dispatch::enum_dispatch;
            use crate::{Result, domain::repositories::models::RepositoryConfiguration};
            /// A trait to fetch a repository's configuration
            pub trait RepositoryConfigurationService: Clone + Send + Sync {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn get_repository_configuration<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    repository_url: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<Option<RepositoryConfiguration>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn upsert_repository_configuration<'life0, 'async_trait>(
                    &'life0 self,
                    repository_configuration: RepositoryConfiguration,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait;
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                fn delete_repository_configuration<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    repository_url: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait;
            }
        }
        pub mod services {
            use std::{collections::HashMap, sync::Arc};
            use async_trait::async_trait;
            use enum_dispatch::enum_dispatch;
            use tokio::sync::RwLock;
            use crate::Result;
            use crate::domain::repositories::{
                models::RepositoryConfiguration, ports::RepositoryConfigurationService,
            };
            pub enum RepositoryConfigurations {
                DefaultRepositoryConfigurationService(
                    DefaultRepositoryConfigurationService,
                ),
            }
            #[automatically_derived]
            impl ::core::clone::Clone for RepositoryConfigurations {
                #[inline]
                fn clone(&self) -> RepositoryConfigurations {
                    match self {
                        RepositoryConfigurations::DefaultRepositoryConfigurationService(
                            __self_0,
                        ) => {
                            RepositoryConfigurations::DefaultRepositoryConfigurationService(
                                ::core::clone::Clone::clone(__self_0),
                            )
                        }
                    }
                }
            }
            impl ::core::convert::From<DefaultRepositoryConfigurationService>
            for RepositoryConfigurations {
                fn from(
                    v: DefaultRepositoryConfigurationService,
                ) -> RepositoryConfigurations {
                    RepositoryConfigurations::DefaultRepositoryConfigurationService(v)
                }
            }
            impl ::core::convert::TryInto<DefaultRepositoryConfigurationService>
            for RepositoryConfigurations {
                type Error = &'static str;
                fn try_into(
                    self,
                ) -> ::core::result::Result<
                    DefaultRepositoryConfigurationService,
                    <Self as ::core::convert::TryInto<
                        DefaultRepositoryConfigurationService,
                    >>::Error,
                > {
                    match self {
                        RepositoryConfigurations::DefaultRepositoryConfigurationService(
                            v,
                        ) => Ok(v),
                    }
                }
            }
            impl RepositoryConfigurationService for RepositoryConfigurations {
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn get_repository_configuration<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<Option<RepositoryConfiguration>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        RepositoryConfigurations::DefaultRepositoryConfigurationService(
                            inner,
                        ) => {
                            RepositoryConfigurationService::get_repository_configuration(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                    }
                }
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn upsert_repository_configuration<'life0, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: RepositoryConfiguration,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        RepositoryConfigurations::DefaultRepositoryConfigurationService(
                            inner,
                        ) => {
                            RepositoryConfigurationService::upsert_repository_configuration(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                    }
                }
                #[must_use]
                #[allow(
                    elided_named_lifetimes,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds
                )]
                #[inline]
                fn delete_repository_configuration<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    __enum_dispatch_arg_0: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    match self {
                        RepositoryConfigurations::DefaultRepositoryConfigurationService(
                            inner,
                        ) => {
                            RepositoryConfigurationService::delete_repository_configuration(
                                inner,
                                __enum_dispatch_arg_0,
                            )
                        }
                    }
                }
            }
            pub struct DefaultRepositoryConfigurationService {
                pub repository_configurations: Arc<
                    RwLock<HashMap<String, RepositoryConfiguration>>,
                >,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for DefaultRepositoryConfigurationService {
                #[inline]
                fn clone(&self) -> DefaultRepositoryConfigurationService {
                    DefaultRepositoryConfigurationService {
                        repository_configurations: ::core::clone::Clone::clone(
                            &self.repository_configurations,
                        ),
                    }
                }
            }
            impl Default for DefaultRepositoryConfigurationService {
                fn default() -> Self {
                    Self::new()
                }
            }
            impl DefaultRepositoryConfigurationService {
                pub fn new() -> Self {
                    Self {
                        repository_configurations: Arc::new(RwLock::new(HashMap::new())),
                    }
                }
            }
            impl RepositoryConfigurationService
            for DefaultRepositoryConfigurationService {
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn get_repository_configuration<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    repository_url: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<Option<RepositoryConfiguration>>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Result<Option<RepositoryConfiguration>>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let __ret: Result<Option<RepositoryConfiguration>> = {
                            Ok(
                                __self
                                    .repository_configurations
                                    .read()
                                    .await
                                    .get(repository_url)
                                    .cloned(),
                            )
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn upsert_repository_configuration<'life0, 'async_trait>(
                    &'life0 self,
                    repository_configuration: RepositoryConfiguration,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Result<()>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let repository_configuration = repository_configuration;
                        let __ret: Result<()> = {
                            __self
                                .repository_configurations
                                .write()
                                .await
                                .insert(
                                    repository_configuration.repository_url.clone(),
                                    repository_configuration,
                                );
                            Ok(())
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
                #[allow(
                    elided_named_lifetimes,
                    clippy::async_yields_async,
                    clippy::diverging_sub_expression,
                    clippy::let_unit_value,
                    clippy::needless_arbitrary_self_type,
                    clippy::no_effect_underscore_binding,
                    clippy::shadow_same,
                    clippy::type_complexity,
                    clippy::type_repetition_in_bounds,
                    clippy::used_underscore_binding
                )]
                fn delete_repository_configuration<'life0, 'life1, 'async_trait>(
                    &'life0 self,
                    repository_url: &'life1 str,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                            Output = Result<()>,
                        > + ::core::marker::Send + 'async_trait,
                    >,
                >
                where
                    'life0: 'async_trait,
                    'life1: 'async_trait,
                    Self: 'async_trait,
                {
                    Box::pin(async move {
                        if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                            Result<()>,
                        > {
                            #[allow(unreachable_code)] return __ret;
                        }
                        let __self = self;
                        let __ret: Result<()> = {
                            __self
                                .repository_configurations
                                .write()
                                .await
                                .remove(repository_url);
                            Ok(())
                        };
                        #[allow(unreachable_code)] __ret
                    })
                }
            }
        }
    }
}
mod error {
    use std::{backtrace::Backtrace, fmt::Display};
    use validator::ValidationErrors;
    pub enum ServiceErrorCode {
        HelmRepoAddFailed,
        HelmRepoIndexFailed,
        HelmChartNotFound,
        HelmChartVersionNotFound,
        HelmChartDownloadFailed,
        HelmInstallOrUpgradeFailed,
        HelmUninstallFailed,
        HelmReleaseStatusFailed,
        VCSFileDownloadFailed,
        VCSFileNotFound,
        InvalidConfiguration,
        SecretParsingFailed,
        VCSFetchPRLastCommitSHARequest,
        GitHubAppAuthentication,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ServiceErrorCode {
        #[inline]
        fn clone(&self) -> ServiceErrorCode {
            match self {
                ServiceErrorCode::HelmRepoAddFailed => {
                    ServiceErrorCode::HelmRepoAddFailed
                }
                ServiceErrorCode::HelmRepoIndexFailed => {
                    ServiceErrorCode::HelmRepoIndexFailed
                }
                ServiceErrorCode::HelmChartNotFound => {
                    ServiceErrorCode::HelmChartNotFound
                }
                ServiceErrorCode::HelmChartVersionNotFound => {
                    ServiceErrorCode::HelmChartVersionNotFound
                }
                ServiceErrorCode::HelmChartDownloadFailed => {
                    ServiceErrorCode::HelmChartDownloadFailed
                }
                ServiceErrorCode::HelmInstallOrUpgradeFailed => {
                    ServiceErrorCode::HelmInstallOrUpgradeFailed
                }
                ServiceErrorCode::HelmUninstallFailed => {
                    ServiceErrorCode::HelmUninstallFailed
                }
                ServiceErrorCode::HelmReleaseStatusFailed => {
                    ServiceErrorCode::HelmReleaseStatusFailed
                }
                ServiceErrorCode::VCSFileDownloadFailed => {
                    ServiceErrorCode::VCSFileDownloadFailed
                }
                ServiceErrorCode::VCSFileNotFound => ServiceErrorCode::VCSFileNotFound,
                ServiceErrorCode::InvalidConfiguration => {
                    ServiceErrorCode::InvalidConfiguration
                }
                ServiceErrorCode::SecretParsingFailed => {
                    ServiceErrorCode::SecretParsingFailed
                }
                ServiceErrorCode::VCSFetchPRLastCommitSHARequest => {
                    ServiceErrorCode::VCSFetchPRLastCommitSHARequest
                }
                ServiceErrorCode::GitHubAppAuthentication => {
                    ServiceErrorCode::GitHubAppAuthentication
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ServiceErrorCode {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    ServiceErrorCode::HelmRepoAddFailed => "HelmRepoAddFailed",
                    ServiceErrorCode::HelmRepoIndexFailed => "HelmRepoIndexFailed",
                    ServiceErrorCode::HelmChartNotFound => "HelmChartNotFound",
                    ServiceErrorCode::HelmChartVersionNotFound => {
                        "HelmChartVersionNotFound"
                    }
                    ServiceErrorCode::HelmChartDownloadFailed => {
                        "HelmChartDownloadFailed"
                    }
                    ServiceErrorCode::HelmInstallOrUpgradeFailed => {
                        "HelmInstallOrUpgradeFailed"
                    }
                    ServiceErrorCode::HelmUninstallFailed => "HelmUninstallFailed",
                    ServiceErrorCode::HelmReleaseStatusFailed => {
                        "HelmReleaseStatusFailed"
                    }
                    ServiceErrorCode::VCSFileDownloadFailed => "VCSFileDownloadFailed",
                    ServiceErrorCode::VCSFileNotFound => "VCSFileNotFound",
                    ServiceErrorCode::InvalidConfiguration => "InvalidConfiguration",
                    ServiceErrorCode::SecretParsingFailed => "SecretParsingFailed",
                    ServiceErrorCode::VCSFetchPRLastCommitSHARequest => {
                        "VCSFetchPRLastCommitSHARequest"
                    }
                    ServiceErrorCode::GitHubAppAuthentication => {
                        "GitHubAppAuthentication"
                    }
                },
            )
        }
    }
    impl Display for ServiceErrorCode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                ServiceErrorCode::HelmRepoAddFailed => {
                    f.write_fmt(format_args!("helm_repo_add_failed"))
                }
                ServiceErrorCode::HelmRepoIndexFailed => {
                    f.write_fmt(format_args!("helm_repo_index_failed"))
                }
                ServiceErrorCode::HelmChartNotFound => {
                    f.write_fmt(format_args!("helm_chart_not_found"))
                }
                ServiceErrorCode::HelmChartVersionNotFound => {
                    f.write_fmt(format_args!("helm_chart_version_not_found"))
                }
                ServiceErrorCode::HelmChartDownloadFailed => {
                    f.write_fmt(format_args!("helm_chart_download_failed"))
                }
                ServiceErrorCode::HelmInstallOrUpgradeFailed => {
                    f.write_fmt(format_args!("helm_install_or_upgrade_failed"))
                }
                ServiceErrorCode::HelmUninstallFailed => {
                    f.write_fmt(format_args!("helm_uninstall_failed"))
                }
                ServiceErrorCode::HelmReleaseStatusFailed => {
                    f.write_fmt(format_args!("helm_release_status_failed"))
                }
                ServiceErrorCode::VCSFileDownloadFailed => {
                    f.write_fmt(format_args!("vcs_file_download_failed"))
                }
                ServiceErrorCode::VCSFileNotFound => {
                    f.write_fmt(format_args!("vcs_file_not_found"))
                }
                ServiceErrorCode::VCSFetchPRLastCommitSHARequest => {
                    f.write_fmt(format_args!("vcs_fetch_pr_last_commit_sha_request"))
                }
                ServiceErrorCode::InvalidConfiguration => {
                    f.write_fmt(format_args!("invalid_configuration"))
                }
                ServiceErrorCode::SecretParsingFailed => {
                    f.write_fmt(format_args!("secret_parsing_failed"))
                }
                ServiceErrorCode::GitHubAppAuthentication => {
                    f.write_fmt(format_args!("github_app_authentication"))
                }
            }
        }
    }
    #[snafu(visibility(pub))]
    pub enum SandcastleError {
        #[snafu(display("{}: {}", message, source))]
        KubeClientError { message: String, source: kube::Error, backtrace: Backtrace },
        #[snafu(display("{}: {}", message, source))]
        Validation { message: String, source: ValidationErrors, backtrace: Backtrace },
        #[snafu(display("{}: {}", code, message))]
        Service {
            code: ServiceErrorCode,
            message: String,
            reason: String,
            backtrace: Backtrace,
        },
        #[snafu(whatever, display("{message}: {source:?}"))]
        Unexpected {
            message: String,
            #[snafu(source(from(Box<dyn std::error::Error+Send+Sync>, Some)))]
            source: Option<Box<dyn std::error::Error + Send + Sync>>,
            backtrace: Backtrace,
        },
        #[snafu(display("Finalizer error: {source}"))]
        Finalizer {
            #[snafu(
                source(from(kube::runtime::finalizer::Error<SandcastleError>, Box::new))
            )]
            source: Box<kube::runtime::finalizer::Error<SandcastleError>>,
            backtrace: Backtrace,
        },
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for SandcastleError {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                SandcastleError::KubeClientError {
                    message: __self_0,
                    source: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "KubeClientError",
                        "message",
                        __self_0,
                        "source",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                SandcastleError::Validation {
                    message: __self_0,
                    source: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Validation",
                        "message",
                        __self_0,
                        "source",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                SandcastleError::Service {
                    code: __self_0,
                    message: __self_1,
                    reason: __self_2,
                    backtrace: __self_3,
                } => {
                    ::core::fmt::Formatter::debug_struct_field4_finish(
                        f,
                        "Service",
                        "code",
                        __self_0,
                        "message",
                        __self_1,
                        "reason",
                        __self_2,
                        "backtrace",
                        &__self_3,
                    )
                }
                SandcastleError::Unexpected {
                    message: __self_0,
                    source: __self_1,
                    backtrace: __self_2,
                } => {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Unexpected",
                        "message",
                        __self_0,
                        "source",
                        __self_1,
                        "backtrace",
                        &__self_2,
                    )
                }
                SandcastleError::Finalizer { source: __self_0, backtrace: __self_1 } => {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "Finalizer",
                        "source",
                        __self_0,
                        "backtrace",
                        &__self_1,
                    )
                }
            }
        }
    }
    ///SNAFU context selector for the `SandcastleError::KubeClientError` variant
    pub struct KubeClientSnafu<__T0> {
        #[allow(missing_docs)]
        pub message: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for KubeClientSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "KubeClientSnafu",
                "message",
                &&self.message,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for KubeClientSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for KubeClientSnafu<__T0> {
        #[inline]
        fn clone(&self) -> KubeClientSnafu<__T0> {
            KubeClientSnafu {
                message: ::core::clone::Clone::clone(&self.message),
            }
        }
    }
    impl<__T0> ::snafu::IntoError<SandcastleError> for KubeClientSnafu<__T0>
    where
        SandcastleError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = kube::Error;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> SandcastleError {
            let error: kube::Error = (|v| v)(error);
            SandcastleError::KubeClientError {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: error,
                message: ::core::convert::Into::into(self.message),
            }
        }
    }
    ///SNAFU context selector for the `SandcastleError::Validation` variant
    pub struct ValidationSnafu<__T0> {
        #[allow(missing_docs)]
        pub message: __T0,
    }
    #[automatically_derived]
    impl<__T0: ::core::fmt::Debug> ::core::fmt::Debug for ValidationSnafu<__T0> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "ValidationSnafu",
                "message",
                &&self.message,
            )
        }
    }
    #[automatically_derived]
    impl<__T0: ::core::marker::Copy> ::core::marker::Copy for ValidationSnafu<__T0> {}
    #[automatically_derived]
    impl<__T0: ::core::clone::Clone> ::core::clone::Clone for ValidationSnafu<__T0> {
        #[inline]
        fn clone(&self) -> ValidationSnafu<__T0> {
            ValidationSnafu {
                message: ::core::clone::Clone::clone(&self.message),
            }
        }
    }
    impl<__T0> ::snafu::IntoError<SandcastleError> for ValidationSnafu<__T0>
    where
        SandcastleError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<String>,
    {
        type Source = ValidationErrors;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> SandcastleError {
            let error: ValidationErrors = (|v| v)(error);
            SandcastleError::Validation {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: error,
                message: ::core::convert::Into::into(self.message),
            }
        }
    }
    ///SNAFU context selector for the `SandcastleError::Service` variant
    pub struct ServiceSnafu<__T0, __T1, __T2> {
        #[allow(missing_docs)]
        pub code: __T0,
        #[allow(missing_docs)]
        pub message: __T1,
        #[allow(missing_docs)]
        pub reason: __T2,
    }
    #[automatically_derived]
    impl<
        __T0: ::core::fmt::Debug,
        __T1: ::core::fmt::Debug,
        __T2: ::core::fmt::Debug,
    > ::core::fmt::Debug for ServiceSnafu<__T0, __T1, __T2> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "ServiceSnafu",
                "code",
                &self.code,
                "message",
                &self.message,
                "reason",
                &&self.reason,
            )
        }
    }
    #[automatically_derived]
    impl<
        __T0: ::core::marker::Copy,
        __T1: ::core::marker::Copy,
        __T2: ::core::marker::Copy,
    > ::core::marker::Copy for ServiceSnafu<__T0, __T1, __T2> {}
    #[automatically_derived]
    impl<
        __T0: ::core::clone::Clone,
        __T1: ::core::clone::Clone,
        __T2: ::core::clone::Clone,
    > ::core::clone::Clone for ServiceSnafu<__T0, __T1, __T2> {
        #[inline]
        fn clone(&self) -> ServiceSnafu<__T0, __T1, __T2> {
            ServiceSnafu {
                code: ::core::clone::Clone::clone(&self.code),
                message: ::core::clone::Clone::clone(&self.message),
                reason: ::core::clone::Clone::clone(&self.reason),
            }
        }
    }
    impl<__T0, __T1, __T2> ServiceSnafu<__T0, __T1, __T2> {
        ///Consume the selector and return the associated error
        #[must_use]
        #[track_caller]
        pub fn build(self) -> SandcastleError
        where
            __T0: ::core::convert::Into<ServiceErrorCode>,
            __T1: ::core::convert::Into<String>,
            __T2: ::core::convert::Into<String>,
        {
            SandcastleError::Service {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                code: ::core::convert::Into::into(self.code),
                message: ::core::convert::Into::into(self.message),
                reason: ::core::convert::Into::into(self.reason),
            }
        }
        ///Consume the selector and return a `Result` with the associated error
        #[allow(dead_code)]
        #[track_caller]
        pub fn fail<__T>(self) -> ::core::result::Result<__T, SandcastleError>
        where
            __T0: ::core::convert::Into<ServiceErrorCode>,
            __T1: ::core::convert::Into<String>,
            __T2: ::core::convert::Into<String>,
        {
            ::core::result::Result::Err(self.build())
        }
    }
    impl<__T0, __T1, __T2> ::snafu::IntoError<SandcastleError>
    for ServiceSnafu<__T0, __T1, __T2>
    where
        SandcastleError: ::snafu::Error + ::snafu::ErrorCompat,
        __T0: ::core::convert::Into<ServiceErrorCode>,
        __T1: ::core::convert::Into<String>,
        __T2: ::core::convert::Into<String>,
    {
        type Source = ::snafu::NoneError;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> SandcastleError {
            SandcastleError::Service {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                code: ::core::convert::Into::into(self.code),
                message: ::core::convert::Into::into(self.message),
                reason: ::core::convert::Into::into(self.reason),
            }
        }
    }
    impl ::snafu::FromString for SandcastleError {
        type Source = Box<dyn std::error::Error + Send + Sync>;
        #[track_caller]
        fn without_source(message: String) -> Self {
            SandcastleError::Unexpected {
                backtrace: ::snafu::GenerateImplicitData::generate(),
                source: core::option::Option::None,
                message: message,
            }
        }
        #[track_caller]
        fn with_source(error: Self::Source, message: String) -> Self {
            SandcastleError::Unexpected {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: (Some)(error),
                message: message,
            }
        }
    }
    ///SNAFU context selector for the `SandcastleError::Finalizer` variant
    pub struct FinalizerSnafu;
    #[automatically_derived]
    impl ::core::fmt::Debug for FinalizerSnafu {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(f, "FinalizerSnafu")
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for FinalizerSnafu {}
    #[automatically_derived]
    impl ::core::clone::Clone for FinalizerSnafu {
        #[inline]
        fn clone(&self) -> FinalizerSnafu {
            *self
        }
    }
    impl ::snafu::IntoError<SandcastleError> for FinalizerSnafu
    where
        SandcastleError: ::snafu::Error + ::snafu::ErrorCompat,
    {
        type Source = kube::runtime::finalizer::Error<SandcastleError>;
        #[track_caller]
        fn into_error(self, error: Self::Source) -> SandcastleError {
            let error: Box<kube::runtime::finalizer::Error<SandcastleError>> = (Box::new)(
                error,
            );
            SandcastleError::Finalizer {
                backtrace: {
                    use ::snafu::AsErrorSource;
                    let error = error.as_error_source();
                    ::snafu::GenerateImplicitData::generate_with_source(error)
                },
                source: error,
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::core::fmt::Display for SandcastleError {
        fn fmt(
            &self,
            __snafu_display_formatter: &mut ::core::fmt::Formatter,
        ) -> ::core::fmt::Result {
            #[allow(unused_variables)]
            match *self {
                SandcastleError::KubeClientError {
                    ref backtrace,
                    ref message,
                    ref source,
                } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("{0}: {1}", message, source))
                }
                SandcastleError::Validation {
                    ref backtrace,
                    ref message,
                    ref source,
                } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("{0}: {1}", message, source))
                }
                SandcastleError::Service {
                    ref backtrace,
                    ref code,
                    ref message,
                    ref reason,
                } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("{0}: {1}", code, message))
                }
                SandcastleError::Unexpected {
                    ref backtrace,
                    ref message,
                    ref source,
                } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("{0}: {1:?}", message, source))
                }
                SandcastleError::Finalizer { ref backtrace, ref source } => {
                    __snafu_display_formatter
                        .write_fmt(format_args!("Finalizer error: {0}", source))
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::Error for SandcastleError
    where
        Self: ::core::fmt::Debug + ::core::fmt::Display,
    {
        fn description(&self) -> &str {
            match *self {
                SandcastleError::KubeClientError { .. } => {
                    "SandcastleError :: KubeClientError"
                }
                SandcastleError::Validation { .. } => "SandcastleError :: Validation",
                SandcastleError::Service { .. } => "SandcastleError :: Service",
                SandcastleError::Unexpected { .. } => "SandcastleError :: Unexpected",
                SandcastleError::Finalizer { .. } => "SandcastleError :: Finalizer",
            }
        }
        fn cause(&self) -> ::core::option::Option<&dyn ::snafu::Error> {
            use ::snafu::AsErrorSource;
            match *self {
                SandcastleError::KubeClientError { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SandcastleError::Validation { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SandcastleError::Service { .. } => ::core::option::Option::None,
                SandcastleError::Unexpected { ref source, .. } => {
                    source.as_ref().map(|e| e.as_error_source())
                }
                SandcastleError::Finalizer { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
            }
        }
        fn source(&self) -> ::core::option::Option<&(dyn ::snafu::Error + 'static)> {
            use ::snafu::AsErrorSource;
            match *self {
                SandcastleError::KubeClientError { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SandcastleError::Validation { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
                SandcastleError::Service { .. } => ::core::option::Option::None,
                SandcastleError::Unexpected { ref source, .. } => {
                    source.as_ref().map(|e| e.as_error_source())
                }
                SandcastleError::Finalizer { ref source, .. } => {
                    ::core::option::Option::Some(source.as_error_source())
                }
            }
        }
    }
    #[allow(single_use_lifetimes)]
    impl ::snafu::ErrorCompat for SandcastleError {
        fn backtrace(&self) -> ::core::option::Option<&::snafu::Backtrace> {
            match *self {
                SandcastleError::KubeClientError { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SandcastleError::Validation { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SandcastleError::Service { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SandcastleError::Unexpected { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
                SandcastleError::Finalizer { ref backtrace, .. } => {
                    ::snafu::AsBacktrace::as_backtrace(backtrace)
                }
            }
        }
    }
}
mod infrastructure {
    pub mod ports {}
    pub mod repo_config_service {
        use std::{collections::BTreeMap, sync::Arc};
        use k8s_openapi::api::core::v1::Secret;
        use crate::{
            domain::repositories::models::{
                Authentication, GitHubAppAuthentication, RepositoryConfiguration,
            },
            error::{SandcastleError, ServiceErrorCode},
        };
        pub struct GithubAppSecretData {
            pub url: String,
            pub app_id: u64,
            pub app_installation_id: u64,
            pub private_key: String,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for GithubAppSecretData {
            #[inline]
            fn clone(&self) -> GithubAppSecretData {
                GithubAppSecretData {
                    url: ::core::clone::Clone::clone(&self.url),
                    app_id: ::core::clone::Clone::clone(&self.app_id),
                    app_installation_id: ::core::clone::Clone::clone(
                        &self.app_installation_id,
                    ),
                    private_key: ::core::clone::Clone::clone(&self.private_key),
                }
            }
        }
        impl From<GithubAppSecretData> for RepositoryConfiguration {
            fn from(secret: GithubAppSecretData) -> Self {
                Self {
                    repository_url: secret.url,
                    authentication: Authentication::GitHubApp(GitHubAppAuthentication {
                        app_id: secret.app_id,
                        installation_id: secret.app_installation_id,
                        private_key: secret.private_key,
                    }),
                }
            }
        }
        impl GithubAppSecretData {
            pub fn from_secret(secret: Arc<Secret>) -> Result<Self, SandcastleError> {
                let mut merged_data = BTreeMap::new();
                if let Some(data) = secret.data.clone() {
                    for (key, value) in data {
                        let decoded = String::from_utf8(value.0)
                            .map_err(|e| SandcastleError::Service {
                                code: ServiceErrorCode::SecretParsingFailed,
                                message: ::alloc::__export::must_use({
                                    ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to decode base64 data for key: {0}",
                                            key,
                                        ),
                                    )
                                }),
                                reason: e.to_string(),
                                backtrace: std::backtrace::Backtrace::capture(),
                            })?;
                        merged_data.insert(key, decoded);
                    }
                }
                if let Some(string_data) = secret.string_data.clone() {
                    for (key, value) in string_data {
                        merged_data.insert(key, value);
                    }
                }
                let url = merged_data
                    .get("url")
                    .ok_or_else(|| SandcastleError::Service {
                        code: ServiceErrorCode::SecretParsingFailed,
                        message: "Missing 'url' field in secret".to_string(),
                        reason: "Required field 'url' not found in secret data"
                            .to_string(),
                        backtrace: std::backtrace::Backtrace::capture(),
                    })?
                    .clone();
                let app_id = merged_data
                    .get("app_id")
                    .ok_or_else(|| SandcastleError::Service {
                        code: ServiceErrorCode::SecretParsingFailed,
                        message: "Missing 'app_id' field in secret".to_string(),
                        reason: "Required field 'app_id' not found in secret data"
                            .to_string(),
                        backtrace: std::backtrace::Backtrace::capture(),
                    })?
                    .parse::<u64>()
                    .map_err(|e| SandcastleError::Service {
                        code: ServiceErrorCode::SecretParsingFailed,
                        message: "Failed to parse 'app_id' as u64".to_string(),
                        reason: e.to_string(),
                        backtrace: std::backtrace::Backtrace::capture(),
                    })?;
                let app_installation_id = merged_data
                    .get("app_installation_id")
                    .ok_or_else(|| SandcastleError::Service {
                        code: ServiceErrorCode::SecretParsingFailed,
                        message: "Missing 'app_installation_id' field in secret"
                            .to_string(),
                        reason: "Required field 'app_installation_id' not found in secret data"
                            .to_string(),
                        backtrace: std::backtrace::Backtrace::capture(),
                    })?
                    .parse::<u64>()
                    .map_err(|e| SandcastleError::Service {
                        code: ServiceErrorCode::SecretParsingFailed,
                        message: "Failed to parse 'app_installation_id' as u64"
                            .to_string(),
                        reason: e.to_string(),
                        backtrace: std::backtrace::Backtrace::capture(),
                    })?;
                let private_key = merged_data
                    .get("private_key")
                    .ok_or_else(|| SandcastleError::Service {
                        code: ServiceErrorCode::SecretParsingFailed,
                        message: "Missing 'private_key' field in secret".to_string(),
                        reason: "Required field 'private_key' not found in secret data"
                            .to_string(),
                        backtrace: std::backtrace::Backtrace::capture(),
                    })?
                    .clone();
                Ok(Self {
                    url,
                    app_id,
                    app_installation_id,
                    private_key,
                })
            }
        }
        mod tests {
            use super::*;
            use k8s_openapi::{ByteString, api::core::v1::Secret};
            use std::collections::BTreeMap;
            extern crate test;
            #[rustc_test_marker = "infrastructure::repo_config_service::tests::test_from_secret_only_data"]
            #[doc(hidden)]
            pub const test_from_secret_only_data: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "infrastructure::repo_config_service::tests::test_from_secret_only_data",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "crates/sandcastle-core/src/infrastructure/repo_config_service.rs",
                    start_line: 123usize,
                    start_col: 8usize,
                    end_line: 123usize,
                    end_col: 34usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::UnitTest,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(test_from_secret_only_data()),
                ),
            };
            fn test_from_secret_only_data() {
                let mut data = BTreeMap::new();
                data.insert(
                    "url".to_string(),
                    ByteString("https://github.com/test/repo.git".as_bytes().to_vec()),
                );
                data.insert(
                    "app_id".to_string(),
                    ByteString("12345".as_bytes().to_vec()),
                );
                data.insert(
                    "app_installation_id".to_string(),
                    ByteString("67890".as_bytes().to_vec()),
                );
                data.insert(
                    "private_key".to_string(),
                    ByteString(
                        "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
                            .as_bytes()
                            .to_vec(),
                    ),
                );
                let secret = Secret {
                    data: Some(data),
                    string_data: None,
                    ..Default::default()
                };
                let result = GithubAppSecretData::from_secret(Arc::new(secret));
                if !result.is_ok() {
                    ::core::panicking::panic("assertion failed: result.is_ok()")
                }
                let auth = result.unwrap();
                match (&auth.url, &"https://github.com/test/repo.git") {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (&auth.app_id, &12345) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (&auth.app_installation_id, &67890) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (
                    &auth.private_key,
                    &"-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----",
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
            }
            extern crate test;
            #[rustc_test_marker = "infrastructure::repo_config_service::tests::test_from_secret_only_string_data"]
            #[doc(hidden)]
            pub const test_from_secret_only_string_data: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "infrastructure::repo_config_service::tests::test_from_secret_only_string_data",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "crates/sandcastle-core/src/infrastructure/repo_config_service.rs",
                    start_line: 166usize,
                    start_col: 8usize,
                    end_line: 166usize,
                    end_col: 41usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::UnitTest,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(test_from_secret_only_string_data()),
                ),
            };
            fn test_from_secret_only_string_data() {
                let mut string_data = BTreeMap::new();
                string_data
                    .insert(
                        "url".to_string(),
                        "https://github.com/test/repo.git".to_string(),
                    );
                string_data.insert("app_id".to_string(), "12345".to_string());
                string_data
                    .insert("app_installation_id".to_string(), "67890".to_string());
                string_data
                    .insert(
                        "private_key".to_string(),
                        "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
                            .to_string(),
                    );
                let secret = Secret {
                    data: None,
                    string_data: Some(string_data),
                    ..Default::default()
                };
                let result = GithubAppSecretData::from_secret(Arc::new(secret));
                if !result.is_ok() {
                    ::core::panicking::panic("assertion failed: result.is_ok()")
                }
                let auth = result.unwrap();
                match (&auth.url, &"https://github.com/test/repo.git") {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (&auth.app_id, &12345) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (&auth.app_installation_id, &67890) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (
                    &auth.private_key,
                    &"-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----",
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
            }
            extern crate test;
            #[rustc_test_marker = "infrastructure::repo_config_service::tests::test_from_secret_mixed_data"]
            #[doc(hidden)]
            pub const test_from_secret_mixed_data: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "infrastructure::repo_config_service::tests::test_from_secret_mixed_data",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "crates/sandcastle-core/src/infrastructure/repo_config_service.rs",
                    start_line: 199usize,
                    start_col: 8usize,
                    end_line: 199usize,
                    end_col: 35usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::UnitTest,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(test_from_secret_mixed_data()),
                ),
            };
            fn test_from_secret_mixed_data() {
                let mut data = BTreeMap::new();
                data.insert(
                    "private_key".to_string(),
                    ByteString(
                        "-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----"
                            .as_bytes()
                            .to_vec(),
                    ),
                );
                let mut string_data = BTreeMap::new();
                string_data
                    .insert(
                        "url".to_string(),
                        "https://github.com/test/repo.git".to_string(),
                    );
                string_data.insert("app_id".to_string(), "12345".to_string());
                string_data
                    .insert("app_installation_id".to_string(), "67890".to_string());
                let secret = Secret {
                    data: Some(data),
                    string_data: Some(string_data),
                    ..Default::default()
                };
                let result = GithubAppSecretData::from_secret(Arc::new(secret));
                if !result.is_ok() {
                    ::core::panicking::panic("assertion failed: result.is_ok()")
                }
                let auth = result.unwrap();
                match (&auth.url, &"https://github.com/test/repo.git") {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (&auth.app_id, &12345) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (&auth.app_installation_id, &67890) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
                match (
                    &auth.private_key,
                    &"-----BEGIN RSA PRIVATE KEY-----\ntest_key\n-----END RSA PRIVATE KEY-----",
                ) {
                    (left_val, right_val) => {
                        if !(*left_val == *right_val) {
                            let kind = ::core::panicking::AssertKind::Eq;
                            ::core::panicking::assert_failed(
                                kind,
                                &*left_val,
                                &*right_val,
                                ::core::option::Option::None,
                            );
                        }
                    }
                };
            }
            extern crate test;
            #[rustc_test_marker = "infrastructure::repo_config_service::tests::test_from_secret_missing_field"]
            #[doc(hidden)]
            pub const test_from_secret_missing_field: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "infrastructure::repo_config_service::tests::test_from_secret_missing_field",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "crates/sandcastle-core/src/infrastructure/repo_config_service.rs",
                    start_line: 238usize,
                    start_col: 8usize,
                    end_line: 238usize,
                    end_col: 38usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::UnitTest,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(test_from_secret_missing_field()),
                ),
            };
            fn test_from_secret_missing_field() {
                let mut string_data = BTreeMap::new();
                string_data
                    .insert(
                        "url".to_string(),
                        "https://github.com/test/repo.git".to_string(),
                    );
                string_data.insert("app_id".to_string(), "12345".to_string());
                let secret = Secret {
                    data: None,
                    string_data: Some(string_data),
                    ..Default::default()
                };
                let result = GithubAppSecretData::from_secret(Arc::new(secret));
                if !result.is_err() {
                    ::core::panicking::panic("assertion failed: result.is_err()")
                }
                if let Err(SandcastleError::Service { code, message, .. }) = result {
                    if !#[allow(non_exhaustive_omitted_patterns)]
                    match code {
                        ServiceErrorCode::SecretParsingFailed => true,
                        _ => false,
                    } {
                        ::core::panicking::panic(
                            "assertion failed: matches!(code, ServiceErrorCode::SecretParsingFailed)",
                        )
                    }
                    if !message.contains("app_installation_id") {
                        ::core::panicking::panic(
                            "assertion failed: message.contains(\"app_installation_id\")",
                        )
                    }
                }
            }
            extern crate test;
            #[rustc_test_marker = "infrastructure::repo_config_service::tests::test_from_secret_invalid_app_id"]
            #[doc(hidden)]
            pub const test_from_secret_invalid_app_id: test::TestDescAndFn = test::TestDescAndFn {
                desc: test::TestDesc {
                    name: test::StaticTestName(
                        "infrastructure::repo_config_service::tests::test_from_secret_invalid_app_id",
                    ),
                    ignore: false,
                    ignore_message: ::core::option::Option::None,
                    source_file: "crates/sandcastle-core/src/infrastructure/repo_config_service.rs",
                    start_line: 262usize,
                    start_col: 8usize,
                    end_line: 262usize,
                    end_col: 39usize,
                    compile_fail: false,
                    no_run: false,
                    should_panic: test::ShouldPanic::No,
                    test_type: test::TestType::UnitTest,
                },
                testfn: test::StaticTestFn(
                    #[coverage(off)]
                    || test::assert_test_result(test_from_secret_invalid_app_id()),
                ),
            };
            fn test_from_secret_invalid_app_id() {
                let mut string_data = BTreeMap::new();
                string_data
                    .insert(
                        "url".to_string(),
                        "https://github.com/test/repo.git".to_string(),
                    );
                string_data.insert("app_id".to_string(), "not_a_number".to_string());
                string_data
                    .insert("app_installation_id".to_string(), "67890".to_string());
                string_data.insert("private_key".to_string(), "test_key".to_string());
                let secret = Secret {
                    data: None,
                    string_data: Some(string_data),
                    ..Default::default()
                };
                let result = GithubAppSecretData::from_secret(Arc::new(secret));
                if !result.is_err() {
                    ::core::panicking::panic("assertion failed: result.is_err()")
                }
                if let Err(SandcastleError::Service { code, message, .. }) = result {
                    if !#[allow(non_exhaustive_omitted_patterns)]
                    match code {
                        ServiceErrorCode::SecretParsingFailed => true,
                        _ => false,
                    } {
                        ::core::panicking::panic(
                            "assertion failed: matches!(code, ServiceErrorCode::SecretParsingFailed)",
                        )
                    }
                    if !message.contains("app_id") {
                        ::core::panicking::panic(
                            "assertion failed: message.contains(\"app_id\")",
                        )
                    }
                }
            }
        }
    }
}
pub type Result<T, E = SandcastleError> = std::result::Result<T, E>;
#[rustc_main]
#[coverage(off)]
#[doc(hidden)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(
        &[
            &test_from_string,
            &test_get_custom_value,
            &test_large_template,
            &test_small_template,
            &test_from_secret_invalid_app_id,
            &test_from_secret_missing_field,
            &test_from_secret_mixed_data,
            &test_from_secret_only_data,
            &test_from_secret_only_string_data,
        ],
    )
}
