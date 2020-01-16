#![deny(warnings)]

extern crate hyper;

use std::env;

use hyper::{body::HttpBody as _, client::connect::HttpConnector, Client};
use hyper_tls::HttpsConnector;
use tokio::io::{self, AsyncWriteExt as _};

type HyperResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> HyperResult<()> {
    let _ = match env::args().nth(1) {
        Some(url) => match Some(&*url) {
            Some("get_online") => get_online_coins().await?,
            Some("get_offline") => get_offline_coins().await?,
            Some("get_coins") => get_coins().await?,
            Some("valid_pairs") => get_valid_pairs().await?,
            Some("help") => show_help().await?,
            Some(_) => {
                println!("Invalid command: {}", url);
                show_help().await?
            }
            None => {
                println!("Null command");
                show_help().await?
            }
        },
        None => {
            show_help().await?;
            return Ok(());
        }
    };

    Ok(())
}

struct HelpInfo {
    cmd: String,
    info: String,
}

impl HelpInfo {
    fn new(c: String, i: String) -> HelpInfo {
        HelpInfo { cmd: c, info: i }
    }
}

async fn show_help() -> HyperResult<()> {
    println!("Help\n");
    println!("Usage:\n\tshapeshift-rs <command>\n");
    println!("Commands:");
    let cmds: Vec<HelpInfo> = vec![
        HelpInfo::new("get_online".into(), "get online coins".into()),
        HelpInfo::new("get_offline".into(), "get offline coins".into()),
        HelpInfo::new("get_coins".into(), "get coins information".into()),
        HelpInfo::new("valid_pairs".into(), "get valid trading pairs".into()),
        HelpInfo::new("help".into(), "show this help menu".into()),
    ];

    for c in cmds.iter() {
        println!("\t{}\t-\t{}", c.cmd, c.info);
    }

    Ok(())
}

fn build_https_client() -> Client<HttpsConnector<HttpConnector>, hyper::Body> {
    Client::builder().build::<_, hyper::Body>(HttpsConnector::new())
}

async fn do_request(req: &mut hyper::client::ResponseFuture) -> HyperResult<()> {
    let mut res = req.await?;
    println!("Response status: {}", res.status());

    while let Some(chunk) = res.body_mut().data().await {
        io::stdout().write_all(&chunk?).await?;
    }

    Ok(())
}

async fn get_online_coins() -> HyperResult<()> {
    let client = build_https_client();
    let uri = "https://shapeshift.io/onlinecoins".parse()?;
    let mut req = client.get(uri);

    do_request(&mut req).await
}

async fn get_offline_coins() -> HyperResult<()> {
    let client = build_https_client();
    let uri = "https://shapeshift.io/offlinecoins".parse()?;
    let mut req = client.get(uri);

    do_request(&mut req).await
}

async fn get_coins() -> HyperResult<()> {
    let client = build_https_client();
    let uri = "https://shapeshift.io/getcoins".parse()?;
    let mut req = client.get(uri);

    do_request(&mut req).await
}

async fn get_valid_pairs() -> HyperResult<()> {
    let client = build_https_client();
    let uri = "https://shapeshift.io/validpairs".parse()?;
    let mut req = client.get(uri);

    do_request(&mut req).await
}
