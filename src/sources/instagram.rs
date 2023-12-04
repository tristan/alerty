use crate::{
    source_iter::{AlertSource, AlertSourceConfig},
    AlertData, AlertyError,
};
use cookie_store::CookieStore;
use scraper::{Html, Selector};
use serde::Deserialize;

pub struct Instagram {
    agent: ureq::Agent,
    username: String,
}

// pub const X_CSRF_TOKEN: &str = "X-CSRFToken";
// pub const CSRF_TOKEN_COOKIE: &str = "csrftoken";
pub const CHROME_WIN_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/77.0.3865.120 Safari/537.36";

#[derive(Deserialize)]
struct WebProfileInfo {
    data: WebProfileInfoData,
}

#[derive(Deserialize)]
struct WebProfileInfoData {
    user: WebProfileInfoDataUser,
}

#[derive(Deserialize)]
struct WebProfileInfoDataUser {
    // id: String,
    // fbid: String,
    // full_name: String,
    // username: String,
    edge_owner_to_timeline_media: EdgeOwnerToTimelineMedia,
}

#[derive(Deserialize)]
struct EdgeOwnerToTimelineMedia {
    edges: Vec<MediaEdge>,
}

#[derive(Deserialize)]
struct MediaEdge {
    node: MediaEdgeNode,
}

#[derive(Deserialize)]
struct MediaEdgeNode {
    // #[serde(rename="__typename")]
    // typename: String,
    id: String,
    shortcode: String,
    // display_url: String,
    // is_video: bool,
    thumbnail_src: String,
    edge_media_to_caption: EdgeMediaToCaption,
}

#[derive(Deserialize)]
struct EdgeMediaToCaption {
    edges: Vec<CaptionEdge>,
}

#[derive(Deserialize)]
struct CaptionEdge {
    node: CaptionEdgeNode,
}

#[derive(Deserialize)]
struct CaptionEdgeNode {
    text: String,
}

impl Instagram {
    // pub fn authorize() {
    //     // NOTE: not really needed if using testing accounts, just generate a token in the app settings
    //     // https://developers.facebook.com/docs/instagram-basic-display-api/getting-started
    //     let app_id = "1034021577933263";
    //     let secret = "ea2bf868e55d33d56bd3f42c02223ed7";
    //     let redirect_uri = "https://tristan.rs";
    //     let req = ureq::get("https://api.instagram.com/oauth/authorize")
    //         .query_pairs([
    //             ("client_id", app_id),
    //             ("redirect_uri", redirect_uri),
    //             ("scope", "user_profile,user_media,instagram_graph_user_profile"),
    //             ("response_type", "code")
    //         ]);
    //     let url = req.url();
    //     println!("Open the following url in a browser");
    //     println!("{url}");
    //     let mut code = None;
    //     while code.is_none() {
    //         code = read_line_from_stdin("After authorizing, enter the code in the url here").unwrap();
    //     }
    //     let code = code.unwrap();
    //     // get short lived access token
    //     let res = ureq::post("https://api.instagram.com/oauth/access_token")
    //         .send_form(&[
    //             ("client_id", app_id),
    //             ("client_secret", secret),
    //             ("grant_type", "authorization_code"),
    //             ("redirect_uri", redirect_uri),
    //             ("code", &code),
    //         ]);
    //     let token: ShortLivedAccessTokenResponse = res.unwrap().into_json().unwrap();
    //     // exchange for a long lived access token
    //     // https://developers.facebook.com/docs/instagram-basic-display-api/guides/long-lived-access-tokens/
    //     let res = ureq::get("https://graph.instagram.com/access_token")
    //         .query_pairs([
    //             ("grant_type", "ig_exchange_token"),
    //             ("client_secret", secret),
    //             ("access_token", &token.access_token),
    //         ])
    //         .call();
    //     let token: LongLivedAccessTokenResponse = res.unwrap().into_json().unwrap();
    //     dbg!(token);
    // }

    // fn fetch_using_access_token(&self) {
    //     // https://www.instagram.com/web/search/topsearch/?query=lowroar
    //     let user_id = "17841401874550850"; // lowroar
    //     let user_id = "17841400981774541"; // t.l.king
    //     let access_token = &self.access_token;
    //     let res = ureq::get(&format!("https://graph.instagram.com/?ids={user_id}&access_token={access_token}"))
    //         .call();

    //     let res = ureq::get(&format!("https://api.instagram.com/v1/users/search?q=lowroar&access_token={access_token}"))
    //         .call().unwrap().into_string().unwrap();

    //     let res = ureq::get(&format!("https://graph.instagram.com/v18.0/{user_id}"))
    //         .query_pairs([
    //             ("fields", "username,media"),
    //             ("access_token", access_token)
    //         ])
    //         .call();
    //     println!("{}", res.unwrap().into_string().unwrap());
    // }

    fn web_profile_info(&self, username: &str) -> Result<Vec<AlertData>, AlertyError> {
        let page_url = format!("https://www.instagram.com/{username}/");
        let res = self.agent.get(&page_url).set("Referer", &page_url).call()?;
        let page_html = res.into_string().unwrap();
        //println!("{page_html}");
        let doc = Html::parse_document(&page_html);
        let script_selector = Selector::parse("body > script[data-sjs]").unwrap();
        let mut selection = doc.select(&script_selector);
        let x_ig_app_id = 'outer: loop {
            let Some(script) = selection.next() else {
                return Err(AlertyError::other("Failed to find RelayAPIConfigDefaults"));
            };
            let text = script.text().collect::<String>();
            let Ok(serde_json::Value::Object(json)) =
                serde_json::from_str::<serde_json::Value>(&text)
            else {
                continue;
            };
            let Some(serde_json::Value::Array(require)) = json.get("require") else {
                continue;
            };
            for element in require {
                let serde_json::Value::Array(element) = element else {
                    continue;
                };
                let Some(serde_json::Value::String(key)) = element.first() else {
                    continue;
                };
                if key != "ScheduledServerJS" {
                    continue;
                }
                let Some(serde_json::Value::Array(args)) = element.get(3) else {
                    continue;
                };
                for arg in args {
                    let serde_json::Value::Object(arg) = arg else {
                        continue;
                    };
                    let Some(serde_json::Value::Object(bbox)) = arg.get("__bbox") else {
                        continue;
                    };
                    let Some(serde_json::Value::Array(define)) = bbox.get("define") else {
                        continue;
                    };
                    for element in define {
                        let serde_json::Value::Array(element) = element else {
                            continue;
                        };
                        let Some(serde_json::Value::String(key)) = element.first() else {
                            continue;
                        };
                        if key != "RelayAPIConfigDefaults" {
                            continue;
                        }
                        let Some(serde_json::Value::Object(kwargs)) = element.get(2) else {
                            continue;
                        };
                        let Some(serde_json::Value::Object(custom_headers)) =
                            kwargs.get("customHeaders")
                        else {
                            continue;
                        };
                        let Some(x_ig_app_id) = custom_headers.iter().find_map(|(k, v)| {
                            if k == "X-IG-App-ID" {
                                if let serde_json::Value::String(value) = v {
                                    Some(value.clone())
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        }) else {
                            continue;
                        };
                        break 'outer x_ig_app_id;
                    }
                }
            }
        };

        // let csrftoken = self.agent.cookie_store().iter_any().find_map(|c| {
        //     let (name, val) = c.name_value();
        //     dbg!(name, val);
        //     if name == CSRF_TOKEN_COOKIE {
        //         Some(val.to_string())
        //     } else {
        //         None
        //     }
        // }).ok_or_else(|| AlertyError::other("missing csrf token"))?;
        //return Ok(());
        let req = self
            .agent
            .get("https://www.instagram.com/api/v1/users/web_profile_info/")
            .set("Referer", &page_url)
            .set("X-IG-WWW-Claim", "0")
            .set("X-Requested-With", "XMLHttpRequest")
            .set("X-ASBD-ID", "129477")
            .set("X-IG-App-ID", &x_ig_app_id)
            //.set(X_CSRF_TOKEN, &csrftoken)
            .query("username", username);

        let res = req.call()?;
        let web_profile_info = res.into_json::<WebProfileInfo>()?;
        let data = web_profile_info
            .data
            .user
            .edge_owner_to_timeline_media
            .edges
            .into_iter()
            .map(|media_edge| {
                let node = media_edge.node;
                let texts = node
                    .edge_media_to_caption
                    .edges
                    .into_iter()
                    .map(|caption_edge| caption_edge.node.text)
                    .collect::<Vec<_>>();
                let shortcode = node.shortcode;
                // TODO: figure out thumbnails that aren't blocked by CORS
                AlertData {
                    id: node.id,
                    thumbnail: Some(node.thumbnail_src),
                    text: Some(texts.join("\n")),
                    link: Some(format!("https://www.instagram.com/p/{shortcode}/")),
                    ..Default::default()
                }
            })
            .collect::<Vec<_>>();
        //println!("{:?}", res.into_string().unwrap());

        Ok(data)
    }
}

#[derive(Deserialize)]
pub struct InstagramConfig {
    username: String,
}

impl AlertSourceConfig for InstagramConfig {
    type Source = Instagram;
    fn initialize_source(&self) -> Self::Source {
        let agent: ureq::Agent = ureq::AgentBuilder::new()
            .user_agent(CHROME_WIN_USER_AGENT)
            .cookie_store(CookieStore::new(None))
            .build();
        Self::Source {
            agent,
            username: self.username.clone(),
        }
    }
}

impl AlertSource for Instagram {
    fn id(&self) -> String {
        self.username.clone()
    }

    fn fetch(&self) -> Result<Vec<AlertData>, AlertyError> {
        self.web_profile_info(&self.username)
    }

    fn diff(&self, lhs: &AlertData, rhs: &AlertData) -> bool {
        lhs.id == rhs.id && lhs.link == rhs.link && lhs.title == rhs.title && lhs.text == rhs.text
    }
}
