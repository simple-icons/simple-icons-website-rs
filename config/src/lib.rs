use config::{Config, File, FileFormat};
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new({
        Config::builder()
            .add_source(File::from_str(
                #[cfg(debug_assertions)]
                include_str!("../settings.dev.json"),
                #[cfg(not(debug_assertions))]
                include_str!("../settings.prod.json"),
                FileFormat::Json,
            ))
            .build()
            .unwrap()
    });
}
