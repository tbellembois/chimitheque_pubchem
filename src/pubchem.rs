use std::io::Cursor;

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
use urlencoding::encode;

use crate::pubchem_compound::{Autocomplete, PropertyTable, Record};

pub fn autocomplete(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    search: &str,
) -> Result<Autocomplete, String> {
    let urlencoded_search = encode(search);

    // Call NCBI REST API.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    let resp = match reqwest::blocking::get(format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/autocomplete/compound/{urlencoded_search}/json",
    )) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp: {:#?}", resp);

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    debug!("body_text: {:?}", body_text);

    // Unmarshall into JSON.
    let autocomplete: Autocomplete = match serde_json::from_str(&body_text.to_owned()) {
        Ok(autocomplete) => autocomplete,
        Err(e) => return Err(e.to_string()),
    };

    Ok(autocomplete)
}

pub fn get_product_by_name(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<Option<PubchemProduct>, String> {
    let record = get_raw_compound_by_name(rate_limiter, name)?;

    let mut product = PubchemProduct::from_pubchem(record);

    //
    // Get 2d image.
    //
    // Call NCBI REST API for png.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    let urlencoded_name = encode(name);

    let query_url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/PNG?image_size=300x300"
    );
    debug!("query_url: {query_url}");

    let resp = match reqwest::blocking::get(query_url) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp.status(): {}", resp.status());

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_bytes = match resp.bytes() {
        Ok(body_bytes) => body_bytes,
        Err(e) => return Err(e.to_string()),
    };

    // Create image.
    let image = match image::load_from_memory_with_format(&body_bytes, image::ImageFormat::Png) {
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
    if let Some(ref mut p) = product {
        p.twodpicture = Some(res_base64)
    }

    Ok(product)
}

// Get the compound CID from the parameter name.
fn get_compound_cid(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<Option<usize>, String> {
    let urlencoded_name = encode(name);

    // Call NCBI REST API for JSON.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    // We need to query at least one property to get the CID. Choosing MolecularFormula.
    let query_url = format!(
        "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{urlencoded_name}/property/MolecularFormula/JSON"
    );
    debug!("query_url: {query_url}");

    let resp = match reqwest::blocking::get(query_url) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp.status(): {}", resp.status());

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    // Unmarshall into JSON.
    let property_table: PropertyTable = match serde_json::from_str(&body_text.to_owned()) {
        Ok(property_table) => property_table,
        Err(e) => return Err(e.to_string()),
    };

    // Extract compound cid.
    let compound_cid = match property_table.property_table.properties.first() {
        Some(compound_cid) => compound_cid.cid,
        None => return Err("can not find compound cid".to_string()),
    };

    Ok(Some(compound_cid))
}

// Get the compound from the parameter name as a raw json.
pub fn get_raw_compound_by_name(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<String, String> {
    //
    // Get compound CID.
    //
    let compound_cid = match get_compound_cid(rate_limiter, name) {
        Ok(maybe_compound_cid) => match maybe_compound_cid {
            Some(compound_cid) => compound_cid,
            None => return Err(String::from("none compound cid")),
        },
        Err(e) => return Err(e.to_string()),
    };

    //
    // Get detailed informations.
    //
    // Call NCBI REST API for JSON.
    debug!(">block_on");
    block_on(rate_limiter.until_ready());
    debug!("<block_on");

    let query_url =
        format!("https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{compound_cid}/JSON");
    debug!("query_url: {query_url}");

    let resp = match reqwest::blocking::get(query_url) {
        Ok(resp) => resp,
        Err(e) => return Err(e.to_string()),
    };

    debug!("resp.status(): {}", resp.status());

    // Check HTTP code.
    if !resp.status().is_success() {
        return Err(resp.status().to_string());
    }

    // Get response body.
    let body_text = match resp.text() {
        Ok(body_text) => body_text,
        Err(e) => return Err(e.to_string()),
    };

    Ok(body_text)
}

// Get the compound from the parameter name as a Record struct.
pub fn get_compound_by_name(
    rate_limiter: &RateLimiter<NotKeyed, InMemoryState, clock::DefaultClock, NoOpMiddleware>,
    name: &str,
) -> Result<Record, String> {
    // Get raw JSON string.
    let raw_compound = match get_raw_compound_by_name(rate_limiter, name) {
        Ok(raw_compound) => raw_compound,
        Err(e) => return Err(e.to_string()),
    };

    // Unmarshall into JSON.
    let record: Record = match serde_json::from_str(&raw_compound) {
        Ok(record) => record,
        Err(e) => return Err(e.to_string()),
    };

    Ok(record)
}

#[cfg(test)]
mod tests {

    use std::{num::NonZeroU32, time::SystemTime};

    use governor::{Quota, RateLimiter};
    use log::info;
    use std::time::Instant;

    use super::*;

    fn init_logger() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn test_autocomplete() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        info!(
            "aspirine: {:?}",
            autocomplete(&rate_limiter, "aspirine").unwrap()
        );
        info!(
            "DIACETYL-L-TARTARIC ANHYDRIDE: {:?}",
            autocomplete(&rate_limiter, "DIACETYL-L-TARTARIC ANHYDRIDE").unwrap()
        );
        info!("#: {:?}", autocomplete(&rate_limiter, "#").unwrap());
    }

    #[test]
    fn test_get_product_by_name() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        let now = Instant::now();
        info!(
            "aspirine: {:#?}",
            get_product_by_name(&rate_limiter, "aspirine")
        );
        let elapsed = now.elapsed();
        info!("elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        info!(
            "D-Diacetyltartaric anhydride: {:#?}",
            get_product_by_name(&rate_limiter, "D-Diacetyltartaric anhydride").unwrap()
        );
        let elapsed = now.elapsed();
        info!("elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        info!(
            "(-)-Diacetyl-D-tartaric Anhydride: {:#?}",
            get_product_by_name(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride").unwrap()
        );
        let elapsed = now.elapsed();
        info!("elapsed: {:.2?}", elapsed);

        let now = Instant::now();
        info!(
            "(+)-Diacetyl-L-tartaric anhydride: {:#?}",
            get_product_by_name(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride").unwrap()
        );
        let elapsed = now.elapsed();
        info!("elapsed: {:.2?}", elapsed);
    }

    #[test]
    fn test_get_compound_by_name() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        info!(
            "aspirine: {:#?}",
            get_compound_by_name(&rate_limiter, "aspirine")
        );
        info!(
            "D-Diacetyltartaric anhydride: {:#?}",
            get_compound_by_name(&rate_limiter, "D-Diacetyltartaric anhydride").unwrap()
        );
        info!(
            "(-)-Diacetyl-D-tartaric Anhydride: {:#?}",
            get_compound_by_name(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride").unwrap()
        );
        info!(
            "(+)-Diacetyl-L-tartaric anhydride: {:#?}",
            get_compound_by_name(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride").unwrap()
        );
    }

    #[test]
    fn test_get_compound_cid() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        assert!(
            get_compound_cid(&rate_limiter, "aspirine").is_ok_and(|x| x.is_some_and(|y| y > 0))
        );
        assert!(
            get_compound_cid(&rate_limiter, "D-Diacetyltartaric anhydride")
                .is_ok_and(|x| x.is_some_and(|y| y > 0))
        );
        assert!(
            get_compound_cid(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride")
                .is_ok_and(|x| x.is_some_and(|y| y > 0))
        );
        assert!(
            get_compound_cid(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride")
                .is_ok_and(|x| x.is_some_and(|y| y > 0))
        );
        assert!(get_compound_cid(&rate_limiter, "abcdefghijklmopqrst").is_err());
    }

    #[test]
    fn test_rate_limiter() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(1).unwrap()));

        let before = SystemTime::now();
        for i in 1..6 {
            block_on(rate_limiter.until_ready());
            debug!("loop {i}");
        }
        info!("{:?}", before.elapsed());
        assert!(before.elapsed().unwrap().as_secs() >= 4);
    }
}
