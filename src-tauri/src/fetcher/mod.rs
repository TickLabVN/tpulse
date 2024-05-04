use std::{collections::HashMap, future::Future, pin::Pin, thread::sleep};

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};

use crate::utils::url::Params;

mod delegate;
mod error;
use delegate::{DefaultDelegate, Delegate, MethodInfo, Retry};
use error::Error;

type Result<T> = std::result::Result<T, Error>;
type GetTokenOutput =
    Pin<Box<dyn Future<Output = std::result::Result<Option<String>, Box<dyn std::error::Error>>>>>;

pub trait GetToken {
    /// Called whenever an API call requires authentication via an oauth2 token.
    /// Returns `Ok(None)` if a token is not necessary - otherwise, returns an error
    /// indicating the reason why a token could not be produced.
    fn get_token(&self) -> GetTokenOutput;
}

pub struct YouTube {
    pub client: reqwest::Client,
    pub auth: Box<dyn GetToken>,
    _base_url: String,
}

impl<'a> YouTube {
    pub fn new<A: 'static + GetToken>(client: reqwest::Client, auth: A) -> YouTube {
        YouTube {
            client,
            auth: Box::new(auth),
            _base_url: "https://youtube.googleapis.com/".to_string(),
        }
    }

    pub fn video_categories(&'a self) -> VideoCategoryMethods<'a> {
        VideoCategoryMethods { hub: &self }
    }
}

pub struct VideoCategoryMethods<'a> {
    hub: &'a YouTube,
}

impl<'a> VideoCategoryMethods<'a> {
    pub fn list(&self, part: &Vec<String>) -> VideoCategoryListCall {
        VideoCategoryListCall {
            hub: self.hub,
            _part: part.clone(),
            _region_code: Default::default(),
            _id: Default::default(),
            _hl: Default::default(),
            _delegate: Default::default(),
            _additional_params: Default::default(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VideoCategoryListResponse {
    /// Etag of this resource.
    pub etag: Option<String>,
    /// Serialized EventId of the request which produced this response.
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
    /// A list of video categories that can be associated with YouTube videos. In this map, the video category ID is the map key, and its value is the corresponding videoCategory resource.
    pub items: Option<Vec<VideoCategory>>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategoryListResponse".
    pub kind: Option<String>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the next page in the result set.
    #[serde(rename = "nextPageToken")]
    pub next_page_token: Option<String>,
    /// General pagination information.
    #[serde(rename = "pageInfo")]
    pub page_info: Option<PageInfo>,
    /// The token that can be used as the value of the pageToken parameter to retrieve the previous page in the result set.
    #[serde(rename = "prevPageToken")]
    pub prev_page_token: Option<String>,
    /// no description provided
    #[serde(rename = "tokenPagination")]
    pub token_pagination: Option<TokenPagination>,
    /// The visitorId identifies the visitor.
    #[serde(rename = "visitorId")]
    pub visitor_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct VideoCategory {
    /// Etag of this resource.
    pub etag: Option<String>,
    /// The ID that YouTube uses to uniquely identify the video category.
    pub id: Option<String>,
    /// Identifies what kind of resource this is. Value: the fixed string "youtube#videoCategory".
    pub kind: Option<String>,
    /// The snippet object contains basic details about the video category, including its title.
    pub snippet: Option<VideoCategorySnippet>,
}

#[derive(Serialize, Deserialize)]
pub struct VideoCategorySnippet {
    /// no description provided
    pub assignable: Option<bool>,
    /// The YouTube channel that created the video category.
    #[serde(rename = "channelId")]
    pub channel_id: Option<String>,
    /// The video category's title.
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PageInfo {
    /// The number of results included in the API response.
    #[serde(rename = "resultsPerPage")]
    pub results_per_page: Option<i32>,
    /// The total number of results in the result set.
    #[serde(rename = "totalResults")]
    pub total_results: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct TokenPagination {
    _never_set: Option<bool>,
}

pub struct VideoCategoryListCall<'a> {
    hub: &'a YouTube,
    _part: Vec<String>,
    _region_code: Option<String>,
    _id: Vec<String>,
    _hl: Option<String>,
    _delegate: Option<&'a mut dyn Delegate>,
    _additional_params: HashMap<String, String>,
}

impl<'a> VideoCategoryListCall<'a> {
    pub async fn doit(self) -> Result<VideoCategoryListResponse> {
        use reqwest::header::AUTHORIZATION;

        let mut dd = DefaultDelegate;
        let dlg = self._delegate.unwrap_or(&mut dd);
        dlg.begin(MethodInfo {
            id: "youtube.videoCategories.list",
            http_method: reqwest::Method::GET,
        });

        for &field in ["alt", "part", "regionCode", "id", "hl"].iter() {
            if self._additional_params.contains_key(field) {
                dlg.finished(false);
                return Err(Error::FieldClash(field));
            }
        }

        let mut params = Params::with_capacity(6 + self._additional_params.len());
        if self._part.len() > 0 {
            for f in self._part.iter() {
                params.push("part", f);
            }
        }
        if let Some(value) = self._region_code.as_ref() {
            params.push("regionCode", value);
        }
        if self._id.len() > 0 {
            for f in self._id.iter() {
                params.push("id", f);
            }
        }
        if let Some(value) = self._hl.as_ref() {
            params.push("hl", value);
        }

        params.extend(self._additional_params.iter());

        params.push("alt", "json");
        let url = self.hub._base_url.clone() + "youtube/v3/videoCategories";

        let url = params.parse_with_url(&url);

        loop {
            let token = match self.hub.auth.get_token().await {
                Ok(token) => token,
                Err(e) => match dlg.token(e) {
                    Ok(token) => token,
                    Err(e) => {
                        dlg.finished(false);
                        return Err(Error::MissingToken(e));
                    }
                },
            };
            let req_result = {
                let client = &self.hub.client;
                dlg.pre_request();
                let mut req_builder = client.get(url.clone());

                if let Some(token) = token.as_ref() {
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
                    );
                    req_builder = req_builder.headers(headers);
                }

                req_builder.send().await
            };

            match req_result {
                Err(err) => {
                    if let Retry::After(d) = dlg.http_error(&err) {
                        sleep(d);
                        continue;
                    }
                    dlg.finished(false);
                    return Err(Error::HttpError(err));
                }
                Ok(res) => {
                    if !res.status().is_success() {
                        if let Retry::After(d) = dlg.http_failure(&res) {
                            sleep(d);
                            continue;
                        }

                        dlg.finished(false);

                        return Err(Error::Failure(res));
                    }
                    let result_value = {
                        match res.json::<VideoCategoryListResponse>().await {
                            Ok(decoded) => decoded,
                            Err(err) => {
                                dlg.response_json_decode_error(&err.to_string());
                                return Err(Error::JsonDecodeError(err.to_string()));
                            }
                        }
                    };

                    dlg.finished(true);
                    return Ok(result_value);
                }
            }
        }
    }

    pub fn add_part(mut self, new_value: &str) -> VideoCategoryListCall<'a> {
        self._part.push(new_value.to_string());
        self
    }
    pub fn region_code(mut self, new_value: &str) -> VideoCategoryListCall<'a> {
        self._region_code = Some(new_value.to_string());
        self
    }
    pub fn add_id(mut self, new_value: &str) -> VideoCategoryListCall<'a> {
        self._id.push(new_value.to_string());
        self
    }
    pub fn hl(mut self, new_value: &str) -> VideoCategoryListCall<'a> {
        self._hl = Some(new_value.to_string());
        self
    }
}
