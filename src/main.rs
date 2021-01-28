use serde::{Deserialize, Serialize};
use kube_derive::CustomResource;
use schemars::JsonSchema; // TODO: what's this for?
use kube::{Client, api::{Api, ListParams}};
use tokio::time::Duration;
use kube_runtime::controller::{Context, Controller, ReconcilerAction};
use snafu::{Backtrace, OptionExt, ResultExt, Snafu};
use futures::StreamExt;

// use k8s_openapi::api::core::v1::ConfigMap;

#[derive(Debug, Snafu)]
enum Error {}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
pub struct Currency {
    pub code: String,
    pub net_debit_cap: u128,
}

#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema)]
#[kube(group = "kubernetes.mojaloop.io", version = "v1", kind = "Participant", namespaced)]
pub struct ParticipantSpec {
    pub name: String,
    pub currencies: Vec<Currency>,
}

/// Triggered when Participant or children change
async fn reconcile(p: Participant, _ctx: Context<()>) -> Result<ReconcilerAction, Error> {
    // .. use api here to reconcile a child ConfigMap with ownerreferences
    // see configmapgen_controller example for full info
    println!("{:?}", p);
    Ok(ReconcilerAction {
        requeue_after: Some(Duration::from_secs(300)),
    })
}

/// an error handler that will be called when the reconciler fails
fn error_policy(_error: &Error, _ctx: Context<()>) -> ReconcilerAction {
    ReconcilerAction {
        requeue_after: Some(Duration::from_secs(60)),
    }
}

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;
    let context = Context::new(()); // bad empty context - put client in here
    let ps = Api::<Participant>::all(client.clone());
    Controller::new(ps, ListParams::default())
        .run(reconcile, error_policy, context)
        .for_each(|res| async move {
            match res {
                Ok(o) => println!("reconciled {:?}", o),
                Err(e) => println!("reconcile failed: {:?}", e),
            }
        })
        .await; // controller does nothing unless polled
    Ok(())
}
