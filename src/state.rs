use crate::config::cqrs_framework;
use crate::domain::aggregate::BankAccount;
use crate::queries::BankAccountView;
use aws_sdk_dynamodb::Client;
use dynamo_es::{DynamoCqrs, DynamoViewRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct ApplicationState {
    pub cqrs: Arc<DynamoCqrs<BankAccount>>,
    pub account_query: Arc<DynamoViewRepository<BankAccountView, BankAccount>>,
}

pub async fn new_application_state() -> ApplicationState {
    // Configure the CQRS framework, backed by a dynamodb database, along with two queries:
    // - a simply-query prints events to stdout as they are published
    // - `account_query` stores the current state of the account in a ViewRepository that we can access
    //
    // The needed database tables are automatically configured with `docker-compose up -d`,
    // see init file at `/db/create_tables.sh` for more.
    let sdk_config: aws_config::SdkConfig = aws_config::load_from_env().await;
    let config = aws_sdk_dynamodb::config::Builder::from(&sdk_config)
        .endpoint_url( "http://localhost:8000")
        .build();

    let client = Client::from_conf(config);
    let (cqrs, account_query) = cqrs_framework(client);

    ApplicationState {
        cqrs,
        account_query,
    }
}
