use lazy_static::lazy_static;

pub struct OAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub auth_url: String,
    pub token_url: String,
    pub scopes: Vec<String>,
}

lazy_static! {
    static ref OAUTH_CONFIG: OAuthConfig = OAuthConfig {
        client_id: "1082099244394-i78b3kar46unfrh2702mi3n4dngpcugj.apps.googleusercontent.com"
            .to_string(),
        client_secret: "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".to_string(),
        redirect_uri: "http://localhost:25794".to_string(),
        auth_url: "https://accounts.google.com/o/oauth2/auth".to_string(),
        token_url: "https://accounts.google.com/o/oauth2/token".to_string(),
        scopes: vec!["email".to_string()],
    };
}
pub fn get_oauth_config() -> &'static OAuthConfig {
    &OAUTH_CONFIG
}
