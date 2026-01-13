use serde::{Deserialize, Serialize};

use crate::{appconfig, utils::appconfig::AppFeature};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConfigPaper {
    pub features: FeatureConfigPaper,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeatureConfigPaper {
    pub signup: bool,
    pub login: bool,
    pub landing: bool,
    pub create_organization: bool,
}

impl ConfigPaper {
    pub fn new() -> ConfigPaper {
        let config = appconfig!();
        return ConfigPaper {
            features: FeatureConfigPaper {
                signup: config.feature_enabled(AppFeature::Signup),
                login: config.feature_enabled(AppFeature::Login),
                landing: config.feature_enabled(AppFeature::Landing),
                create_organization: config.feature_enabled(AppFeature::CreateOrganization),
            },
        };
    }
}
