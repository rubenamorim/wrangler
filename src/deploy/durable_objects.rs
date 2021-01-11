use crate::http;
use crate::settings::global_user::GlobalUser;

use serde::Serialize;

#[derive(Clone, Debug, PartialEq)]
pub struct DurableObjectNSTarget {
    pub account_id: String,
    pub script_name: String,
    pub class_name: String,
}

#[derive(Serialize)]
pub struct NamespaceUpsertRequest {
    pub name: String,
    pub script: String,
    pub class: String,
}

impl From<&DurableObjectNSTarget> for NamespaceUpsertRequest {
    fn from(target: &DurableObjectNSTarget) -> NamespaceUpsertRequest {
        NamespaceUpsertRequest {
            name: format!("{}-{}", target.script_name, target.class_name),
            script: target.script_name.clone(),
            class: target.class_name.clone(),
        }
    }
}

impl DurableObjectNSTarget {
    pub fn new(account_id: String, script_name: String, class_name: String) -> Self {
        Self {
            account_id,
            script_name,
            class_name,
        }
    }

    pub fn deploy(&self, user: &GlobalUser) -> Result<String, failure::Error> {
        log::info!("publishing durable object");
        let schedule_worker_addr = format!(
            "https://api.cloudflare.com/client/v4/accounts/{}/workers/durable_objects/namespaces",
            self.account_id,
        );

        let client = http::legacy_auth_client(user);
        let request_body: NamespaceUpsertRequest = self.into();

        let res = client
            .post(&schedule_worker_addr)
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&request_body)?)
            .send()?;

        if !res.status().is_success() {
            failure::bail!(
                "Something went wrong! Status: {}, Details {}",
                res.status(),
                res.text()?
            )
        }

        Ok(request_body.name)
    }
}
