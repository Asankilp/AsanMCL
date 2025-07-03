<template>
  <v-container class="pa-0 fill-height" fluid>
    <!-- 顶部工具栏 -->
    <v-app-bar flat>
      <v-toolbar-title class="text-h6">{{ $t('version.version_list') }}</v-toolbar-title>
      <v-spacer></v-spacer>
      <!-- 游戏目录切换菜单组件 -->
      <GamePathMenu @switch="switchGamePath"
        @refreshVersions="refreshVersions" />
      <v-btn icon color="grey lighten-1" @click="showAddVersionDialog = true">
        <v-icon left>mdi-plus</v-icon>
      </v-btn>
    </v-app-bar>

    <!-- 版本列表 -->
    <v-main>
      <v-list lines="two">
        <v-list-item v-for="version in versions" :key="version.name" :title="version.name" rounded="lg">
          <template v-slot:prepend>
            <v-avatar size="40" rounded="lg">
              <img width="40" height="40" :src="getVersionIcon(version.info.type ?? VersionType.Other)" alt="icon">
            </v-avatar>
          </template>

          <template v-slot:append>
            <div class="d-flex align-center">
              <v-menu offset-y>
                <template v-slot:activator="{ props }">
                  <v-btn icon color="lighten-1" variant="plain" v-bind="props">
                    <v-icon>mdi-dots-vertical</v-icon>
                  </v-btn>
                </template>
                <v-list>
                  <v-list-item @click="handleMoreOptions(version)">
                    <v-list-item-title>{{ $t('general.more_options') }}</v-list-item-title>
                  </v-list-item>
                </v-list>
              </v-menu>
            </div>
          </template>
        </v-list-item>

        <v-list-item v-if="versions.length === 0" class="text-center">
          <v-list-item-title>{{ $t('version.no_version') }}</v-list-item-title>
          <v-list-item-subtitle>{{ $t('version.add_version_hint') }}</v-list-item-subtitle>
        </v-list-item>
      </v-list>

      <AddVersionDialog v-model="showAddVersionDialog" @close="showAddVersionDialog = false"
        v-if="launcherConfigStore.config" :launcher-config="launcherConfigStore.config"
        @confirm="onAddVersionConfirm" />
    </v-main>
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'
import { LocalVersionInfo, VersionType } from '../types/version'
import { useSnackbar } from '../composables/useSnackbar'
import GamePathMenu from '../components/GamePathMenu.vue'
import AddVersionDialog from '../components/AddVersionDialog.vue'
import { getVersionIcon } from '../utils/icon'
import { useLauncherConfigStore } from '../composables/useConfig'
import { onBeforeRouteLeave } from 'vue-router'
import { useDownloadDialogStore } from '../composables/useDownloadDialog'
const { showError } = useSnackbar()

const versions = ref<LocalVersionInfo[]>([])
const launcherConfigStore = useLauncherConfigStore()
const downloadDialogStore = useDownloadDialogStore()
const showAddVersionDialog = ref(false)



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
  try {
    console.log('切换游戏目录:', gamePath)
    versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath })
    if (launcherConfigStore.config) {
      launcherConfigStore.config.lastGamePath = gamePathName
    }
    launcherConfigStore.saveConfig()
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

const onAddVersionConfirm = () => {
  // 添加版本后刷新当前目录下的版本列表
  const gamePath = launcherConfigStore.config.gamePath[launcherConfigStore.config.lastGamePath]
  refreshVersions(gamePath)
}

onMounted(async () => {
  try {
    versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath: launcherConfigStore.config.gamePath[launcherConfigStore.config.lastGamePath] })
  } catch (error: string | any) {
    showError(error)
  }
})

onBeforeRouteLeave(() => {
  downloadDialogStore.clear()
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