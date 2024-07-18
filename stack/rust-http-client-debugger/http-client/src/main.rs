use anyhow::{anyhow, bail};

use jaq_interpret::{
    Ctx,
    // Error,
    FilterT,
    ParseCtx,
    RcIter,
    Val,
};
use reqwest::{
    redirect::Policy,
    //  tls,
    Certificate,
};
use serde_json::Value;
use tokio::io::AsyncReadExt;
use tokio::{fs::File, time::Duration};

type AppResult<T> = Result<T, anyhow::Error>;

#[tokio::main]
async fn main() -> AppResult<()> {
    const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
    const FILE_MAX_SIZE: usize = 10_485_760; // 10 MBytes

    tracing_subscriber::fmt::init();

    let proxy = reqwest::Proxy::https("http://localhost:8090")?;

    let file_path = "zap_root_ca.cer";
    {
        let file_metadata = std::fs::metadata(file_path)
            .map_err(|error| anyhow!("Error on reading [{file_path}]. Reason: {error}"))?;
        let file_size: usize = file_metadata.len().try_into()?;

        if file_size > FILE_MAX_SIZE {
            bail!("File {file_path} is too large. Max size is ");
        }
    }

    let mut file = File::open(file_path).await?;

    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).await?;

    let client = reqwest::Client::builder()
        .proxy(proxy)
        .user_agent(APP_USER_AGENT)
        .redirect(Policy::limited(10))
        .timeout(Duration::from_secs(10))
        .https_only(true)
        .add_root_certificate(Certificate::from_pem(&file_buf)?)
        .use_rustls_tls()
        .connection_verbose(true)
        // .min_tls_version(tls::Version::TLS_1_2)
        // .tls_sni(true)
        // .tls_info(true)
        // .cookie_store(true)
        .build()
        .expect("should be able to build reqwest client");
    // dbg!(&client);

    let req = client.get("https://api.github.com/users/ancos2505/repos");
    // let req = client.get("https://example.net");
    // dbg!(&req);

    let res = req.send().await?;
    let body: serde_json::Value = res.json().await?;

    let output = filter_body(body)?;
    // dbg!(&output);
    // output.iter().try_for_each(|val| -> AppResult<()> {
    //     let s: String = serde_json::to_string(val)?;
    //     println!("{s}");
    //     Ok(())
    // })?;

    println!("{}", serde_json::to_string(&output)?);

    Ok(())
}

fn filter_body(input: serde_json::Value) -> AppResult<Vec<Value>> {
    // let input: serde_json::Value = serde_json::from_str(r#"["Hello", "world"]"#).unwrap();

    // let filter_str = "[*].name";
    // let filter_str = ".";
    let filter_str = ".[].name";

    // start out only from core filters,
    // which do not include filters in the standard library
    // such as `map`, `select` etc.
    let mut jaq_defs = ParseCtx::new(Vec::new());

    // parse the filter
    let (maybe_parsed_filter, errs) = jaq_parse::parse(filter_str, jaq_parse::main());

    if !errs.is_empty() {
        dbg!(errs);
        bail!("Error");
    }
    // compile the filter in the context of the given definitions
    let compiled_filter = jaq_defs.compile(maybe_parsed_filter.unwrap());

    let inputs = RcIter::new(core::iter::empty());

    // iterator over the output values
    let mut out = compiled_filter.run((Ctx::new([], &inputs), Val::from(input)));
    let outcome = out
        .into_iter()
        .map(|item| item.map_err(|err| anyhow!("{err}")))
        .map(|item| item.map(|ok| ok.into()))
        .collect();
    outcome
}
