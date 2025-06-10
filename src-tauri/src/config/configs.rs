use serde::{Deserialize, Serialize};

use crate::jre::model::JreInfo;

#[derive(Serialize, Deserialize)]
pub struct JreConfig {
    pub jres: Vec<JreInfo>, // 存储 JRE 路径的字符串列表
}

impl Default for JreConfig {
    fn default() -> Self {
        JreConfig {
            jres: Vec::new(), // 初始化为空列表
        }
    }
}
