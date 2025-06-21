<template>
  <v-container class="pa-0 fill-height" fluid>
    <!-- 顶部工具栏 -->
    <v-app-bar flat>
      <v-toolbar-title class="text-h6">版本列表</v-toolbar-title>
      <v-spacer></v-spacer>
      <!-- 游戏目录切换菜单组件 -->
      <GamePathMenu
        :launcherConfig="launcherConfig"
        @switch="switchGamePath"
        @update:launcherConfig="val => launcherConfig = val"
        @refreshVersions="refreshVersions"
      />
    </v-app-bar>

    <!-- 版本列表 -->
    <v-main>
      <v-list lines="two">
        <v-list-item
          v-for="version in versions"
          :key="version.name"
          :title="version.name"
          rounded="lg"
        >
          <template v-slot:prepend>
            <v-avatar size="40" rounded="lg">
              <img width="40" height="40" :src="getVersionIcon(version)" alt="版本图标">
            </v-avatar>
          </template>

          <template v-slot:append>
            <div class="d-flex align-center">
              <v-menu offset-y>
                <template v-slot:activator="{ props }">
                  <v-btn
                    icon
                    color="lighten-1"
                    variant="plain"
                    v-bind="props"
                  >
                    <v-icon>mdi-dots-vertical</v-icon>
                  </v-btn>
                </template>
                <v-list>
                  <v-list-item @click="handleMoreOptions(version)">
                    <v-list-item-title>更多选项</v-list-item-title>
                  </v-list-item>
                </v-list>
              </v-menu>
            </div>
          </template>
        </v-list-item>

        <v-list-item v-if="versions.length === 0" class="text-center">
          <v-list-item-title>暂无版本</v-list-item-title>
          <v-list-item-subtitle>请添加或下载新版本</v-list-item-subtitle>
        </v-list-item>
      </v-list>
    </v-main>
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'
import { LauncherConfig } from '../types/config/launcher'
import { LocalVersionInfo, VersionType } from '../types/version'
import { useSnackbar } from '../composables/useSnackbar'
import GamePathMenu from '../components/GamePathMenu.vue'
import { getProfileIconUrl } from '../utils/icon'
const { showError } = useSnackbar()

const versions = ref<LocalVersionInfo[]>([])
const launcherConfig = ref<LauncherConfig>()

const getVersionIcon = (version: LocalVersionInfo): string => {
  const versionType = version.info?.type;
  switch (versionType) {
    case VersionType.Release:
      return getProfileIconUrl('grassblock');
    case VersionType.Snapshot:
      return getProfileIconUrl('dirt');
    case undefined:
    case null:
      return getProfileIconUrl('cobblestone');
    default:
      return getProfileIconUrl('cobblestone');
  }
}

// 启动版本
const launchVersion = async (version: any) => {
  console.log('启动版本:', version)
}

// 处理更多选项
const handleMoreOptions = (version: any) => {
  console.log('更多选项:', version)
}

// 切换游戏目录
const switchGamePath = async (gamePath: string, gamePathName: string) => {
  try{
    console.log('切换游戏目录:', gamePath)
    versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath })
    if (launcherConfig.value) {
      launcherConfig.value.lastGamePath = gamePathName
    }
    writeLauncherConfig()
  } catch (error: string | any) {
    showError(error)
  }
}

// 刷新版本
const refreshVersions = async (gamePath: string) => {
  if (gamePath) {
    versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath })
  } else {
    versions.value = []
  }
}

const writeLauncherConfig = async () => {
  await invoke('save_launcher_config_command', { config: launcherConfig.value })
}

const loadLauncherConfig = async () => {
  launcherConfig.value = await invoke<LauncherConfig>('get_launcher_config_command')
}

onMounted(async () => {
  try {
    loadLauncherConfig()
    const launcherConfig = await invoke<LauncherConfig>('get_launcher_config_command')
    versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath: launcherConfig.gamePath[launcherConfig.lastGamePath] })
  } catch (error: string | any) {
    showError(error)
  }
})
</script>

<style>
.v-list-item {
  margin-inline: 12px;
  border-radius: 8px !important;
}

.v-list-item--active {
  background: rgb(var(--v-theme-primary), 0.15) !important;
}

.v-list-item:hover {
  background: rgb(var(--v-theme-primary), 0.05) !important;
} 
</style>