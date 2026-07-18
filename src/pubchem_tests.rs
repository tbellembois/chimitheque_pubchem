#[cfg(test)]
mod tests {
    #![allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::too_many_lines
    )]

    use crate::pubchem::{autocomplete, get_compound_cid, get_product_by_name};
    use futures::executor::block_on;
    use governor::{Quota, RateLimiter};
    use log::info;
    use std::{num::NonZeroU32, time::SystemTime};

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

        assert!(get_product_by_name(&rate_limiter, "1,4-butanediol").is_ok());
        assert!(get_product_by_name(&rate_limiter, "1,4-dioxane").is_ok());
        assert!(get_product_by_name(&rate_limiter, "acetic acid").is_ok());
        assert!(get_product_by_name(&rate_limiter, "acetone").is_ok());
        assert!(get_product_by_name(&rate_limiter, "Aluminium oxide").is_ok());
        assert!(get_product_by_name(&rate_limiter, "ammonium chloride").is_ok());
        assert!(get_product_by_name(&rate_limiter, "cesium carbonate").is_ok());
        assert!(get_product_by_name(&rate_limiter, "chloroform").is_ok());
        assert!(get_product_by_name(&rate_limiter, "chlorotrimethylsilane").is_ok());
        assert!(get_product_by_name(&rate_limiter, "cyclohexane").is_ok());

        assert!(get_product_by_name(&rate_limiter, "Xyl€Θöl-42").is_err());
        assert!(get_product_by_name(&rate_limiter, "$uperN€ 컴퓨터트Delivery").is_err());
        assert!(get_product_by_name(&rate_limiter, "Hēⓘ로는로로 encode.").is_err());
        assert!(get_product_by_name(&rate_limiter, "Nϵith€rHērε≈Ξ").is_err());
        assert!(get_product_by_name(&rate_limiter, "RαdiⱤX-!And@").is_err());
        assert!(get_product_by_name(&rate_limiter, "QuΛntumΦυs!$").is_err());
        assert!(get_product_by_name(&rate_limiter, "Kยรรย´ะย´ะs sufficeϗγ").is_err());
        assert!(get_product_by_name(&rate_limiter, "Δ wherebyΞxploded").is_err());
        assert!(get_product_by_name(&rate_limiter, "Сhusва® ey°").is_err());
        assert!(get_product_by_name(&rate_limiter, "FractstrapΞΔMΞGA^2077").is_err());
    }

    #[test]
    fn test_get_compound_cid() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

        assert!(get_compound_cid(&rate_limiter, "aspirine").is_ok_and(|x| x > 0));
        assert!(
            get_compound_cid(&rate_limiter, "D-Diacetyltartaric anhydride").is_ok_and(|x| x > 0)
        );
        assert!(
            get_compound_cid(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride")
                .is_ok_and(|x| x > 0)
        );
        assert!(
            get_compound_cid(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride")
                .is_ok_and(|x| x > 0)
        );
        assert!(get_compound_cid(&rate_limiter, "abcdefghijklmopqrst").is_err());
    }

    #[test]
    fn test_rate_limiter() {
        init_logger();

        let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(1).unwrap()));

        let before = SystemTime::now();
        for _i in 1..6 {
            block_on(rate_limiter.until_ready());
        }
        assert!(before.elapsed().unwrap().as_secs() >= 4);
    }
}
