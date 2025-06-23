<template>
    <v-dialog v-model="dialogVisible" max-width="800">
        <v-card>
            <v-card-title>
                添加版本
            </v-card-title>
            <v-card-text style="max-height: 800px; overflow-y: auto;">
                <v-row align="center" class="mb-2" no-gutters>
                    <v-row align="center" no-gutters style="flex:0 0 auto; width:auto;">
                        <v-checkbox v-model="selectedTypes" label="正式版" :value="VersionType.Release" density="compact"
                            hide-details class="mr-2" />
                        <v-checkbox v-model="selectedTypes" label="快照版" :value="VersionType.Snapshot" density="compact"
                            hide-details class="mr-2" />
                        <v-checkbox v-model="selectedTypes" label="旧Beta版" :value="VersionType.OldBeta"
                            density="compact" hide-details class="mr-2" />
                        <v-checkbox v-model="selectedTypes" label="旧Alpha版" :value="VersionType.OldAlpha"
                            density="compact" hide-details />
                    </v-row>
                    <v-spacer />
                    <v-text-field v-model="searchText" label="搜索版本号" prepend-inner-icon="mdi-magnify" clearable
                        density="compact" style="max-width:500px;" />
                </v-row>
                <v-row justify="center" v-if="loading">
                    <v-progress-circular indeterminate color="primary" size="40" />
                </v-row>
                <v-alert v-else-if="errorMsg" type="error" class="mb-4">{{ errorMsg }}</v-alert>
                <v-radio-group v-else v-model="selectedVersionId">
                    <v-list>
                        <v-list-item v-for="version in filteredVersions" :key="version.id"
                            :active="selectedVersionId === version.id" @click="selectedVersionId = version.id"
                            style="cursor:pointer;">
                            <template v-slot:prepend>
                                <v-avatar rounded="lg" size="40" class="mr-2">
                                    <img :src="getVersionIcon(version.type)" width="40" height="40" alt="icon" />
                                </v-avatar>
                                <v-radio :value="version.id" @click.stop />
                            </template>
                            <v-list-item-title>
                                {{ version.id }}
                                <span v-if="versionThemes[version.id]" style="font-size: 12px; color: #888; margin-left: 8px;">{{ versionThemes[version.id] }}</span>
                            </v-list-item-title>
                            <v-list-item-subtitle>{{ version.releaseTime }}</v-list-item-subtitle>
                        </v-list-item>
                    </v-list>
                </v-radio-group>
            </v-card-text>
            <v-card-actions>
                <v-spacer />
                <v-btn text @click="dialogVisible = false; $emit('close')">关闭</v-btn>
                <v-btn color="primary" :disabled="!selectedVersionId" @click="showStep2 = true">下一步</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>

    <!-- 第二步对话框：选择组件 -->
    <v-dialog v-model="showStep2" max-width="400">
      <v-card>
        <v-card-title>选择需要安装的组件</v-card-title>
        <v-card-text>
          <v-row dense class="flex-column">
            <!-- Minecraft 卡片 -->
            <v-col cols="12">
              <v-card>
                <v-card-title class="d-flex align-center">
                    <v-avatar rounded="lg" size="40" class="mr-2">
                        <img :src="getProfileIconUrl('grassblock')" width="40" height="40" alt="icon" />
                    </v-avatar>
                  <v-checkbox v-model="selectedComponents" :value="'minecraft'" :disabled="true" :ripple="false" hide-details class="mr-2" />
                  Minecraft
                </v-card-title>
                <v-card-subtitle>{{ selectedVersionId }}</v-card-subtitle>
              </v-card>
            </v-col>
            <v-col cols="12" v-for="mod in modComponents" :key="mod.key">
              <v-card :outlined="true" :elevation="selectedComponents.includes(mod.key) ? 8 : 2" :class="{'card-clickable': selectedComponents.includes(mod.key)}" @click="selectedComponents.includes(mod.key) && openComponentVersion(mod.key)">
                <v-card-title class="d-flex align-center">
                  <v-avatar rounded="lg" size="40" class="mr-2">
                    <img :src="getProfileIconUrl(mod.icon as ProfileIcon)" width="40" height="40" alt="icon" />
                  </v-avatar>
                  <v-checkbox v-model="selectedComponents" :value="mod.key" hide-details class="mr-2" @click.stop />
                  <span :style="!selectedComponents.includes(mod.key) ? 'color:#aaa;' : ''">{{ mod.label }}</span>
                  <v-spacer />
                  <v-progress-circular v-if="componentLoading[mod.key]" indeterminate color="primary" size="20" class="ml-2" />
                </v-card-title>
                <v-card-subtitle v-if="componentVersions[mod.key]">{{ componentVersions[mod.key] }}</v-card-subtitle>
              </v-card>
            </v-col>
          </v-row>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn text @click="showStep2 = false">返回</v-btn>
          <v-btn color="primary" @click="confirmComponents">确定</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>

    <!-- 组件版本选择对话框 -->
    <v-dialog v-model="showComponentVersionDialog" max-width="600">
      <v-card>
        <v-card-title>{{ componentVersionTitle }}</v-card-title>
        <v-card-text style="max-height: 400px; overflow-y: auto;">
          <v-row justify="center" v-if="componentVersionLoading">
            <v-progress-circular indeterminate color="primary" size="40" />
          </v-row>
          <v-radio-group v-else v-model="componentVersionSelected">
            <v-list>
              <v-list-item v-for="item in componentVersionList" :key="item" :active="componentVersionSelected === item" @click="componentVersionSelected = item" style="cursor:pointer;">
                <template v-slot:prepend>
                  <v-radio :value="item" @click.stop />
                </template>
                <v-list-item-title>{{ item }}</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-radio-group>
        </v-card-text>
        <v-card-actions>
          <v-spacer />
          <v-btn text @click="showComponentVersionDialog = false">取消</v-btn>
          <v-btn color="primary" :disabled="!componentVersionSelected" @click="selectComponentVersion">确定</v-btn>
        </v-card-actions>
      </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed, PropType, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { VersionManifest, VersionType } from '../types/version'
import { LauncherConfig } from '../types/config/launcher'
import { getProfileIconUrl, getVersionIcon } from '../utils/icon'
import { getMajorUpdateThemeById } from '../utils/version'
import { ProfileIcon } from '../types/profile'

const props = defineProps({
    modelValue: Boolean,
    launcherConfig: {
        type: Object as PropType<LauncherConfig>,
        required: true
    }
})
const emit = defineEmits(['update:modelValue', 'close', 'confirm'])

const dialogVisible = computed({
    get: () => props.modelValue,
    set: v => emit('update:modelValue', v)
})

const loading = ref(false)
const errorMsg = ref('')
const remoteVersions = ref<VersionManifest>()
const selectedVersionId = ref<string | null>(null)
const searchText = ref('')
const selectedTypes = ref([VersionType.Release])
const versionThemes = ref<Record<string, string>>({})

watch(() => props.modelValue, async (val) => {
    if (val) {
        loading.value = true
        errorMsg.value = ''
        try {
            remoteVersions.value = await invoke<VersionManifest>('get_version_manifest', { downloadSource: props.launcherConfig.downloadSource })
        } catch (e: any) {
            errorMsg.value = e?.message || String(e) || '加载失败'
        } finally {
            loading.value = false
        }
    }
})

const filteredVersions = computed(() => {
    if (!remoteVersions.value?.versions) return []
    return remoteVersions.value.versions.filter(v => {
        const typeMatch = selectedTypes.value.includes(v.type)
        const searchMatch = !searchText.value || v.id.toLowerCase().includes(searchText.value.toLowerCase())
        return typeMatch && searchMatch
    })
})

watch(filteredVersions, async (versions) => {
  const themes: Record<string, string> = {}
  await Promise.all(versions.map(async v => {
    themes[v.id] = await getMajorUpdateThemeById(v.id)
  }))
  versionThemes.value = themes
}, { immediate: true })

function confirmSelect() {
    if (selectedVersionId.value) {
        // emit('add', selectedVersionId.value)
        dialogVisible.value = false
        emit('close')
    }
}

const showStep2 = ref(false)
const selectedComponents = ref(['minecraft'])
const componentVersions = ref<{[k:string]: string}>({})
const showComponentVersionDialog = ref(false)
const componentVersionList = ref<string[]>([])
const componentVersionSelected = ref<string | null>(null)
const componentVersionTitle = ref('')
const componentVersionLoading = ref(false)
let currentComponent: string | null = null
const componentLoading = ref<{[k:string]: boolean}>({ forge: false, fabric: false, neoforge: false })
const componentVersionListMap = ref<{[k:string]: string[]}>({ forge: [], fabric: [], neoforge: [] })

async function fetchComponentVersions(component: string) {
  componentLoading.value[component] = true
  // TODO: 替换为实际异步获取版本的函数
  return new Promise<string[]>(resolve => {
    setTimeout(() => {
      let list: string[] = []
      if (component === 'forge') {
        list = ['Forge-1.20.1', 'Forge-1.19.4', 'Forge-1.18.2']
      } else if (component === 'fabric') {
        list = ['Fabric-0.15.7', 'Fabric-0.14.21']
      } else if (component === 'neoforge') {
        list = ['NeoForge-20.2.45', 'NeoForge-19.3.12']
      }
      componentVersionListMap.value[component] = list
      componentLoading.value[component] = false
      resolve(list)
    }, 1000)
  })
}

watch(showStep2, (val) => {
  if (val) {
    ['forge', 'fabric', 'neoforge'].forEach((component) => {
      if (!componentVersionListMap.value[component] || componentVersionListMap.value[component].length === 0) {
        fetchComponentVersions(component)
      }
    })
  }
})

async function openComponentVersion(component: string) {
  currentComponent = component
  componentVersionTitle.value = `选择 ${component.charAt(0).toUpperCase() + component.slice(1)} 版本`
  componentVersionSelected.value = componentVersions.value[component] || null
  componentVersionLoading.value = componentLoading.value[component]
  if (!componentVersionListMap.value[component] || componentVersionListMap.value[component].length === 0) {
    await fetchComponentVersions(component)
  }
  componentVersionList.value = componentVersionListMap.value[component]
  componentVersionLoading.value = false
  showComponentVersionDialog.value = true
}

function selectComponentVersion() {
  if (currentComponent && componentVersionSelected.value) {
    componentVersions.value[currentComponent] = componentVersionSelected.value
    showComponentVersionDialog.value = false
  }
}

function confirmComponents() {
  // 关闭所有对话框
  showStep2.value = false
  dialogVisible.value = false
  // emit 选择结果
  emit('confirm', {
    versionId: selectedVersionId.value,
    components: selectedComponents.value,
    componentVersions: { ...componentVersions.value }
  })
}

const modComponents = [
  { key: 'forge', label: 'Forge', icon: 'forge' },
  { key: 'fabric', label: 'Fabric', icon: 'fabric' },
  { key: 'neoforge', label: 'NeoForge', icon: 'neoforge' }
]
</script>

<style scoped>
.card-clickable {
  cursor: pointer;
  transition: box-shadow 0.2s;
}
.card-clickable:hover {
  box-shadow: 0 4px 20px rgba(0,0,0,0.15);
}
</style>
