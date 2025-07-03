import { path } from "@tauri-apps/api";
import { appLocale } from "../main";
import { getCurrentGamePath, isPathExists, readLocalJson } from "./utils";
import { DownloadSource } from "../types/config/launcher";
import { ClientJson, VersionInfo, VersionManifest } from "../types/version";
import { downloadFiles, downloadFilesWithoutOverwrite, getLibrariesToDownloadByClientJsons } from "./download";
import { useDownloadDialogStore } from "../composables/useDownloadDialog";
const downloadDialogStore = useDownloadDialogStore();
/**
 * 根据版本号获取更新主题。
 * @param versionId 版本号字符串
 * @returns 更新主题，未找到则返回空字符串
 */
export async function getMajorUpdateThemeById(versionId: string): Promise<string> {
  try {
    // 动态加载 major_updates.json（Vite 静态资源）
    const locale = appLocale.value
    const url = new URL(`../assets/data/major_updates/${locale}.json`, import.meta.url).toString();
    const res = await fetch(url);
    if (!res.ok) return '';
    const data = await res.json();
    return data[versionId] || '';
  } catch (e) {
    return '';
  }
}

export function getVersionInfoInManifestById(versionId: string, versionManifest: VersionManifest): VersionInfo {
  const versionInfo = versionManifest.versions.find(v => v.id === versionId);
  if (!versionInfo) {
    throw new Error(`Version ${versionId} not found in manifest`);
  }
  return versionInfo;
}

export async function installGameVersion(
  vanillaVersionId: string,
  versionName: string,
  modLoaders: Record<string, string>,
  downloadSource: DownloadSource,
  version_manifest: VersionManifest
): Promise<boolean> {
  const gamePath = getCurrentGamePath();
  const versionsPath = await path.join(gamePath, 'versions');
  const librariesPath = await path.join(gamePath, 'libraries');
  const gameVersionDestPath = await path.join(versionsPath, versionName, `${versionName}.json`);
  const vanillaJsonDownloadUrl = getVersionInfoInManifestById(vanillaVersionId, version_manifest).url;
  let filesToDownload: { [key: string]: string } = {};
  let clientJsonPathsToRead: string[] = [];

  // 处理 Fabric
  if (modLoaders.fabric) {
    const fabricVersion = modLoaders['fabric'];
    const vanillaVersionDestPath = await path.join(versionsPath, vanillaVersionId, `${vanillaVersionId}.json`);
    let fabricJsonDownloadUrl = '';
    switch (downloadSource) {
      case DownloadSource.Official:
        fabricJsonDownloadUrl = `https://meta.fabricmc.net/v2/versions/loader/${vanillaVersionId}/${fabricVersion}/profile/json`;
        break;
      case DownloadSource.BmclApi:
        fabricJsonDownloadUrl = `https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader/${vanillaVersionId}/${fabricVersion}/profile/json`;
        break;
    }
    // BMCLAPI 替换 vanilla json url
    const vanillaUrl = downloadSource === DownloadSource.BmclApi
      ? vanillaJsonDownloadUrl.replace('piston-meta.mojang.com', 'bmclapi2.bangbang93.com')
      : vanillaJsonDownloadUrl;
    filesToDownload[fabricJsonDownloadUrl] = gameVersionDestPath;
    clientJsonPathsToRead.push(gameVersionDestPath);
    if (!await isPathExists(vanillaVersionDestPath)) {
      filesToDownload[vanillaUrl] = vanillaVersionDestPath;
      clientJsonPathsToRead.push(vanillaVersionDestPath);
    }
  } else {
    // 仅原版
    filesToDownload[vanillaJsonDownloadUrl] = gameVersionDestPath;
    clientJsonPathsToRead.push(gameVersionDestPath);
  }

  try {
    // 下载 json 文件
    if (Object.keys(filesToDownload).length > 0) {
      if (modLoaders.fabric) {
        await downloadFilesWithoutOverwrite(filesToDownload, true);
      } else {
        await downloadFiles(filesToDownload, true);
      }
    }
    // 读取所有 clientJson
    const clientJsons: ClientJson[] = [];
    for (const clientJsonPath of clientJsonPathsToRead) {
      const clientJson = await readLocalJson<ClientJson>(clientJsonPath);
      if (clientJson) clientJsons.push(clientJson);
    }
    // 下载所有库
    const libsToDownload = await getLibrariesToDownloadByClientJsons(clientJsons, librariesPath);
    if (libsToDownload) {
      await downloadFilesWithoutOverwrite(libsToDownload);
    }
    downloadDialogStore.clear();
    return true;
  } catch (error) {
    console.error("安装游戏版本时出错:", error);
    return false;
  }
}