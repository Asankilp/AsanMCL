<template>
  <v-menu offset-y>
    <template v-slot:activator="{ props }">
      <v-btn icon color="grey lighten-1" v-bind="props">
        <v-icon>mdi-folder-multiple</v-icon>
      </v-btn>
    </template>
    <v-list>
      <v-list-item
        v-for="(path, pathName) in config.gamePath"
        :key="pathName"
        @click="onSwitch(path, pathName)"
        :title="pathName"
        :subtitle="path"
      >
        <template v-slot:prepend>
          <v-radio
            :model-value="config.lastGamePath === pathName"
            readonly
            hide-details
            density="compact"
          ></v-radio>
        </template>
        <template v-slot:append>
          <v-btn
            icon
            variant="plain"
            color="transparent"
            @click.stop="onDelete(pathName)"
          >
            <v-icon color="error">mdi-delete</v-icon>
          </v-btn>
        </template>
      </v-list-item>
      <v-list-item @click="onAdd" class="add-game-path-item">
        <template v-slot:prepend>
          <v-icon>mdi-folder-plus</v-icon>
        </template>
        <v-list-item-title>添加游戏目录</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
  <!-- 添加目录名称对话框 -->
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
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open, ask } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { useLauncherConfigStore } from '../composables/useConfig'
import { useSnackbar } from '../composables/useSnackbar'
const { showError } = useSnackbar()

const launcherConfigStore = useLauncherConfigStore()
const config = launcherConfigStore.config

const emits = defineEmits<{
  (e: 'switch', path: string, pathName: string): void
  (e: 'refreshVersions', gamePath: string): void
}>()

const showNameDialog = ref(false)
const newGamePathName = ref('')
const pendingGamePath = ref<string | null>(null)

const onSwitch = async (path: string, pathName: string) => {
  emits('switch', path, pathName)
}

const onAdd = async () => {
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
  } catch (error: any) {
    showError(error)
  }
}

const confirmAddGamePath = async () => {
  const name = newGamePathName.value.trim()
  if (!name || !pendingGamePath.value) return
  if (config) {
    if (!config.gamePath) config.gamePath = {}
    if (Object.prototype.hasOwnProperty.call(config.gamePath, name)) {
      showError('该名称已存在')
      return
    }
    config.gamePath[name] = pendingGamePath.value
    config.lastGamePath = name
    await launcherConfigStore.saveConfig(config)
    await invoke('init_game_path_command', { path: pendingGamePath.value })
    emits('refreshVersions', pendingGamePath.value)
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

const onDelete = async (pathName: string) => {
  if (!config?.gamePath) return
  const keys = Object.keys(config.gamePath)
  if (keys.length <= 1) {
    showError('至少需要保留一个游戏目录')
    return
  }
  const confirmed = await ask(`确定要删除游戏目录 "${pathName}" 吗？`, { title: '删除确认' })
  if (!confirmed) return
  delete config.gamePath[pathName]
  if (config.lastGamePath === pathName) {
    const newKeys = Object.keys(config.gamePath)
    if (newKeys.length > 0) {
      config.lastGamePath = newKeys[0]
      emits('refreshVersions', config.gamePath[newKeys[0]])
    } else {
      config.lastGamePath = ''
      emits('refreshVersions', '')
    }
  }
  await launcherConfigStore.saveConfig(config)
}
</script>

<style scoped>
.add-game-path-item {
  color: var(--v-theme-primary);
}
</style>
