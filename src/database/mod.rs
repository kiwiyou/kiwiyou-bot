use crate::i18n;
use lazy_static::lazy_static;
use rusoto_core::RusotoError;
use rusoto_dynamodb::{
    AttributeValue, DynamoDb, DynamoDbClient, GetItemError, GetItemInput, PutItemError,
    PutItemInput,
};
use std::collections::HashMap;

lazy_static! {
    pub static ref DB: DynamoDbClient = {
        let var = std::env::var("REGION").expect("REGION is not set.");
        let region = var.parse().expect("REGION is invalid.");
        DynamoDbClient::new(region)
    };
}

pub async fn get_language(
    chat_id: i64,
) -> Result<Option<i18n::LanguageKind>, RusotoError<GetItemError>> {
    let result = DB
        .get_item(GetItemInput {
            table_name: "ChatLanguage".into(),
            key: {
                let mut key_map = HashMap::new();
                key_map.insert(
                    "chatId".into(),
                    AttributeValue {
                        n: Some(chat_id.to_string()),
                        ..Default::default()
                    },
                );
                key_map
            },
            ..Default::default()
        })
        .await?
        .item
        .and_then(|map| {
            map.get("language")
                .and_then(|attr| attr.s.as_ref().and_then(|s| s.parse().ok()))
        });
    Ok(result)
}

pub async fn set_language(
    chat_id: i64,
    language: i18n::LanguageKind,
) -> Result<(), RusotoError<PutItemError>> {
    DB.put_item(PutItemInput {
        table_name: "ChatLanguage".into(),
        item: {
            let mut item_map = HashMap::new();
            item_map.insert(
                "chatId".into(),
                AttributeValue {
                    n: Some(chat_id.to_string()),
                    ..Default::default()
                },
            );
            item_map.insert(
                "language".into(),
                AttributeValue {
                    s: Some(language.canonical_name().into()),
                    ..Default::default()
                },
            );
            item_map
        },
        ..Default::default()
    })
    .await
    .map(|_| ())
}
