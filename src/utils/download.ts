import { event, path } from "@tauri-apps/api";
import { ClientJson } from "../types/version";
import { getMavenJarUrl, getMavenPathList, isPathExists } from "./utils";
import { Channel, invoke } from "@tauri-apps/api/core";
import { DownloadEvent } from "../types/event";
import { useDownloadDialogStore } from "../composables/useDownloadDialog";
import { useSnackbar } from "../composables/useSnackbar";

const downloadDialogStore = useDownloadDialogStore();
const { showError } = useSnackbar();
export async function downloadFiles(files: { [url: string]: string}, waitForNext?: boolean) {
    const onEvent = new Channel<DownloadEvent>();
    onEvent.onmessage = async (event: DownloadEvent) => {
        if (event.event === 'progress') {
            // Update the download dialog with progress
            downloadDialogStore.addOrUpdateItem({
                id: event.data.id,
                path: event.data.path,
                progress: event.data.progress,
                speed: event.data.speed,
            });
        } else if (event.event === 'error') {
            // Handle download error
            if (event.data.error != "\"canceled\"") {
                showError(`下载发生错误：${event.data.error}`)
            }
            // 下载失败后，取消所有下载任务并清空任务列表
            for (const item of downloadDialogStore.items) {
                invoke('cancel_download', { id: item.id })
                downloadDialogStore.removeItem(item.id);
            }
        } else if (event.event === 'finished' && !waitForNext) {
            // Handle download finished
            downloadDialogStore.clear();
        }
    };
    await invoke("download_files", { onEvent, files });
}

export async function downloadFilesWithoutOverwrite(files: { [url: string]: string }, waitForNext?: boolean) {
    const filesToDownload: { [url: string]: string } = {};
    for (const [url, dest] of Object.entries(files)) {
        if (!(await isPathExists(dest))) {
            filesToDownload[url] = dest;
        }
    }
    if (Object.keys(filesToDownload).length > 0) {
        await downloadFiles(filesToDownload, waitForNext);
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
