use base64::{Engine, engine::general_purpose};
use chimitheque_types::pubchemproduct::PubchemProduct;
use futures::executor::block_on;
use governor::{
    RateLimiter, clock,
    middleware::NoOpMiddleware,
    state::{InMemoryState, NotKeyed},
};
use image::ImageFormat;
use log::debug;
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use ureq::{config::Config, http::HeaderValue};
use urlencoding::encode;

// Autocomplete
#[derive(Serialize, Deserialize, Debug)]
pub struct AutocompleteTerm {
    compound: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Autocomplete {
    total: usize,

    #[serde(skip_serializing_if = "Option::is_none")]
    dictionary_terms: Option<AutocompleteTerm>,
}

// Returns the auto complete strings with the X-Throttling-Control response header.
pub fn autocomplete(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    search: &str,
) -> Result<(Autocomplete, String), String> {
    let urlencoded_search = encode(search);

    let query_url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/autocomplete/compound/{urlencoded_search}/json"
    );
    debug!("query_url: {query_url}");

    // Build TLS HTTP client.
    let tls_config = ureq::tls::TlsConfig::builder()
        .disable_verification(false)
        .build();

    // Build request config.
    let config = Config::builder().tls_config(tls_config).build();

    // Create client.
    let http_client = config.new_agent();

    // Call NCBI REST API.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    match http_client.get(query_url).call() {
        Ok(mut response) => match response.body_mut().read_json::<Autocomplete>() {
            Ok(autocomplete) => Ok((
                autocomplete,
                response
                    .headers()
                    .get("X-Throttling-Control")
                    .unwrap_or(&HeaderValue::from_static(
                        "X-Throttling-Control header not found",
                    ))
                    .to_str()
                    .unwrap_or_default()
                    .to_string(),
            )),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

// Returns the product as a string with the X-Throttling-Control response header.
pub fn get_product_by_name(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<(PubchemProduct, String), String> {
    //
    // Get compound CID.
    //
    let compound_cid = get_compound_cid(rate_limiter, name)?;

    //
    // Get compound by CID as a raw JSON string.
    //
    let record = get_pubchem_json_compound_by_cid(rate_limiter, compound_cid)?;

    //
    // Parse JSON into a PubchemProduct.
    //
    let mut product = PubchemProduct::from_pubchem_json(record.as_str());

    //
    // Get 2d image.
    //
    // Build TLS HTTP client.
    let tls_config = ureq::tls::TlsConfig::builder()
        .disable_verification(false)
        .build();

    // Build request config.
    let config = Config::builder().tls_config(tls_config).build();

    // Create client.
    let http_client = config.new_agent();

    // Call NCBI REST API for png.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    let urlencoded_name = encode(name);

    let query_url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/PNG?image_size=300x300"
    );
    debug!("query_url: {query_url}");

    let response = match http_client.get(query_url).call() {
        Ok(response) => response,
        Err(err) => return Err(err.to_string()),
    };

    let header = response
        .headers()
        .get("X-Throttling-Control")
        .unwrap_or(&HeaderValue::from_static(
            "X-Throttling-Control header not found",
        ))
        .to_str()
        .unwrap_or_default()
        .to_string();

    let bytes = match response.into_body().read_to_vec() {
        Ok(bytes) => bytes,
        Err(err) => return Err(err.to_string()),
    };

    // Create image.
    let image = match image::load_from_memory_with_format(&bytes, image::ImageFormat::Png) {
        Ok(image) => image,
        Err(e) => return Err(e.to_string()),
    };

    // Convert to base64.
    let mut image_data: Vec<u8> = Vec::new();
    if let Err(e) = image.write_to(&mut Cursor::new(&mut image_data), ImageFormat::Png) {
        return Err(e.to_string());
    }
    let res_base64 = general_purpose::STANDARD.encode(&image_data);

    // Update the result.
    product.twodpicture = Some(res_base64);

    Ok((product, header))
}

// Get the compound CID from the parameter name.
fn get_compound_cid(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<usize, String> {
    let urlencoded_name = encode(name);

    // Build TLS HTTP client.
    let tls_config = ureq::tls::TlsConfig::builder()
        .disable_verification(false)
        .build();

    // Build request config.
    let config = Config::builder().tls_config(tls_config).build();

    // Create client.
    let http_client = config.new_agent();

    // Call NCBI REST API for JSON.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    // We need to query at least one property to get the CID. Choosing MolecularFormula.
    let query_url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/cids/TXT"
    );
    debug!("query_url: {query_url}");

    let compound_cid = match http_client.get(query_url).call() {
        Ok(mut response) => match response.body_mut().read_to_string() {
            Ok(cid_str) => cid_str.trim().parse::<usize>().ok().unwrap_or_default(),
            Err(err) => return Err(err.to_string()),
        },
        Err(err) => return Err(err.to_string()),
    };

    // let property_table = match http_client.get(query_url).call() {
    //     Ok(mut response) => match response.body_mut().read_json::<PropertyTable>() {
    //         Ok(property_table) => property_table,
    //         Err(err) => return Err(err.to_string()),
    //     },
    //     Err(err) => return Err(err.to_string()),
    // };

    // Extract compound cid.
    // let compound_cid = match property_table.property_table.properties.first() {
    //     Some(compound_cid) => compound_cid.cid,
    //     None => return Err("can not find compound cid".to_string()),
    // };

    Ok(compound_cid)
}

// Get the compound by name from the pubchem API as a raw JSON string.
fn get_pubchem_json_compound_by_cid(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    cid: usize,
) -> Result<String, String> {
    //
    // Get detailed informations.
    //
    // Build TLS HTTP client.
    let tls_config = ureq::tls::TlsConfig::builder()
        .disable_verification(false)
        .build();

    // Build request config.
    let config = Config::builder().tls_config(tls_config).build();

    // Create client.
    let http_client = config.new_agent();

    // Call NCBI REST API for JSON.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    // We use the PUG view API to get detailed informations.
    // see https://pubchem.ncbi.nlm.nih.gov/docs/pug-view
    let query_url =
        format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{cid}/JSON");
    debug!("query_url: {query_url}");

    match http_client.get(query_url).call() {
        Ok(mut response) => match response.body_mut().read_to_string() {
            Ok(body_text) => Ok(body_text),
            Err(err) => Err(err.to_string()),
        },
        Err(err) => Err(err.to_string()),
    }
}

// -> not used
// Get the compound from the parameter name as a Record struct.
// pub fn get_compound_by_name(
//     rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
//     name: &str,
// ) -> Result<Record, String> {
//     // Get raw JSON string.
//     let raw_compound = get_raw_compound_by_name(rate_limiter, name)?;

//     // Unmarshall into JSON.
//     let record: Record = match serde_json::from_str(&raw_compound) {
//         Ok(record) => record,
//         Err(e) => return Err(e.to_string()),
//     };

//     Ok(record)
// }

#[cfg(test)]
#[path = "pubchem_tests.rs"]
mod pubchem_tests;
