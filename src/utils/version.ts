import { path } from "@tauri-apps/api";
import { useLauncherConfigStore } from "../composables/useConfig";
import { appLocale } from "../main";
import { getCurrentGamePath, isPathExists, readLocalJson } from "./utils";
import { DownloadSource } from "../types/config/launcher";
import { invoke } from "@tauri-apps/api/core";
import { ClientJson, VersionInfo, VersionManifest } from "../types/version";
import { downloadFiles, downloadFilesWithoutOverwrite, getLibrariesToDownloadByClientJsons } from "./download";
const launcherConfigStore = useLauncherConfigStore();
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

export async function installGameVersion(vanillaVersionId: string, versionName: string, modLoaders: Object, downloadSource: DownloadSource, version_manifest: VersionManifest): Promise<boolean> {
  const isInstallForgeAndLiteLoader = "forge" in modLoaders && "liteloader" in modLoaders
  const isInstallForgeAndOptifine = "forge" in modLoaders && "optifine" in modLoaders
  const gamePath = getCurrentGamePath()
  const versionsPath = await path.join(gamePath, 'versions')
  const librariesPath = await path.join(gamePath, 'libraries')
  const gameVersionDestPath = await path.join(versionsPath, versionName, `${versionName}.json`)
  var vanillaJsonDownloadUrl = getVersionInfoInManifestById(vanillaVersionId, version_manifest).url
  var filesToDownload: { [key: string]: string } = {}
  var clientJsons: ClientJson[] = []
  var clientJsonPathsToRead: string[] = []
  if ("fabric" in modLoaders) { // 安装 Fabric
    const fabricVersion = modLoaders['fabric']
    const vanillaVersionDestPath = await path.join(versionsPath, vanillaVersionId, `${vanillaVersionId}.json`)
    switch (downloadSource) {
      case DownloadSource.Official:
        var fabricJsonDownloadUrl = `https://meta.fabricmc.net/v2/versions/loader/${vanillaVersionId}/${fabricVersion}/profile/json`
        break
      case DownloadSource.BmclApi:
        var fabricJsonDownloadUrl = `https://bmclapi2.bangbang93.com/fabric-meta/v2/versions/loader/${vanillaVersionId}/${fabricVersion}/profile/json`
        var vanillaJsonDownloadUrl = vanillaJsonDownloadUrl.replace('piston-meta.mojang.com', 'bmclapi2.bangbang93.com');
        break
    }
    try {
      filesToDownload[fabricJsonDownloadUrl] = gameVersionDestPath
      clientJsonPathsToRead.push(gameVersionDestPath)
      if (!await isPathExists(vanillaVersionDestPath)) {
        console.log(`Vanilla version file not found, downloading: ${vanillaVersionDestPath}`);
        filesToDownload[vanillaJsonDownloadUrl] = vanillaVersionDestPath
        clientJsonPathsToRead.push(vanillaVersionDestPath)
      }
      await downloadFilesWithoutOverwrite(filesToDownload)
      for (const clientJsonPath of clientJsonPathsToRead) {
        const clientJson = await readLocalJson<ClientJson>(clientJsonPath)
        if (clientJson) {
          clientJsons.push(clientJson)
        }
      }
      const libsToDownload = await getLibrariesToDownloadByClientJsons(clientJsons, librariesPath)
      console.log(libsToDownload)
      if (libsToDownload) {
        await downloadFilesWithoutOverwrite(libsToDownload)
      }

    } catch (error) {
      console.error("安装游戏版本时出错:", error);
      return false
    }
    return true
  } else if (Object.keys(modLoaders).length == 0) { // 仅安装原版
    try {
      filesToDownload[vanillaJsonDownloadUrl] = gameVersionDestPath
      clientJsonPathsToRead.push(gameVersionDestPath)
      await downloadFiles(filesToDownload)
      const clientJson = await readLocalJson<ClientJson>(gameVersionDestPath)
      clientJsons.push(clientJson)
      const libsToDownload = await getLibrariesToDownloadByClientJsons(clientJsons, librariesPath)
      if (libsToDownload) {
        await downloadFilesWithoutOverwrite(libsToDownload)
      }
    } catch {
      return false
    }
  }
  return true
}