use super::model::JreInfo;

/// 将版本字符串转换为可比较的数字向量
/// 例如: "1.8.0_556" -> [1, 8, 0, 556]
fn parse_version(version: &str) -> Vec<i32> {
    // 首先按 '_' 分割处理更新编号
    let parts: Vec<&str> = version.split('_').collect();
    let mut numbers = parts[0]
        .split('.')
        .map(|s| s.parse::<i32>().unwrap_or(0))
        .collect::<Vec<i32>>();

    // 如果存在更新编号，添加到数组末尾
    if parts.len() > 1 {
        if let Ok(update_num) = parts[1].parse::<i32>() {
            numbers.push(update_num);
        }
    }

    numbers
}

/// 比较两个版本号
/// 返回值:
/// - 负数: a < b
/// - 0: a == b
/// - 正数: a > b
fn compare_versions(a: &str, b: &str) -> i32 {
    let version_a = parse_version(a);
    let version_b = parse_version(b);

    for i in 0..version_a.len().max(version_b.len()) {
        let num_a = version_a.get(i).copied().unwrap_or(0);
        let num_b = version_b.get(i).copied().unwrap_or(0);

        if num_a != num_b {
            return num_a - num_b;
        }
    }
    0
}

/// 对JRE列表按版本号进行排序（降序）
pub fn sort_jres_by_version_desc(jres: &mut Vec<JreInfo>) {
    jres.sort_by(|a, b| compare_versions(&b.version, &a.version).cmp(&0));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::jre::model::Architecture;
    use std::path::PathBuf;
    #[test]
    fn test_version_comparison() {
        // 基本版本号比较
        assert!(compare_versions("1.8.0", "1.7.0") > 0);
        assert!(compare_versions("11.0.2", "1.8.0") > 0);
        assert!(compare_versions("17.0.1", "11.0.2") > 0);
        assert_eq!(compare_versions("1.8.0", "1.8.0"), 0);
        assert!(compare_versions("1.8.0", "1.8.1") < 0);

        // 带更新编号的版本比较
        assert!(compare_versions("1.8.0_556", "1.8.0_447") > 0);
        assert!(compare_versions("1.8.0_556", "1.8.0") > 0);
        assert!(compare_versions("11.0.2", "1.8.0_556") > 0);
        assert_eq!(compare_versions("1.8.0_556", "1.8.0_556"), 0);
    }

    #[test]
    fn test_sort_jres() {
        let mut jres = vec![
            JreInfo {
                path: PathBuf::from("/path/to/java8"),
                version: "1.8.0".to_string(),
                arch: Architecture::X86_64,
                implementor: Some("Oracle".to_string()),
            },
            JreInfo {
                path: PathBuf::from("/path/to/java17"),
                version: "17.0.1".to_string(),
                arch: Architecture::X86_64,
                implementor: Some("Oracle".to_string()),
            },
            JreInfo {
                path: PathBuf::from("/path/to/java11"),
                version: "11.0.2".to_string(),
                arch: Architecture::X86_64,
                implementor: Some("Oracle".to_string()),
            },
        ];

        sort_jres_by_version_desc(&mut jres);

        assert_eq!(jres[0].version, "17.0.1");
        assert_eq!(jres[1].version, "11.0.2");
        assert_eq!(jres[2].version, "1.8.0");
    }
}
