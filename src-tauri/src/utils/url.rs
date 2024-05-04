use url::Url;

use std::borrow::Cow;

pub struct Params<'a> {
    params: Vec<(&'a str, Cow<'a, str>)>,
}

impl<'a> Params<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            params: Vec::with_capacity(capacity),
        }
    }

    pub fn push<I: Into<Cow<'a, str>>>(&mut self, param: &'a str, value: I) {
        self.params.push((param, value.into()))
    }

    pub fn extend<I: Iterator<Item = (&'a String, IC)>, IC: Into<Cow<'a, str>>>(
        &mut self,
        params: I,
    ) {
        self.params
            .extend(params.map(|(k, v)| (k.as_str(), v.into())))
    }

    pub fn parse_with_url(&self, url: &str) -> Url {
        Url::parse_with_params(url, &self.params).unwrap()
    }
}
