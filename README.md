# akeyless-grpc-rust

 Rust library for interacting with the Akeyless API using gRPC

## Table of Contents

- [Introduction](#introduction)
- [Installation](#installation)
- [Example](#example)
- [License](#license)

## Introduction

The `akeyless_grpc` library provides a convenient way to interact with the Akeyless API using gRPC.

## Installation

To use `akeyless_grpc` in your project, run the following command `cargo add akeyless_grpc`

## Example

```rust
use akeyless_grpc::akeyless_v2_service_client::AkeylessV2ServiceClient;
use akeyless_grpc::AuthRequest;
use akeyless_grpc::Auth;
use akeyless_grpc::ListItemsRequest;
use akeyless_grpc::ListItems;
use akeyless_grpc::GetSecretValueRequest;
use akeyless_grpc::GetSecretValue
use prost_types::value::Kind;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut client = AkeylessV2ServiceClient::connect("http://localhost:8085").await?;

    let auth = Auth{
        access_id : "*********".to_string(),
        access_type : "access_key".to_string(),
        access_key : "*********".to_string(),
        ..Default::default() 
   }; 

    let auth_req = AuthRequest {
       body : Some(auth),
    };

    let auth_resp = client.auth(auth_req).await?;

    let token = auth_resp.into_inner().token.to_string();

    let list_item = ListItems {
        token: token.clone(),
        ..Default::default()
    };

    let list_item_req = ListItemsRequest {
        body : Some(list_item)
    };

    let list_items_resp = client.list_items(list_item_req).await?;

    for item in list_items_resp.into_inner().items {
        println!("{}", item.item_name);
    }

    let get_secret_value = GetSecretValue{
        token: token.clone(),
        names: vec![String::from("/MyFirstSecret")],
        ..Default::default() 
    };

    let get_secret_value_req = GetSecretValueRequest {
        body : Some(get_secret_value)
    };

    let list_items_resp = client.get_secret_value(get_secret_value_req).await?;
    
    if let Some(data) = list_items_resp.into_inner().data {
        for (key, value) in data.fields.iter() {
            
            let string_value = match &value.kind {
                Some(Kind::StringValue(string_value)) => string_value.clone(),
                _ => String::new(), // Handle if value is not a StringValue
            };

            println!("{} {:?}", key, string_value);

        }
    }

    Ok(())
}
```

### License

This project is licensed under the Apache License 2.0. See the LICENSE file for details.
