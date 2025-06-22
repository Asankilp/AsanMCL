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
                <v-btn color="primary" :disabled="!selectedVersionId" @click="confirmSelect">确定</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup lang="ts">
import { ref, watch, computed, PropType } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { VersionManifest, VersionType } from '../types/version'
import { LauncherConfig } from '../types/config/launcher'
import { getVersionIcon } from '../utils/icon'
import { getMajorUpdateThemeById } from '../utils/version'

const props = defineProps({
    modelValue: Boolean,
    launcherConfig: {
        type: Object as PropType<LauncherConfig>,
        required: true
    }
})
const emit = defineEmits(['update:modelValue', 'close'])

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


</script>
