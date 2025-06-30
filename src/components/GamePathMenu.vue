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
        :title="$t(pathName)"
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
        <v-list-item-title>{{ $t('game.dir.add_game_dir') }}</v-list-item-title>
      </v-list-item>
    </v-list>
  </v-menu>
  <!-- 添加目录名称对话框 -->
  <v-dialog v-model="showNameDialog" persistent max-width="400">
    <v-card>
      <v-card-title>{{ $t('game.dir.enter_dir_name') }}</v-card-title>
      <v-card-text>
        <v-text-field v-model="newGamePathName" :label="$t('game.dir.dir_name')" autofocus @keyup.enter="confirmAddGamePath" />
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="primary" @click="confirmAddGamePath">{{ $t('general.confirm') }}</v-btn>
        <v-btn text @click="cancelAddGamePath">{{ $t('general.cancel') }}</v-btn>
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
import { useI18n } from 'vue-i18n'
const { showError } = useSnackbar()

const launcherConfigStore = useLauncherConfigStore()
const config = launcherConfigStore.config
const { t } = useI18n()

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
      title: t('game.dir.select_game_dir')
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
      showError(t('general.name_existed'))
      return
    }
    config.gamePath[name] = pendingGamePath.value
    config.lastGamePath = name
    await launcherConfigStore.saveConfig()
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
    showError(t('game.dir.at_least_one_directory'))
    return
  }
  const confirmed = await ask(t('game.dir.confirm_remove_game_dir_hint', { path: t(pathName) }), { title: t('general.confirm_remove'), kind: 'warning' })
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
  await launcherConfigStore.saveConfig()
}
</script>

<style scoped>
.add-game-path-item {
  color: var(--v-theme-primary);
}
</style>
