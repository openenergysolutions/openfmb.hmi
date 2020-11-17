
use elasticsearch::{
    auth::Credentials, http::transport::Transport, params::Refresh, DeleteParts, Elasticsearch, IndexParts, SearchParts,
};



use serde_json::{json};
use snafu::{Snafu};
use tokio::runtime::Runtime;


use crate::actors::coordinator::CoordinatorMsg;
use crate::{messages::*};


use riker::actors::*;

#[actor(RequestActorStats, OpenFMBMessage)]
#[derive(Debug)]
pub struct ElasticSearchPublisher {
    message_count: u32,
    elasticsearch_client: Elasticsearch, //openfmb_file_subscriber: Option<OpenFMBFileSubscriber>,
    runtime: Runtime,
}

impl Actor for ElasticSearchPublisher {
    type Msg = ElasticSearchPublisherMsg;

    fn pre_start(&mut self, _ctx: &Context<Self::Msg>) {
        self.runtime.block_on(
            self.elasticsearch_client
                .delete(DeleteParts::IndexId("by_message", "*"))
                .send(),
        );
        // let response = self.elasticsearch_client.create(CreateParts::IndexId("by_time", "timestamp"))
        //     .body("foo")
        //     .send();
        // Runtime::new().context(RuntimeError).unwrap().block_on(response);
        // //        let subscribers = vec![ctx
        //            .actor_of(Props::new(OpenFMBFileSubscriber::actor), "OpenFMBFileSubscriber")
        //            .unwrap()];
        //        let publisher = vec![ctx
        //            .actor_of(Props::new(OpenFMBFilePublisher::actor), "OpenFMBFilePublisher")
        //            .unwrap()];
    }

    fn post_start(&mut self, _ctx: &Context<Self::Msg>) {}

    fn post_stop(&mut self) {}

    fn supervisor_strategy(&self) -> Strategy { Strategy::Restart }

    fn sys_recv(&mut self, _ctx: &Context<Self::Msg>, _msg: SystemMsg, _sender: Option<BasicActorRef>) {}

    fn recv(&mut self, ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Option<BasicActorRef>) {
        self.message_count += 1;
        self.receive(ctx, msg, sender);
    }
}

impl Receive<RequestActorStats> for ElasticSearchPublisher {
    type Msg = ElasticSearchPublisherMsg;

    fn receive(&mut self, ctx: &Context<Self::Msg>, _msg: RequestActorStats, sender: Sender) {
        let stats_msg: CoordinatorMsg = ActorStats {
            message_count: self.message_count,
            persisted_message_count: None,
        }
        .into();
        sender
            .clone()
            .unwrap()
            .try_tell(stats_msg, Some(ctx.myself.clone().into()))
            .unwrap();
    }
}

impl Receive<OpenFMBMessage> for ElasticSearchPublisher {
    type Msg = ElasticSearchPublisherMsg;

    fn receive(&mut self, _ctx: &Context<Self::Msg>, msg: OpenFMBMessage, _sender: Sender) {
        self.runtime.block_on(
            self.elasticsearch_client
                .index(IndexParts::IndexId(
                    "by_message",
                    &msg.message_mrid().unwrap().to_string(),
                ))
                .body(json!({
                    "message_mrid": msg.message_mrid().unwrap(),
                    "device_mrid": msg.device_mrid().unwrap(),
                    "device_name": msg.device_name().unwrap(),
                    "@timestamp": &datetime_from_timestamp(msg.message_timestamp().unwrap()).timestamp(),                    // "post_date": "2009-11-15T00:00:00Z",
                    "openfmb_payload": msg
                }))
                .send(),
        );
        self.runtime.block_on(
            self.elasticsearch_client
                .index(IndexParts::IndexId(
                    "by_deviceid",
                    &msg.device_mrid().unwrap().to_string(),
                ))
                .body(json!({
                    "message_mrid": msg.message_mrid().unwrap(),
                    "device_mrid": msg.device_mrid().unwrap(),
                    "device_name": msg.device_name().unwrap(),
                    "@timestamp": &datetime_from_timestamp(msg.message_timestamp().unwrap()).timestamp(),                    // "post_date": "2009-11-15T00:00:00Z",
                    "openfmb_payload": msg
                }))
                .send(),
        );
        self.runtime.block_on(
            self.elasticsearch_client
                .index(IndexParts::IndexId("by_message_type", &msg.message_type()))
                .body(json!({
                    "message_mrid": msg.message_mrid().unwrap(),
                    "message_type": msg.message_type(),
                    "device_mrid": msg.device_mrid().unwrap(),
                    "device_name": msg.device_name().unwrap(),
                    "@timestamp": &datetime_from_timestamp(msg.message_timestamp().unwrap()).timestamp(),                    // "post_date": "2009-11-15T00:00:00Z",
                    "openfmb_payload": msg
                }))
                .send(),
        );
    }
}

//---------------------------------------------------------------------
//---------------------------------------------------------------------
//---------------------------------------------------------------------
//---------------------------------------------------------------------
//---------------------------------------------------------------------
//---------------------------------------------------------------------
//---------------------------------------------------------------------

// async fn my_async() -> Result<(), Error> {
//     {
//         let client = Elasticsearch::default();
//
//         // make a search API call
//         let search_response = client
//             .search(SearchParts::None)
//             .body(json!({
//                 "query": {
//                     "match_all": {}
//                 }
//             }))
//             .allow_no_indices(true)
//             .send()
//             .await
//             .context(ElasticError)?;
//
//         // get the HTTP response status code
//         let status_code = search_response.status_code();
//
//         // read the response body. Consumes search_response
//         let response_body = search_response.read_body::<Value>().await.context(ElasticError)?;
//
//         // read fields from the response body
//         let took = response_body["took"].as_i64().unwrap();
//
//         let index_response = client
//             .index(IndexParts::IndexId("tweets", "1"))
//             .body(json!({
//                 "user": "kimchy",
//                 "post_date": "2009-11-15T00:00:00Z",
//                 "message": "Trying out Elasticsearch, so far so good?"
//             }))
//             .refresh(Refresh::WaitFor)
//             .send()
//             .await
//             .context(ElasticError)?;
//         if !index_response.status_code().is_success() {
//             panic!("indexing document failed")
//         }
//         let index_response = client
//             .index(IndexParts::IndexId("tweets", "2"))
//             .body(json!({
//                 "user": "forloop",
//                 "post_date": "2020-01-08T00:00:00Z",
//                 "message": "Indexing with the rust client, yeah!"
//             }))
//             .refresh(Refresh::WaitFor)
//             .send()
//             .await
//             .context(ElasticError)?;
//         if !index_response.status_code().is_success() {
//             panic!("indexing document failed")
//         }
//         Ok(())
//     }
// }
#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Elastic Error"))]
    ElasticError { source: elasticsearch::Error },
    #[snafu(display("Tokio Runtime Error"))]
    RuntimeError { source: std::io::Error },
}
