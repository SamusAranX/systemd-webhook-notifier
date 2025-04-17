use const_format::formatcp;

pub(crate) const GIT_HASH: &str = env!("GIT_HASH");
pub(crate) const GIT_BRANCH: &str = env!("GIT_BRANCH");
pub(crate) const GIT_VERSION: &str = env!("GIT_VERSION");
pub(crate) const BUILD_DATE: &str = env!("BUILD_DATE");

pub(crate) const CLAP_VERSION: &str = formatcp!("{GIT_VERSION} [{GIT_BRANCH}, {GIT_HASH}, {BUILD_DATE}]");