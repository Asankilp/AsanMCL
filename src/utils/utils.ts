import { invoke } from "@tauri-apps/api/core";
import { useLauncherConfigStore } from "../composables/useConfig";
import { platform, type as osType, arch as osArch } from "@tauri-apps/plugin-os";
const launcherConfigStore = useLauncherConfigStore();

/**
 * 获取当前的游戏路径。
 *
 * 从启动器配置中读取上次选择的游戏路径标识，并返回对应的路径。
 *
 * @returns 当前游戏路径的字符串表示。
 */
export function getCurrentGamePath(): string {
    const currentGamePathName = launcherConfigStore.config.lastGamePath;
    const currentGamePath = launcherConfigStore.config.gamePath[currentGamePathName];
    return currentGamePath;
}

/**
 * 检查指定路径是否存在。
 *
 * 调用后端的 `is_path_exists` 命令判断传入路径是否存在于文件系统中。
 *
 * @param path - 要检查的路径。
 * @returns 一个 Promise，解析为布尔值，表示路径是否存在。
 */
export async function isPathExists(path: string): Promise<boolean> {
    try {
        return await invoke<boolean>('is_path_exists', { path: path });
    } catch (error) {
        console.error(`检查路径 ${path} 是否存在时出错:`, error);
        return false;
    }
}

/**
 * 生成 Maven 仓库中指定 Jar 包的下载地址。
 *
 * 根据 Maven 坐标（格式为 `groupId:artifactId:version`）生成该 Jar 包在指定基础 URL 下的完整下载路径。
 *
 * @param name - Maven 坐标，例如 `net.fabricmc:fabric-loader:0.16.14`。
 * @param baseUrl - Maven 仓库的基础地址，例如 `https://maven.fabricmc.net`。
 * @returns 生成的 Jar 包下载 URL。
 * @throws 如果 Maven 坐标格式不合法，则抛出错误。
 */
export function getMavenJarUrl(name: string, baseUrl: string): string {
    const parts = name.split(":");
    if (parts.length !== 3 && parts.length !== 4) {
        console.log(name)
        throw new Error("Invalid Maven coordinate format. Expected: groupId:artifactId:version or groupId:artifactId:version:classifier");
    }

    const [groupId, artifactId, version, classifier] = parts;
    const path = `${groupId.replace(/\./g, "/")}/${artifactId}/${version}`;
    const jarName = classifier
        ? `${artifactId}-${version}-${classifier}.jar`
        : `${artifactId}-${version}.jar`;

    return `${baseUrl.replace(/\/+$/, "")}/${path}/${jarName}`;
}

/**
 * 根据 Maven 坐标（groupId:artifactId:version）生成路径列表。
 * 
 * 示例输入: "net.fabricmc:fabric-loader:0.16.14"
 * 
 * 示例输出: ["net", "fabricmc", "fabric-loader", "0.16.14", "fabric-loader-0.16.14.jar"]
 * 
 * 可以使用诸如 `const localPath = path.join(...getMavenPathList(name));` 的方式来获取该库在本地的路径
 *
 * @param name - Maven 坐标字符串，格式为 "groupId:artifactId:version"
 * @returns 表示该库在仓库中保存路径的字符串数组
 * @throws 如果格式无效（不是三个冒号分隔的部分），将抛出错误
 */
export function getMavenPathList(name: string): string[] {
  const parts = name.split(":");
  if (parts.length !== 3 && parts.length !== 4) {
    console.log(name)
    throw new Error("Invalid Maven coordinate format. Expected: groupId:artifactId:version or groupId:artifactId:version:classifier");
  }

  const [groupId, artifactId, version, classifier] = parts;
  const groupPath = groupId.split(".");
  const jarFile = classifier
    ? `${artifactId}-${version}-${classifier}.jar`
    : `${artifactId}-${version}.jar`;

  return [...groupPath, artifactId, version, jarFile];
}

export async function readLocalJson<T>(path: string): Promise<T> {
    try {
        return await invoke<T>('read_local_json', { path: path });
    } catch (error) {
        console.error(`读取本地 JSON 文件 ${path} 时出错:`, error);
        throw error;
    }
}

export function getOsType(): "windows" | "linux" | "osx" {
    const currentOsType = osType();
    if (currentOsType === "windows") return "windows";
    if (currentOsType === "macos") return "osx";
    if (currentOsType === "linux") return "linux";
    throw new Error("Unsupported OS: " + currentOsType);
}

export function getArchBits(): 32 | 64 {
    const arch = osArch();
    switch (arch) {
        case 'x86':
        case 'arm':
        case 'mips':
        case 'powerpc':
            return 32;

        case 'x86_64':
        case 'aarch64':
        case 'mips64':
        case 'powerpc64':
        case 'riscv64':
        case 's390x':
        case 'sparc64':
            return 64;

        default:
            throw new Error(`Unknown architecture: ${arch}`);
    }
}
