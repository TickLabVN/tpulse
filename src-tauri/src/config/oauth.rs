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
    static ref OAUTH_CONFIG: OAuthConfig = OAuthConfig::new(
        "68238066373-iaiepboevrqvu1q6hp7gcirvro06rgvg.apps.googleusercontent.com".to_string(),
        "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".to_string(),
        "http://localhost:5917".to_string(),
        "https://accounts.google.com/o/oauth2/auth".to_string(),
        "https://accounts.google.com/o/oauth2/token".to_string(),
        vec!["email".to_string()]
    );
}

pub fn oauth() -> &'static OAuthConfig {
    &OAUTH_CONFIG
}

impl OAuthConfig {
    fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        auth_url: String,
        token_url: String,
        scopes: Vec<String>,
    ) -> Self {
        OAuthConfig {
            client_id,
            client_secret,
            redirect_uri,
            auth_url,
            token_url,
            scopes,
        }
    }
}
