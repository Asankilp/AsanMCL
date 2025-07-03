import { path } from "@tauri-apps/api";
import { Library, ClientJson } from "../types/version";
import { getOsType, getArchBits, getMavenJarUrl, getMavenPathList } from "./utils";

function checkLibraryRules(
    lib: Library,
    osType: string,
    arch: number,
    features?: Record<string, boolean>
): boolean {
    const rules = lib.rules;
    if (!rules || rules.length === 0) return true;

    let hasAllowRule = false;

    for (const rule of rules) {
        let osMatch = true;
        let archMatch = true;
        let versionMatch = true;
        let featuresMatch = true;

        // OS 匹配判断
        if (rule.os) {
            if (rule.os.name && rule.os.name !== osType) osMatch = false;
            if (rule.os.arch && rule.os.arch !== String(arch)) archMatch = false;
            // TODO: version匹配
        }

        // Features 匹配判断
        if (rule.features && features) {
            for (const key in rule.features) {
                if (features[key] !== rule.features[key]) {
                    featuresMatch = false;
                    break;
                }
            }
        }

        const matched = osMatch && archMatch && versionMatch && featuresMatch;

        // 若 disallow 匹配 → 立即 false
        if (rule.action === 'disallow' && matched) {
            return false;
        }

        // 若 allow 匹配 → 标记为允许
        if (rule.action === 'allow') {
            hasAllowRule = true;
            if (matched) return true;
        }
    }

    // 存在 allow 但都不匹配 → false
    if (hasAllowRule) return false;

    // 无匹配、无有效 allow/disallow → 默认 false
    return false;
}

async function getLibraryTargetPath(lib: Library, librariesPath: string, osType: string, arch: number, features?: Record<string, boolean>): Promise<{ artifact?: string, native?: string }> {
    if (!checkLibraryRules(lib, osType, arch, features)) return {};
    let artifact, native;
    if (lib.downloads?.artifact?.path) {
        artifact = await path.join(librariesPath, lib.downloads.artifact.path);
    }
    if (lib.natives && lib.downloads?.classifiers) {
        const nativeKeyTemplate = lib.natives[osType];
        if (nativeKeyTemplate) {
            const nativeKey = nativeKeyTemplate.replace("${arch}", arch.toString());
            const nativeObj = lib.downloads.classifiers[nativeKey];
            if (nativeObj && nativeObj.path) {
                native = await path.join(librariesPath, nativeObj.path);
            }
        }
    }
    return { artifact, native };
}

export async function getLibrariesToDownloadByClientJsons(
    clientJsons: ClientJson[],
    librariesPath: string,
    features?: Record<string, boolean>
): Promise<{ [url: string]: string } | undefined> {

    const filesToDownload: { [url: string]: string } = {};

    const osType = getOsType();
    const arch = getArchBits();
    for (const clientJson of clientJsons) {
        const libraries = clientJson.libraries;
        if (!libraries) continue;

        for (const lib of libraries) {
            if (!lib.name) continue;
            if (!checkLibraryRules(lib, osType, arch, features)) continue;

            const { artifact, native } = await getLibraryTargetPath(lib, librariesPath, osType, arch, features);
            console.log(native)
            // ✅ 处理普通 artifact
            if (lib.downloads?.artifact?.url && artifact) {
                filesToDownload[lib.downloads.artifact.url] = artifact;
            } else if (lib.url) {
                const url = getMavenJarUrl(lib.name, lib.url);
                const basePath = await path.join(librariesPath, ...getMavenPathList(lib.name));
                filesToDownload[url] = basePath;
            }

            // ✅ 处理 native classifier
            if (lib.natives && lib.downloads?.classifiers) {
                const nativeKeyTemplate = lib.natives[osType];
                if (!nativeKeyTemplate) continue;

                const nativeKey = nativeKeyTemplate.replace("${arch}", arch.toString());
                const nativeObj = lib.downloads.classifiers[nativeKey];
                if (nativeObj && nativeObj.url && native) {
                    filesToDownload[nativeObj.url] = native;
                }
            }
        }
    }
    console.log(filesToDownload)
    return Object.keys(filesToDownload).length > 0 ? filesToDownload : undefined;
}