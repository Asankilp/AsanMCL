import { path } from "@tauri-apps/api";
import { ClientJson } from "../types/version";
import { getMavenJarUrl, getMavenPathList, isPathExists } from "./utils";
import { invoke } from "@tauri-apps/api/core";

export async function downloadFiles(files: { [url: string]: string }) {
    await invoke("download_files", { files });
}

export async function downloadFilesWithoutOverwrite(files: { [url: string]: string }) {
    const filesToDownload: { [url: string]: string } = {};
    for (const [url, dest] of Object.entries(files)) {
        if (!(await isPathExists(dest))) {
            filesToDownload[url] = dest;
        }
    }
    if (Object.keys(filesToDownload).length > 0) {
        await downloadFiles(filesToDownload);
    }
}

export async function getLibrariesToDownloadByClientJsons(clientJsons: ClientJson[], librariesPath: string): Promise<{ [url: string]: string } | undefined> {
    var filesToDownload: { [url: string]: string } = {};
    for (const clientJson of clientJsons) {
        const libraries = clientJson.libraries;
        if (!libraries) {
            continue;
        }
        for (const lib of libraries) {
            if (!lib.name) {
                continue;
            }
            const libPath = await path.join(librariesPath, ...getMavenPathList(lib.name));
            if (lib.url) {
                filesToDownload[getMavenJarUrl(lib.name, lib.url)] = libPath;
            } else {
                if (lib.downloads) {
                    if (lib.downloads.artifact && lib.downloads.artifact.url && lib.downloads.artifact.path) {
                        filesToDownload[lib.downloads.artifact.url] = await path.join(librariesPath, lib.downloads.artifact.path);
                    }
                }
            }
        }
    }
    return Object.keys(filesToDownload).length > 0 ? filesToDownload : undefined;
}
