pub mod typestate_examples {
    use serde::Deserialize;
    use std::marker::PhantomData;

    // For simplification purposes this is just a string here
    pub struct UrlEncoded(String);

    #[derive(Deserialize)]
    pub enum ContentType {
        UrlEncoded,
        Json,
        NotSet,
    }
    // Example of building a simplified version for an http builder pattern with 'content-type' and 'body'(String for simplification)
    pub struct HttpBuilder {
        content_type: ContentType,
        body: String,
    }
    impl HttpBuilder {
        pub fn new() -> Self {
            HttpBuilder {
                content_type: ContentType::NotSet,
                body: String::default(),
            }
        }
        pub fn add_url_content_type(mut self) -> Self {
            self.content_type = ContentType::UrlEncoded;
            self
        }
        pub fn _add_json_content_type(mut self) -> Self {
            self.content_type = ContentType::Json;
            self
        }
        pub fn _add_url_body(mut self, body: UrlEncoded) -> Self {
            self.body = body.0;
            self
        }
        pub fn add_json_body(mut self, body: serde_json::Value) -> Self {
            self.body = body.to_string();
            self
        }
    }

    pub fn bad_http_builder_example() {
        let body = serde_json::json!({
            "name": "Max",
            "age": 43
        });
        // With normal builder pattern the following is possible
        // The content type is url encodede but the body is json. To prevent this we have to add checks at **Runtime** to handle incorrect states
        let _http = HttpBuilder::new()
            .add_url_content_type()
            .add_json_body(body);
    }
    // We can prevent incorrect behavior at **Compile Time** with Typestate patterns

    // Setup Typestate structure (Note that these are zero sized and have no performance/memory overhead at runtime)
    // They only exist at compile time (including PhantomData)
    // -------------------------------
    pub trait TypestateTrait {}

    pub struct Empty;
    impl TypestateTrait for Empty {}

    pub struct ContentTypeUrl;
    impl TypestateTrait for ContentTypeUrl {}

    pub struct ContentTypeJson;
    impl TypestateTrait for ContentTypeJson {}

    pub struct BodyUrl;
    impl TypestateTrait for BodyUrl {}

    pub struct BodyJson;
    impl TypestateTrait for BodyJson {}

    // ---------------------------------

    pub struct BetterHttpBuilder<T: TypestateTrait> {
        content_type: ContentType,
        body: String,
        // Necessary for unused generic specifiers (and unused lifetime specifiers)
        _marker: PhantomData<T>,
    }
    impl BetterHttpBuilder<Empty> {
        pub fn new() -> BetterHttpBuilder<Empty> {
            BetterHttpBuilder {
                content_type: ContentType::NotSet,
                body: String::default(),
                _marker: PhantomData,
            }
        }
    }
    // Functions that are only valid in the [`Empty`] State
    impl BetterHttpBuilder<Empty> {
        pub fn add_url_content_type(self) -> BetterHttpBuilder<ContentTypeUrl> {
            BetterHttpBuilder {
                content_type: ContentType::UrlEncoded,
                body: String::default(),
                _marker: PhantomData,
            }
        }
        pub fn add_json_content_type(self) -> BetterHttpBuilder<ContentTypeJson> {
            BetterHttpBuilder {
                content_type: ContentType::Json,
                body: String::default(),
                _marker: PhantomData,
            }
        }
    }

    // Functions that are only valid in the [`ContentTypeUrl`] State
    impl BetterHttpBuilder<ContentTypeUrl> {
        pub fn add_url_body(self, body: UrlEncoded) -> BetterHttpBuilder<BodyUrl> {
            BetterHttpBuilder {
                content_type: self.content_type,
                body: body.0,
                _marker: PhantomData,
            }
        }
    }

    // Functions that are only valid in the [`ContentTypeJson`] State
    impl BetterHttpBuilder<ContentTypeJson> {
        pub fn add_json_body(self, body: serde_json::Value) -> BetterHttpBuilder<BodyJson> {
            BetterHttpBuilder {
                content_type: self.content_type,
                body: body.to_string(),
                _marker: PhantomData,
            }
        }
    }
    pub fn good_http_builder_example() {
        let body = serde_json::json!({
            "name": "Max",
            "age": 43
        });

        // Now the following won't even compile. No runtime checks needed because we can verify at compile time that the [`BetterHttpBuilder`] struct behaves as intended.
        //
        //  let _http = BetterHttpBuilder::new()
        //      .add_url_content_type()
        //      .add_json_body(body);
        
        // These are the only viable options at compile time.
        let _http = BetterHttpBuilder::new().add_json_content_type().add_json_body(body);
        let _http = BetterHttpBuilder::new().add_url_content_type().add_url_body(UrlEncoded(String::default()));
    }
}
