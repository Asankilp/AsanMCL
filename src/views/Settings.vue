<template>
  <v-container>
    <h1 class="text-h4 mb-4">设置</h1>

    <v-card>
      <v-tabs v-model="activeTab" color="primary" align-tabs="start">
        <v-tab value="launcher">启动器设置</v-tab>
        <v-tab value="jre">JRE 列表</v-tab>
      </v-tabs>

      <v-card-text>
        <v-window v-model="activeTab">
          <!-- 启动器设置 -->
          <v-window-item value="launcher">
            <v-form class="mt-4">
              <v-switch v-model="closeAfterLaunch" label="启动游戏后关闭启动器" color="primary" inset class="mt-4"></v-switch>
              
              <v-divider class="my-4"></v-divider>
              
              <h3 class="text-h6 mb-2">颜色主题</h3>
              <v-btn-group rounded="pill" class="mt-2">
                <v-btn
                  v-for="option in themeOptions"
                  :key="option.value"
                  :prepend-icon="option.icon"
                  :color="colorTheme === option.value ? 'primary' : undefined"
                  :variant="colorTheme === option.value ? 'flat' : 'text'"
                  @click="colorTheme = option.value"
                >
                  {{ option.text }}
                </v-btn>
              </v-btn-group>
            </v-form>
          </v-window-item>

          <!-- JRE 列表 -->
          <v-window-item value="jre">
            <v-table class="mt-4">
              <thead>
                <tr>
                  <th>路径</th>
                  <th>版本</th>
                  <th>架构</th>
                  <th>提供者</th>
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="jre in jreList" :key="jre.path">
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
import { ColorTheme, type LauncherConfig } from '../types/config/launcher'
import { useAppTheme } from '../composables/useTheme'

// 当前激活的选项卡
const activeTab = ref('launcher')

// 启动器设置
const closeAfterLaunch = ref(false)
const { colorTheme } = useAppTheme()

// 主题选项
const themeOptions = [
  { text: '跟随系统', value: ColorTheme.FollowSystem, icon: 'mdi-theme-light-dark' },
  { text: '浅色', value: ColorTheme.Light, icon: 'mdi-white-balance-sunny' },
  { text: '深色', value: ColorTheme.Dark, icon: 'mdi-weather-night' }
]

// JRE 列表
const jreList = ref<JreInfo[]>([])
const showDeleteDialog = ref(false)
const jreToDelete = ref<JreInfo | null>(null)

// 加载启动器配置
const loadLauncherConfig = async () => {
  try {
    const config = await invoke<LauncherConfig>('get_launcher_config_command')
    closeAfterLaunch.value = config.closeAfterLaunch
    colorTheme.value = config.colorTheme
  } catch (error) {
    console.error('Failed to load launcher config:', error)
  }
}

// 监听关闭启动器设置变化
watch(closeAfterLaunch, async (newValue) => {
  try {
    const config = await invoke<LauncherConfig>('get_launcher_config_command')
    config.closeAfterLaunch = newValue
    await invoke('save_launcher_config_command', { config })
  } catch (error) {
    console.error('Failed to save close after launch setting:', error)
  }
})

// 组件挂载时加载配置
onMounted(() => {
  loadLauncherConfig()
})

// 加载 JRE 列表
const loadJreList = async () => {
  try {
    const jres = await invoke<JreInfo[]>('get_all_jres')
    jreList.value = jres
  } catch (error) {
    console.error('Failed to load JRE list:', error)
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
    title: '选择包含 bin 目录的 JRE 安装位置',
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
  } catch (error) {
    console.error('Failed to add JRE:', error)
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
  } catch (error) {
    console.error('Failed to remove JRE:', error)
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
