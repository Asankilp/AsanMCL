<template>
  <v-container class="pa-0 fill-height" fluid>
    <!-- 顶部工具栏 -->
    <v-app-bar flat>
      <v-toolbar-title class="text-h6">版本列表</v-toolbar-title>
      <v-spacer></v-spacer>
      
      <!-- 游戏目录切换菜单 -->
      <v-menu offset-y>
        <template v-slot:activator="{ props }">
          <v-btn
            icon
            color="grey lighten-1"
            v-bind="props"
          >
            <v-icon>mdi-folder-multiple</v-icon>
          </v-btn>
        </template>
        <v-list>
          <v-list-item
            v-for="(pathName, path) in gamePaths"
            :key="pathName"
            @click="switchGamePath(path)"
            :title=pathName
            :subtitle=path
          >
          </v-list-item>
        </v-list>
      </v-menu>
    </v-app-bar>

    <!-- 版本列表 -->
    <v-main>
      <v-list lines="two">
        <v-list-item
          v-for="version in versions"
          :key="version.id"
          :title="version.name"
          rounded="lg"
        >
          <template v-slot:prepend>
            <v-avatar size="40">
              <img :src="getVersionIcon(version)" alt="版本图标">
            </v-avatar>
          </template>

          <template v-slot:append>
            <div class="d-flex align-center">
              <v-btn
                icon
                color="primary"
                @click="launchVersion(version)"
              >
                <v-icon>mdi-play</v-icon>
              </v-btn>
              <v-menu offset-y>
                <template v-slot:activator="{ props }">
                  <v-btn
                    icon
                    color="grey lighten-1"
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

const versions = ref<any[]>([])
const gamePaths = ref<Record<string, string>>({})

const getVersionIcon = (version: any): string => {
  return `https://placehold.co/40x40?text=${encodeURIComponent(version.name)}`
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
const switchGamePath = (gamePath: any) => {
  console.log('切换游戏目录:', gamePath)
}

onMounted(async () => {
  const launcherConfig = await invoke<LauncherConfig>('get_launcher_config_command')
  gamePaths.value = launcherConfig.gamePath
})
</script>

<style scoped>
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