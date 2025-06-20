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
            v-for="(path, pathName) in launcherConfig?.gamePath"
            :key="pathName"
            @click="switchGamePath(path, pathName)"
            :title="pathName"
            :subtitle="path"
          >
            <template v-slot:prepend>
              <v-radio
                :model-value="launcherConfig?.lastGamePath === pathName"
                readonly
                hide-details
                density="compact"
              ></v-radio>
            </template>

            <!-- 新增：删除按钮 -->
            <template v-slot:append>
              <v-btn
                icon
                variant="plain"
                color="transparent"
                @click.stop="deleteGamePath(pathName)"
              >
                <v-icon color="error">mdi-delete</v-icon>
              </v-btn>
            </template>
          </v-list-item>
          <!-- 添加游戏目录菜单项 -->
          <v-list-item
            @click="addGamePath"
            class="add-game-path-item"
          >
            <template v-slot:prepend>
              <v-icon color="primary">mdi-folder-plus</v-icon>
            </template>
            <v-list-item-title>添加游戏目录</v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
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
            <v-avatar size="40">
              <img :src="getVersionIcon(version)" alt="版本图标">
            </v-avatar>
          </template>

          <template v-slot:append>
            <div class="d-flex align-center">
              <v-menu offset-y>
                <template v-slot:activator="{ props }">
                  <v-btn
                    icon
                    color="lighten-1"
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
    <!-- 新增：添加游戏目录名称对话框 -->
    <v-dialog v-model="showNameDialog" persistent max-width="400">
      <v-card>
        <v-card-title>请输入该目录的名称</v-card-title>
        <v-card-text>
          <v-text-field v-model="newGamePathName" label="目录名称" autofocus @keyup.enter="confirmAddGamePath" />
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="primary" @click="confirmAddGamePath">确定</v-btn>
          <v-btn text @click="cancelAddGamePath">取消</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { ref, onMounted } from 'vue'
import { LauncherConfig } from '../types/config/launcher'
import { LocalVersionInfo } from '../types/version'
import { useSnackbar } from '../composables/useSnackbar'
import { open, ask } from '@tauri-apps/plugin-dialog'
const { showError } = useSnackbar()

const versions = ref<LocalVersionInfo[]>([])
const launcherConfig = ref<LauncherConfig>()
const showNameDialog = ref(false)
const newGamePathName = ref('')
const pendingGamePath = ref<string | null>(null)

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

// 添加游戏目录
const addGamePath = async () => {
  try {
    const gamePath = await open({
      directory: true,
      multiple: false,
      title: '选择游戏目录'
    })
    if (!gamePath) return
    pendingGamePath.value = gamePath as string
    newGamePathName.value = ''
    showNameDialog.value = true
  } catch (error: string | any) {
    showError(error)
  }
}

const confirmAddGamePath = async () => {
  const name = newGamePathName.value.trim()
  if (!name || !pendingGamePath.value) return
  if (launcherConfig.value) {
    if (!launcherConfig.value.gamePath) launcherConfig.value.gamePath = {}
    console.log(pendingGamePath.value)
    launcherConfig.value.gamePath[name] = pendingGamePath.value
    launcherConfig.value.lastGamePath = name
    await writeLauncherConfig()
    await invoke("init_game_path_command", { path: pendingGamePath.value })
    // await loadLauncherConfig()
    versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath: pendingGamePath.value })
  }
  showNameDialog.value = false
  pendingGamePath.value = null
  newGamePathName.value = ''
}

const cancelAddGamePath = () => {
  showNameDialog.value = false
  pendingGamePath.value = null
  newGamePathName.value = ''
}

// 删除游戏目录
const deleteGamePath = async (pathName: string) => {
  if (!launcherConfig.value?.gamePath) return
  const keys = Object.keys(launcherConfig.value.gamePath)
  // 防止删除唯一一个游戏目录
  if (keys.length <= 1) {
    showError('至少需要保留一个游戏目录')
    return
  }
  const confirmed = await ask(`确定要删除游戏目录 "${pathName}" 吗？`, { title: "删除确认" })
  if (!confirmed) return
  // 删除目录
  delete launcherConfig.value.gamePath[pathName]
  if (launcherConfig.value.lastGamePath === pathName) {
    const newKeys = Object.keys(launcherConfig.value.gamePath)
    if (newKeys.length > 0) {
      launcherConfig.value.lastGamePath = newKeys[0]
      versions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath: launcherConfig.value.gamePath[newKeys[0]] })
    } else {
      launcherConfig.value.lastGamePath = ''
      versions.value = []
    }
  }
  await writeLauncherConfig()
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

const writeLauncherConfig = async () => {
  await invoke('save_launcher_config_command', { config: launcherConfig.value })
}

const loadLauncherConfig = async () => {
  launcherConfig.value = await invoke<LauncherConfig>('get_launcher_config_command')
}
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