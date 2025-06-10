use super::model::{Architecture, JreInfo};
use std::path::PathBuf;

/// 检查给定路径是否为有效的 JRE 安装
pub fn verify_jre_path(path: &PathBuf) -> Option<JreInfo> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    #[cfg(target_os = "windows")]
    let java_bin = path.join("bin").join("java.exe");
    #[cfg(not(target_os = "windows"))]
    let java_bin = path.join("bin").join("java");

    if !java_bin.exists() {
        return None;
    }

    let release_file = path.join("release");
    if !release_file.exists() {
        return None;
    }

    let file = File::open(release_file).ok()?;
    let reader = BufReader::new(file);

    let mut version = None;
    let mut detected_arch = None;
    let mut implementor = None;
    let manual = Some(false);

    // 从release文件获取版本和架构信息
    for line in reader.lines().flatten() {
        let line = line.to_lowercase();
        if line.starts_with("implementor=") {
            implementor = Some(
                line.trim_start_matches("implementor=")
                    .trim_matches('"')
                    .trim()
                    .to_string(),
            );
        }
        if line.starts_with("java_version=") {
            version = Some(
                line.trim_start_matches("java_version=")
                    .trim_matches('"')
                    .trim()
                    .to_string(),
            );
        } else if line.starts_with("os_arch=") {
            let arch = line.trim_start_matches("os_arch=").trim_matches('"').trim();
            detected_arch = match arch {
                "amd64" | "x86_64" => Some(Architecture::X86_64),
                "x86" | "i386" | "i586" | "i686" => Some(Architecture::X86),
                "aarch64" | "arm64" => Some(Architecture::Arm64),
                _ => None,
            };
        }
    }

    // 如果没有找到架构信息，使用系统默认架构
    let arch = detected_arch.unwrap_or_else(|| {
        #[cfg(target_pointer_width = "64")]
        {
            #[cfg(target_arch = "aarch64")]
            return Architecture::Arm64;
            #[cfg(target_arch = "x86_64")]
            return Architecture::X86_64;
        }
        #[cfg(target_pointer_width = "32")]
        return Architecture::X86;

        #[allow(unreachable_code)]
        Architecture::Unknown
    });

    // 如果找不到版本信息，返回None
    let version = version?;

    Some(JreInfo {
        path: path.to_owned(),
        version,
        arch,
        implementor,
        manual
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
    #[cfg(target_os = "windows")]
    {
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
        paths.extend(
            [
                r"C:\Program Files\Java",
                r"C:\Program Files (x86)\Java",
                r"C:\Program Files\Eclipse Adoptium",
                r"C:\Program Files (x86)\Eclipse Adoptium",
                r"C:\Program Files\Microsoft\jdk",
                r"C:\Program Files (x86)\Microsoft\jdk",
                r"C:\Program Files\Microsoft",
                r"C:\Program Files\Common Files\Oracle\Java",
                r"C:\Program Files\BellSoft\",
                r"C:\Program Files (x86)\BellSoft\",
                // format!("{}\\Packages\\Microsoft.4297127D64EC6_8wekyb3d8bbwe\\LocalCache\\Local\\runtime", std::env::var("LOCALAPPDATA").unwrap_or_default()).as_str(),
            ]
            .iter()
            .map(PathBuf::from),
        );
    }

    #[cfg(target_os = "linux")]
    {
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
        paths.extend(
            ["/usr/lib/jvm", "/usr/java", "/usr/local/java", "/opt/java"]
                .iter()
                .map(PathBuf::from),
        );
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // 使用 java_home 命令
        if let Ok(output) = Command::new("/usr/libexec/java_home").output() {
            if let Ok(path) = String::from_utf8(output.stdout) {
                paths.push(PathBuf::from(path.trim()));
            }
        }

        // macOS 常见安装路径
        paths.extend(
            [
                "/Library/Java/JavaVirtualMachines",
                "/System/Library/Java/JavaVirtualMachines",
            ]
            .iter()
            .map(PathBuf::from),
        );
    }

    paths
}

/// 查找系统中的第一个可用 JRE
pub fn find_jre() -> Option<JreInfo> {
    scan_jres().into_iter().next()
}

/// 扫描系统中所有可用的 JRE
pub fn scan_jres() -> Vec<JreInfo> {
    use std::collections::HashSet;
    use std::fs;

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
            println!("  Architecture: {:?}", jre.arch);
            println!("  Implementor: {:?}", jre.implementor);
        }
    }
    #[test]
    fn test_unknown_jre() {
        let jre = verify_jre_path(&PathBuf::from(r"/path/to/aaa"));
        assert!(jre.is_none());
    }
    #[test]
    fn test_potential_jre_paths() {
        let paths = get_potential_jre_paths();
        assert!(!paths.is_empty(), "Expected some potential JRE paths");
        for path in paths {
            println!("Potential JRE path: {:?}", path);
        }
    }
}
