<template>
  <v-container>
    <h1 class="text-h4 mb-4">设置</h1>
    
    <v-card>
      <v-tabs
        v-model="activeTab"
        color="primary"
        align-tabs="start"
      >
        <v-tab value="launcher">启动器设置</v-tab>
        <v-tab value="jre">JRE 列表</v-tab>
      </v-tabs>

      <v-card-text>
        <v-window v-model="activeTab">
          <!-- 启动器设置 -->
          <v-window-item value="launcher">
            <v-form class="mt-4">
              <v-text-field
                v-model="maxMemory"
                label="最大内存 (MB)"
                type="number"
                variant="outlined"
                density="comfortable"
              ></v-text-field>

              <v-text-field
                v-model="javaArgs"
                label="Java 参数"
                variant="outlined"
                density="comfortable"
                hint="添加自定义 Java 启动参数"
                persistent-hint
              ></v-text-field>

              <v-switch
                v-model="closeAfterLaunch"
                label="启动游戏后关闭启动器"
                color="primary"
                inset
                class="mt-4"
              ></v-switch>
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
                  <th>操作</th>
                </tr>
              </thead>
              <tbody>                <tr v-for="jre in jreList" :key="jre.path">
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
                  <td>
                    <v-btn
                      icon="mdi-delete"
                      variant="text"
                      color="error"
                      density="comfortable"
                      @click="removeJre(jre)"
                    ></v-btn>
                  </td>
                </tr>
              </tbody>
            </v-table>

            <v-btn
              color="primary"
              class="mt-4"
              prepend-icon="mdi-plus"
              @click="addJre"
            >
              添加 JRE
            </v-btn>
          </v-window-item>
        </v-window>
      </v-card-text>
    </v-card>
  </v-container>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { JreInfo } from '../types/jre'

// 当前激活的选项卡
const activeTab = ref('launcher')

// 启动器设置
const maxMemory = ref(2048)
const javaArgs = ref('')
const closeAfterLaunch = ref(false)

// JRE 列表
const jreList = ref<JreInfo[]>([])

// 加载 JRE 列表
const loadJreList = async () => {
  try {
    jreList.value = await invoke<JreInfo[]>('scan_all_jres')
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
const addJre = () => {
  // TODO: 实现添加 JRE 的逻辑
}

// 移除 JRE
const removeJre = async (jre: JreInfo) => {
  try {
    await invoke('remove_jre', { path: jre.path })
    await loadJreList()
  } catch (error) {
    console.error('Failed to remove JRE:', error)
  }
}
</script>

<style scoped>
.v-table {
  background: transparent !important;
}
</style>
