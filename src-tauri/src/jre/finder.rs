use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JreInfo {
    /// JRE 目录的路径
    pub path: PathBuf,
    /// Java 版本号
    pub version: String,
    /// 是否是 64 位
    pub is_64bit: bool,
}

/// 检查给定路径是否为有效的 JRE 安装
fn verify_jre_path(path: &PathBuf) -> Option<JreInfo> {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    let java_bin = path.join("bin").join("java.exe");
    #[cfg(not(target_os = "windows"))]
    let java_bin = path.join("bin").join("java");

    if !java_bin.exists() {
        return None;
    }

    // 运行 java -version 获取版本信息
    let output = Command::new(&java_bin)
        .arg("-version")
        .output()
        .ok()?;

    let version_output = String::from_utf8_lossy(&output.stderr);
    
    // 解析版本信息
    let version = version_output
        .lines()
        .next()?
        .split('"')
        .nth(1)?
        .to_string();    // 检测是否是 64 位，通过解析 java -version 的输出
    let is_64bit = version_output.to_lowercase().contains("64-bit");

    Some(JreInfo {
        path: path.to_owned(),
        version,
        is_64bit,
    })
}

/// 获取系统中可能安装 JRE 的路径列表
fn get_potential_jre_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    
    // 检查 JAVA_HOME 环境变量
    if let Ok(java_home) = std::env::var("JAVA_HOME") {
        paths.push(PathBuf::from(java_home));
    }

    // 添加用户目录下的 .jdks 目录
    if let Some(home_dir) = dirs::home_dir() {
        paths.push(home_dir.join(".jdks"));
    }

    // 系统特定的路径
    #[cfg(target_os = "windows")] {
        use winreg::enums::*;
        use winreg::RegKey;

        // 从注册表获取路径
        let locations = [
            r"SOFTWARE\JavaSoft\Java Runtime Environment",
            r"SOFTWARE\JavaSoft\Java Development Kit",
            r"SOFTWARE\JavaSoft\JRE",
            r"SOFTWARE\JavaSoft\JDK",
        ];

        for key_path in locations {
            if let Ok(key) = RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey(key_path) {
                if let Ok(current_version) = key.get_value::<String, _>("CurrentVersion") {
                    if let Ok(version_key) = key.open_subkey(&current_version) {
                        if let Ok(java_home) = version_key.get_value::<String, _>("JavaHome") {
                            paths.push(PathBuf::from(java_home));
                        }
                    }
                }
            }
        }

        // Windows 常见安装路径
        paths.extend([
            r"C:\Program Files\Java",
            r"C:\Program Files (x86)\Java",
            r"C:\Program Files\Eclipse Adoptium",
            r"C:\Program Files (x86)\Eclipse Adoptium",
            r"C:\Program Files\Microsoft\jdk",
            r"C:\Program Files (x86)\Microsoft\jdk",
            r"C:\Program Files\Microsoft",
            r"C:\Program Files\Common Files\Oracle\Java",
            // format!("{}\\Packages\\Microsoft.4297127D64EC6_8wekyb3d8bbwe\\LocalCache\\Local\\runtime", std::env::var("LOCALAPPDATA").unwrap_or_default()).as_str(),
        ].iter().map(PathBuf::from));
    }

    #[cfg(target_os = "linux")] {
        use std::process::Command;
        
        // 通过 which 命令查找
        if let Ok(output) = Command::new("which").arg("java").output() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                if let Some(parent) = PathBuf::from(path.trim()).parent() {
                    if let Some(jre_root) = parent.parent() {
                        paths.push(jre_root.to_owned());
                    }
                }
            }
        }

        // Linux 常见安装路径
        paths.extend([
            "/usr/lib/jvm",
            "/usr/java",
            "/usr/local/java",
            "/opt/java",
        ].iter().map(PathBuf::from));
    }

    #[cfg(target_os = "macos")] {
        // 使用 java_home 命令
        if let Ok(output) = Command::new("/usr/libexec/java_home").output() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                paths.push(PathBuf::from(path.trim()));
            }
        }

        // macOS 常见安装路径
        paths.extend([
            "/Library/Java/JavaVirtualMachines",
            "/System/Library/Java/JavaVirtualMachines",
        ].iter().map(PathBuf::from));
    }

    paths
}

/// 查找系统中的第一个可用 JRE
pub fn find_jre() -> Option<JreInfo> {
    scan_jres().into_iter().next()
}

/// 扫描系统中所有可用的 JRE
pub fn scan_jres() -> Vec<JreInfo> {
    use std::fs;
    use std::collections::HashSet;

    let mut result = HashSet::new();
    
    // 获取所有可能的路径
    for base_path in get_potential_jre_paths() {
        if !base_path.exists() {
            continue;
        }

        // 如果是目录，直接检查
        if let Some(jre) = verify_jre_path(&base_path) {
            result.insert(jre);
            continue;
        }

        // 如果是父目录，遍历子目录
        if let Ok(entries) = fs::read_dir(&base_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // 在 macOS 上，JDK 在 Contents/Home 下
                    #[cfg(target_os = "macos")]
                    let path = if path.ends_with("JavaVirtualMachines") {
                        path.join("Contents").join("Home")
                    } else {
                        path
                    };

                    if let Some(jre) = verify_jre_path(&path) {
                        result.insert(jre);
                    }
                }
            }
        }
    }

    result.into_iter().collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_jre() {
        // if let Some(jre) = find_jre() {
        //     println!("Found JRE:");
        //     println!("Path: {:?}", jre.path);
        //     println!("Version: {}", jre.version);
        //     println!("64-bit: {}", jre.is_64bit);
        // } else {
        //     println!("No JRE found");
        // }
        let all_jres = scan_jres();
        for jre in all_jres {
            println!("Found JRE:");
            println!("  Path: {:?}", jre.path);
            println!("  Version: {}", jre.version);
            println!("  64-bit: {}", jre.is_64bit);
}
    }
}