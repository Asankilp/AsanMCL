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
                                <span v-if="versionThemes[version.id]"
                                    style="font-size: 12px; color: #888; margin-left: 8px;">{{
                                    versionThemes[version.id] }}</span>
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

    <!-- 第二步对话框：选择模组加载器 -->
    <v-dialog v-model="showStep2" max-width="400">
        <v-card>
            <v-card-title>添加版本</v-card-title>
            <v-card-text>
                <v-row dense class="flex-column">
                    <v-text-field v-model="versionName" label="版本名称" :rules="versionNameRules" ref="versionNameField" />
                    <!-- Minecraft 卡片 -->
                    <v-col cols="12">
                        <v-card>
                            <v-card-title class="d-flex align-center">
                                <v-avatar rounded="lg" size="40" class="mr-2">
                                    <img :src="getProfileIconUrl('grassblock')" width="40" height="40" alt="icon" />
                                </v-avatar>
                                <v-checkbox v-model="selectedModLoaders" :value="'minecraft'" :disabled="true"
                                    :ripple="false" hide-details class="mr-2" />
                                Minecraft
                            </v-card-title>
                            <v-card-subtitle>{{ selectedVersionId }}</v-card-subtitle>
                        </v-card>
                    </v-col>
                    <v-col cols="12" v-for="mod in modLoaders" :key="mod.key">
                        <v-card :outlined="true" :elevation="selectedModLoaders.includes(mod.key) ? 8 : 2"
                            :class="{ 'card-clickable': selectedModLoaders.includes(mod.key), 'disabled-card': modLoaderVersionErrorMap[mod.key] }"
                            @click="selectedModLoaders.includes(mod.key) && !modLoaderVersionErrorMap[mod.key] && openModLoaderVersion(mod.key)">
                            <v-card-title class="d-flex align-center">
                                <v-avatar rounded="lg" size="40" class="mr-2">
                                    <img :src="getProfileIconUrl(mod.icon as ProfileIcon)" width="40" height="40"
                                        alt="icon" />
                                </v-avatar>
                                <v-checkbox v-model="selectedModLoaders" :value="mod.key" hide-details class="mr-2"
                                    @click.stop
                                    :disabled="modLoaderLoading[mod.key] || !!modLoaderVersionErrorMap[mod.key] || (!selectedModLoaders.includes(mod.key) && selectedModLoaders.some(k => modLoaders.map(m => m.key).includes(k)))" />
                                <span :style="!selectedModLoaders.includes(mod.key) ? 'color:#aaa;' : ''">{{ mod.label }}</span>
                                <v-spacer />
                                <v-progress-circular v-if="modLoaderLoading[mod.key]" indeterminate color="primary"
                                    size="20" class="ml-2" />
                            </v-card-title>
                            <v-card-subtitle v-if="modLoaderVersions[mod.key]">{{ modLoaderVersions[mod.key] }}</v-card-subtitle>
                            <v-alert v-if="modLoaderVersionErrorMap[mod.key]" type="error" dense class="mt-1 mb-0">{{ modLoaderVersionErrorMap[mod.key] }}</v-alert>
                        </v-card>
                    </v-col>
                </v-row>
            </v-card-text>
            <v-card-actions>
                <v-spacer />
                <v-btn text @click="showStep2 = false">返回</v-btn>
                <v-btn color="primary" @click="confirmModLoaders" :disabled="!versionNameRules.every(rule => rule(versionName) === true)">确定</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>

    <!-- 模组加载器版本选择对话框 -->
    <v-dialog v-model="showModLoaderVersionDialog" max-width="600">
        <v-card>
            <v-card-title>{{ modLoaderVersionTitle }}</v-card-title>
            <v-card-text style="max-height: 400px; overflow-y: auto;">
                <v-row justify="center" v-if="modLoaderVersionLoading">
                    <v-progress-circular indeterminate color="primary" size="40" />
                </v-row>
                <v-radio-group v-else v-model="modLoaderVersionSelected">
                    <v-list>
                        <v-list-item v-for="item in modLoaderVersionList" :key="item"
                            :active="modLoaderVersionSelected === item" @click="modLoaderVersionSelected = item"
                            style="cursor:pointer;">
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
                <v-btn text @click="showModLoaderVersionDialog = false">取消</v-btn>
                <v-btn color="primary" :disabled="!modLoaderVersionSelected" @click="selectModLoaderVersion">确定</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed, PropType, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { LocalVersionInfo, VersionManifest, VersionType } from '../types/version'
import { LauncherConfig } from '../types/config/launcher'
import { getProfileIconUrl, getVersionIcon } from '../utils/icon'
import { getMajorUpdateThemeById } from '../utils/version'
import { ProfileIcon } from '../types/profile'
import { useSnackbar } from '../composables/useSnackbar'
import { useLauncherConfigStore } from '../composables/useConfig'

const launcherConfigStore = useLauncherConfigStore()
const { showError } = useSnackbar()
const props = defineProps({
    modelValue: Boolean,
    launcherConfig: {
        type: Object as PropType<LauncherConfig>,
        required: true
    }
})
const emit = defineEmits<{
    (e: 'update:modelValue', value: boolean): void
    (e: 'confirm', data: Object): void
    (e: 'close'): void
}>()

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
const selectedModLoaders = ref(['minecraft'])
const modLoaderVersions = ref<{ [k: string]: string }>({})
const showModLoaderVersionDialog = ref(false)
const modLoaderVersionList = ref<string[]>([])
const modLoaderVersionSelected = ref<string | null>(null)
const modLoaderVersionTitle = ref('')
const modLoaderVersionLoading = ref(false)
let currentModLoader: string | null = null
const modLoaderLoading = ref<{ [k: string]: boolean }>({ forge: false, fabric: false, neoforge: false })
const modLoaderVersionListMap = ref<{ [k: string]: string[] }>({ forge: [], fabric: [], neoforge: [] })
const modLoaderVersionErrorMap = ref<{ [k: string]: string }>({})
const existedLocalVersions = ref<LocalVersionInfo[]>([])
const versionName = ref<string>('')
const versionNameField = ref()

const versionNameRules = [
  (v: string) => !!v || '请输入版本名称',
  (v: string) => !existedLocalVersions.value.some(ver => ver.name === v) || '该版本已存在，请换个名称'
]

async function fetchModLoaderVersions(modLoader: string, versionId: string | null) {
    modLoaderLoading.value[modLoader] = true
    switch (modLoader) {
        case 'forge':
            // TODO: Forge
            break
        case 'fabric':
            try {
                modLoaderVersionListMap.value[modLoader] = await invoke('get_fabric_loader_versions_by_game_version', { gameVersion: versionId, downloadSource: launcherConfigStore.config.downloadSource })
                modLoaderVersionErrorMap.value[modLoader] = ''
            } catch (e: string | any) {
                modLoaderVersionErrorMap.value[modLoader] = e
                modLoaderVersionListMap.value[modLoader] = []
            }
            break
        case 'quilt':
            // TODO: Quilt
            break
    }
    modLoaderLoading.value[modLoader] = false
}


watch(showStep2, (val) => {
    versionName.value = selectedVersionId.value ?? ''
    if (val) {
        // 每次都重新拉取所有modLoader的版本
        ['forge', 'fabric', 'neoforge'].forEach((modLoader) => {
            fetchModLoaderVersions(modLoader, selectedVersionId.value)
        })
        // 自动触发版本名称校验
        nextTick(() => {
            versionNameField.value?.validate?.()
        })
    } else {
        selectedModLoaders.value = ['minecraft']
        modLoaderVersions.value = {}
    }
})

async function openModLoaderVersion(modLoader: string) {
    currentModLoader = modLoader
    modLoaderVersionTitle.value = `选择 ${modLoader.charAt(0).toUpperCase() + modLoader.slice(1)} 版本`
    modLoaderVersionSelected.value = modLoaderVersions.value[modLoader] || null
    modLoaderVersionLoading.value = modLoaderLoading.value[modLoader]
    if (!modLoaderVersionListMap.value[modLoader] || modLoaderVersionListMap.value[modLoader].length === 0) {
        await fetchModLoaderVersions(modLoader, selectedVersionId.value)
    }
    modLoaderVersionList.value = modLoaderVersionListMap.value[modLoader]
    modLoaderVersionLoading.value = false
    showModLoaderVersionDialog.value = true
}

function selectModLoaderVersion() {
    if (currentModLoader && modLoaderVersionSelected.value) {
        modLoaderVersions.value[currentModLoader] = modLoaderVersionSelected.value
        showModLoaderVersionDialog.value = false
    }
}

function confirmModLoaders() {
    // 关闭所有对话框
    console.log(selectedModLoaders.value)
    console.log(modLoaderVersions.value)
    if (selectedModLoaders.value.length != Object.keys(modLoaderVersions.value).length + 1) {
        showError('请选择所有已勾选的模组加载器的版本')
        return
    }
    showStep2.value = false
    dialogVisible.value = false
    // emit 选择结果
    emit('confirm', {
        versionId: selectedVersionId.value,
        modLoaders: selectedModLoaders.value,
        modLoaderVersions: { ...modLoaderVersions.value }
    })
    selectedVersionId.value = null
    selectedModLoaders.value = ['minecraft']
    modLoaderVersions.value = {}
}

const modLoaders = [
    { key: 'forge', label: 'Forge', icon: 'forge' },
    { key: 'fabric', label: 'Fabric', icon: 'fabric' },
    { key: 'neoforge', label: 'NeoForge', icon: 'neoforge' }
]

watch(selectedModLoaders, (val) => {
    // 只允许modLoaderList中的key最多选中一个
    const modKeys = modLoaders.map(m => m.key)
    const selected = val.filter((k: string) => modKeys.includes(k))
    if (selected.length > 1) {
        // 只保留最后一个
        selectedModLoaders.value = val.filter((k: string) => !modKeys.includes(k)).concat(selected.slice(-1))
    }
})

onMounted(async () => { 
    existedLocalVersions.value = await invoke<LocalVersionInfo[]>('get_local_versions_command', { gamePath: launcherConfigStore.config.gamePath[launcherConfigStore.config.lastGamePath] })
})
</script>


<style scoped>
.card-clickable {
    cursor: pointer;
    transition: box-shadow 0.2s;
}

.card-clickable:hover {
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}
.disabled-card {
    pointer-events: none;
    opacity: 0.6;
}
</style>
