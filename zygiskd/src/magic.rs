use std::fs;
use anyhow::Result;
use crate::constants;
use crate::utils::LateInit;

pub static MAGIC: LateInit<String> = LateInit::new();
pub static PATH_TMP_DIR: LateInit<String> = LateInit::new();
pub static PATH_TMP_PROP: LateInit<String> = LateInit::new();

pub fn setup() -> Result<()> {
    let name = fs::read_to_string(constants::ZYGISK_MAGIC)?;
    let path_tmp_dir = format!("/dev/{}", name);
    let path_tmp_prop = format!("{}/module.prop", path_tmp_dir);

    MAGIC.init(name);
    PATH_TMP_DIR.init(path_tmp_dir);
    PATH_TMP_PROP.init(path_tmp_prop);
    Ok(())
}
