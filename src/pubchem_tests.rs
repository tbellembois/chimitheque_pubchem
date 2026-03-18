#[cfg(test)]
mod tests {

    use futures::executor::block_on;
    use governor::{Quota, RateLimiter};
    use log::info;
    use std::time::Instant;
    use std::{num::NonZeroU32, time::SystemTime};

    use crate::pubchem::{autocomplete, get_compound_cid, get_product_by_name};

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
        info!("elapsed: {elapsed:.2?}");

        let now = Instant::now();
        info!(
            "D-Diacetyltartaric anhydride: {:#?}",
            get_product_by_name(&rate_limiter, "D-Diacetyltartaric anhydride").unwrap()
        );
        let elapsed = now.elapsed();
        info!("elapsed: {elapsed:.2?}");

        let now = Instant::now();
        info!(
            "(-)-Diacetyl-D-tartaric Anhydride: {:#?}",
            get_product_by_name(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride").unwrap()
        );
        let elapsed = now.elapsed();
        info!("elapsed: {elapsed:.2?}");

        let now = Instant::now();
        info!(
            "(+)-Diacetyl-L-tartaric anhydride: {:#?}",
            get_product_by_name(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride").unwrap()
        );
        let elapsed = now.elapsed();
        info!("elapsed: {elapsed:.2?}");
    }

    // #[test]
    // fn test_get_compound_by_name() {
    //     init_logger();

    //     let rate_limiter = RateLimiter::direct(Quota::per_second(NonZeroU32::new(5).unwrap()));

    //     info!(
    //         "aspirine: {:#?}",
    //         get_compound_by_name(&rate_limiter, "aspirine")
    //     );
    //     info!(
    //         "D-Diacetyltartaric anhydride: {:#?}",
    //         get_compound_by_name(&rate_limiter, "D-Diacetyltartaric anhydride").unwrap()
    //     );
    //     info!(
    //         "(-)-Diacetyl-D-tartaric Anhydride: {:#?}",
    //         get_compound_by_name(&rate_limiter, "(-)-Diacetyl-D-tartaric Anhydride").unwrap()
    //     );
    //     info!(
    //         "(+)-Diacetyl-L-tartaric anhydride: {:#?}",
    //         get_compound_by_name(&rate_limiter, "(+)-Diacetyl-L-tartaric anhydride").unwrap()
    //     );
    // }

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
        for i in 1..6 {
            block_on(rate_limiter.until_ready());
        }
        assert!(before.elapsed().unwrap().as_secs() >= 4);
    }
}
