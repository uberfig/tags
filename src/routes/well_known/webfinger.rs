use crate::{db::pg_conn::PgConn, types::webfinger::*};
use actix_web::{
    error::{ErrorBadRequest, ErrorNotFound},
    get,
    web::{self, Data},
    HttpResponse, Result,
};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug)]
pub struct WebfingerQuery {
    pub has_prefix: bool,
    pub preferred_username: Option<String>,
    pub domain: Option<String>,
}

impl WebfingerQuery {
    fn parse_query(input: String) -> Self {
        let resource = input.strip_prefix("acct:");

        let has_prefix;

        let resource = match resource {
            Some(x) => {
                has_prefix = true;
                x
            }
            None => {
                has_prefix = false;
                &input
            }
        };

        let mut vals = resource.split('@');
        let preferred_username = vals.next();
        let domain = vals.next();
        match preferred_username {
            Some(uname) => {
                if let Some(d) = domain {
                    WebfingerQuery {
                        has_prefix,
                        preferred_username: Some(uname.to_string()),
                        domain: Some(d.to_string()),
                    }
                } else {
                    WebfingerQuery {
                        has_prefix,
                        preferred_username: Some(uname.to_string()),
                        domain: None,
                    }
                }
            }
            None => WebfingerQuery {
                has_prefix,
                preferred_username: None,
                domain: None,
            },
        }
    }
}

#[derive(Deserialize, Debug)]
struct Info {
    resource: String,
}

#[get("/webfinger")]
async fn webfinger(
    state: Data<crate::config::Config>,
    conn: Data<PgConn>,
    info: web::Query<Info>,
) -> Result<HttpResponse> {
    let resource = info.into_inner().resource;
    let result = WebfingerQuery::parse_query(resource);

    if let Some(x) = result.domain {
        if !x.eq_ignore_ascii_case(&state.instance_domain) {
            return Err(ErrorBadRequest("not from this domain"));
        }
    }
    let preferred_username = match result.preferred_username {
        Some(x) => x,
        None => return Err(ErrorBadRequest("no preferred username provided")),
    };

    let actor = match preferred_username.eq("instance.actor") {
        //is the instance actor
        true => conn
            .get_instance_actor()
            .await
            .expect("instance actor not init")
            .to_actor(&state.instance_domain),
        //not the instance actor
        false => {
            if !preferred_username.chars().all(char::is_alphanumeric) {
                return Err(ErrorNotFound("preferred username not alphanumeric"));
            }
            let actor = conn.get_or_init_tag(&preferred_username).await;
            actor
        }
    };

    let subject = format!("acct:{}@{}", &preferred_username, &state.instance_domain);
    let profile_page = format!(
        "https://{}/@{}",
        &state.instance_domain, &preferred_username
    );

    let id = actor.id.as_str();

    let webfinger = WebfingerResult {
        subject,
        aliases: Some(vec![id.to_string(), profile_page.clone()]),
        links: vec![
            WebfingerLink {
                rel: RelWrap::Defined(RelTypes::RelSelf),
                type_field: TypeWrap::Defined(WebfingerLinkTypes::Activitypub),
                href: Url::parse(id).unwrap(),
            },
            // WebfingerLink {
            //     rel: "self".to_string(),
            //     type_field: "application/json".to_string(),
            //     href: format!("{}/versia", id),
            // },
            WebfingerLink {
                rel: RelWrap::Defined(RelTypes::ProfilePage),
                type_field: TypeWrap::Defined(WebfingerLinkTypes::Webpage),
                href: Url::parse(&profile_page).unwrap(),
            },
        ],
    };
    let webfinger = serde_json::to_string(&webfinger).unwrap();

    Ok(HttpResponse::Ok()
        .content_type("application/jrd+json; charset=utf-8")
        .body(webfinger))
}
