<template>
  <v-container>
    <h1 class="text-h4 mb-4">{{ $t('setting.setting') }}</h1>

    <v-card>
      <v-tabs v-model="activeTab" color="primary" align-tabs="start">
        <v-tab value="launcher">{{ $t('setting.launcher_setting') }}</v-tab>
        <v-tab value="jre">{{ $t('setting.jre_list') }}</v-tab>
      </v-tabs>

      <v-card-text>
        <v-window v-model="activeTab">
          <!-- 启动器设置 -->
          <v-window-item value="launcher">
            <v-form class="mt-4">
              <h3 class="text-h6 mb-2">{{ $t('setting.general') }}</h3>
              <v-switch v-model="closeAfterLaunch" :label="$t('setting.close_after_launch')" color="primary" inset class="mt-4"></v-switch>
              <v-select v-model="language" :items="languageOptions" :label="$t('setting.language')" class="mt-4"
                item-title="label" item-value="value" />

              <v-divider class="my-4"></v-divider>

              <h3 class="text-h6 mb-2">{{ $t('setting.download') }}</h3>
              <v-select v-model="downloadSource" :items="downloadSourceOptions" :label="$t('setting.download_source')" class="mb-4"
                item-title="label" item-value="value" />

              <v-divider class="my-4"></v-divider>
              <h3 class="text-h6 mb-2">{{ $t('setting.color_theme') }}</h3>
              <v-btn-group rounded="pill" class="mt-2">
                <v-btn v-for="option in themeOptions" :key="option.value" :prepend-icon="option.icon"
                  :color="colorTheme === option.value ? 'primary' : undefined"
                  :variant="colorTheme === option.value ? 'flat' : 'text'" @click="colorTheme = option.value">
                  {{ option.text }}
                </v-btn>
              </v-btn-group>

              <v-divider class="my-4"></v-divider>

              <h3 class="text-h6 mb-2">{{ $t('setting.proxy') }}</h3>
              <v-switch v-model="proxyEnabled" :label="$t('setting.enable_proxy')" color="primary" inset class="mt-2" />
              <v-text-field v-model="proxyHost" :label="$t('setting.proxy_host')" :disabled="!proxyEnabled" class="mb-2" />
              <v-switch v-model="proxyAuthEnabled" :label="$t('setting.enable_proxy_auth')" color="primary" inset class="mt-2" :disabled="!proxyEnabled" />
              <v-text-field v-model="proxyUsername" :label="$t('general.username')" :disabled="!proxyEnabled || !proxyAuthEnabled" class="mb-2" />
              <v-text-field v-model="proxyPassword" :label="$t('general.password')" :type="showProxyPassword ? 'text' : 'password'" :append-icon="showProxyPassword ? 'mdi-eye-off' : 'mdi-eye'" @click:append="showProxyPassword = !showProxyPassword" :disabled="!proxyEnabled || !proxyAuthEnabled" class="mb-2" />
            </v-form>
          </v-window-item>

          <!-- JRE 列表 -->
          <v-window-item value="jre">
            <v-table class="mt-4">
              <thead>
                <tr>
                  <th>选择</th>
                  <th>路径</th>
                  <th>版本</th>
                  <th>架构</th>
                  <th>提供者</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="jre in jreList" :key="jre.path">
                  <td>
                    <v-radio-group v-model="selectedJrePath" class="ma-0 pa-0" hide-details>
                      <v-radio :value="jre.path" color="primary" density="compact" />
                    </v-radio-group>
                  </td>
                  <td class="text-no-wrap">
                    <v-tooltip :text="jre.path">
                      <template v-slot:activator="{ props }">
                        <span v-bind="props" class="text-truncate d-inline-block" style="max-width: 300px">
                          {{ jre.path }}
                        </span>
                      </template>
                    </v-tooltip>
                  </td>
                  <td>{{ jre.version }}</td>
                  <td>{{ jre.arch }}</td>
                  <td>{{ jre.implementor }}</td>
                  <td>
                    <v-btn v-if="jre.manual" icon="mdi-close" variant="text" color="error" density="comfortable"
                      @click="removeJre(jre)"></v-btn>
                  </td>
                </tr>
              </tbody>
            </v-table>

            <v-btn color="primary" class="mt-4" prepend-icon="mdi-plus" @click="addJre">
              添加 JRE
            </v-btn>
          </v-window-item>
        </v-window>
      </v-card-text>
    </v-card>

    <!-- 移除确认对话框 -->
    <v-dialog v-model="showDeleteDialog" max-width="400">
      <v-card>
        <v-card-title class="text-h6">
          确认移除
        </v-card-title>
        <v-card-text>
          确定要移除此 JRE 吗？
          <div class="mt-2">
            <strong>路径：</strong>{{ jreToDelete?.path }}
            <br>
            <strong>版本：</strong>{{ jreToDelete?.version }}
          </div>
        </v-card-text>
        <v-card-actions>
          <v-spacer></v-spacer>
          <v-btn color="primary" variant="text" @click="showDeleteDialog = false">
            取消
          </v-btn>
          <v-btn color="error" variant="text" @click="confirmDeleteJre">
            移除
          </v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { JreInfo } from '../types/config/jre'
import { open } from '@tauri-apps/plugin-dialog'
import { JreConfig } from '../types/config/jre'
import { ColorTheme, DownloadSource, type LauncherConfig } from '../types/config/launcher'
import { useAppTheme } from '../composables/useTheme'
import { useSnackbar } from '../composables/useSnackbar'
import { useLauncherConfigStore } from '../composables/useConfig'
import { useI18n } from 'vue-i18n'
import { i18n } from '../main'

// 当前激活的选项卡
const activeTab = ref('launcher')
const { t } = useI18n()
// 主题选项
const themeOptions = [
  { text: t('setting.color_themes.follow_system'), value: ColorTheme.FollowSystem, icon: 'mdi-theme-light-dark' },
  { text: t('setting.color_themes.light'), value: ColorTheme.Light, icon: 'mdi-white-balance-sunny' },
  { text: t('setting.color_themes.dark'), value: ColorTheme.Dark, icon: 'mdi-weather-night' }
]

// JRE 列表
const jreList = ref<JreInfo[]>([])
const selectedJrePath = ref<string | null>(null)
const showDeleteDialog = ref(false)
const jreToDelete = ref<JreInfo | null>(null)
const launcherConfigStore = useLauncherConfigStore()
const closeAfterLaunch = ref(launcherConfigStore.config.closeAfterLaunch)
const { colorTheme } = useAppTheme()
const { showError } = useSnackbar()



// 下载源选项
const downloadSourceOptions = [
  { label: t('setting.download_sources.official'), value: DownloadSource.Official },
  { label: t('setting.download_sources.bmclapi'), value: DownloadSource.BmclApi },
]
const downloadSource = ref(DownloadSource.Official)

const language = ref(launcherConfigStore.config.language ?? 'en')
const languageOptions = [
  { label: 'English', value: 'en' },
  { label: '简体中文', value: 'zh-hans' },
  { label: '繁體中文', value: 'zh-hant' },
  { label: '日本語', value: 'ja' },
  
]


// 代理服务器设置
const proxyEnabled = ref(launcherConfigStore.config.enableProxy ?? false)
const proxyHost = ref(launcherConfigStore.config.proxy.host ?? '')
const proxyAuthEnabled = ref(launcherConfigStore.config.proxy.enableAuth ?? false)
const proxyUsername = ref(launcherConfigStore.config.proxy.username ?? '')
const proxyPassword = ref(launcherConfigStore.config.proxy.password ?? '')
const showProxyPassword = ref(false)

watch([proxyEnabled, proxyHost, proxyAuthEnabled, proxyUsername, proxyPassword], async ([enabled, host, auth, user, pass]) => {
  if (launcherConfigStore.config) {
    launcherConfigStore.config.enableProxy = enabled
    launcherConfigStore.config.proxy.host = host
    launcherConfigStore.config.proxy.enableAuth = auth
    launcherConfigStore.config.proxy.username = user
    launcherConfigStore.config.proxy.password = pass
    launcherConfigStore.saveConfig()
    await invoke('update_reqwest_client', { config: launcherConfigStore.config })
  }
})

// 监听下载源变化
watch(downloadSource, async (newSource) => {
  try {
    if (launcherConfigStore.config) {
      launcherConfigStore.config.downloadSource = newSource
    }
    launcherConfigStore.saveConfig()
  } catch (error: string | any) {
    showError(error)
  }
})

watch(language, async (newLanguage) => {
  i18n.global.locale = newLanguage as typeof i18n.global.locale
  try {
    if (launcherConfigStore.config) {
      launcherConfigStore.config.language = newLanguage
    }
    launcherConfigStore.saveConfig()
  } catch (error: string | any) {
    showError(error)
  }
})

// 监听选中JRE变化
watch(selectedJrePath, async (newPath) => {

  console.log('JRE切换为:', newPath)
})

// 监听关闭启动器设置变化
watch(closeAfterLaunch, async (newValue) => {
  try {
    if (launcherConfigStore.config) {
      launcherConfigStore.config.closeAfterLaunch = newValue ?? false
    }
    launcherConfigStore.saveConfig()
  } catch (error: string | any) {
    showError(error)
  }
})

// 组件挂载时加载配置
onMounted(() => {
  console.log(launcherConfigStore.config)
  colorTheme.value = launcherConfigStore.config.colorTheme
  downloadSource.value = launcherConfigStore.config.downloadSource
})

// 加载 JRE 列表
const loadJreList = async () => {
  try {
    const jres = await invoke<JreInfo[]>('get_all_jres')
    jreList.value = jres
    // 如果当前选中的JRE被移除，则自动选择第一个
    if (jreList.value.length > 0) {
      if (!jreList.value.some(jre => jre.path === selectedJrePath.value)) {
        selectedJrePath.value = jreList.value[0].path
      }
    } else {
      selectedJrePath.value = null
    }
  } catch (error: string | any) {
    showError(error)
  }
}

// 当切换到 JRE 列表标签时加载数据
watch(activeTab, (newValue) => {
  if (newValue === 'jre') {
    loadJreList()
  }
})

// 添加 JRE
const addJre = async () => {
  const jre_directory = await open({
    title: t('setting.select_jre_directory_title'),
    directory: true,
  })
  console.log('Selected JRE dir:', jre_directory)
  try {
    const result = await invoke<JreInfo>('get_jre_info', { path: jre_directory })
    console.log(result)

    // 检查是否已存在相同的JRE
    const isDuplicate = jreList.value.some(jre =>
      jre.path === result.path &&
      jre.version === result.version &&
      jre.arch === result.arch
    )

    if (!isDuplicate) {
      // 设置为手动添加的JRE
      result.manual = true

      const jre_config = await invoke<JreConfig>('get_jre_config_command')
      jre_config.jres.push(result)
      await invoke('save_jre_config_command', { config: jre_config })
      // 重新加载列表以显示新添加的JRE
      await loadJreList()
    } else {
      console.log('JRE already exists, skipping...')
    }
  } catch (error: string | any) {
    showError(error)
  }
}

// 移除 JRE
const removeJre = async (jre: JreInfo) => {
  jreToDelete.value = jre
  showDeleteDialog.value = true
}

// 确认移除 JRE
const confirmDeleteJre = async () => {
  if (!jreToDelete.value) return

  try {
    await invoke('remove_jre', { jre: jreToDelete.value })
    await loadJreList()
  } catch (error: string | any) {
    showError(error)
  } finally {
    showDeleteDialog.value = false
    jreToDelete.value = null
  }
}
</script>

<style scoped>
.v-table {
  background: transparent !important;
}
</style>
