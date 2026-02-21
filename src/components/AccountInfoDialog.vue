<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="720">
    <v-card>
      <v-card-title class="text-h6">账户信息</v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="5" md="4" class="d-flex justify-center align-start mb-4 mb-md-0">
              <SkinView3d
                class="account-skin-preview"
                :skin-url="skinUrl"
                :cape-url="capeUrl"
                :width="256"
                :height="360"
                :global-light="3"
                :skin-options="skinViewerOptions"
              ></SkinView3d>
            </v-col>
            <v-col cols="7" md="8">
              <v-list>
                <v-list-item>
                  <v-list-item-title>用户名</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.username }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <v-list-item-title>UUID</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.uuid }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <v-list-item-title>账户类型</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.type }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="accountInfo.skins?.length">
                  <v-list-item-title>皮肤数量</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.skins.length }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="accountInfo.capes?.length">
                  <v-list-item-title>披风数量</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.capes.length }}</v-list-item-subtitle>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="primary"
          variant="text"
          @click="emit('update:modelValue', false)"
        >
          {{ $t('general.confirm') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { computed, ref, watch } from 'vue'
import { SkinView3d } from 'vue-skinview3d'
import { invoke } from '@tauri-apps/api/core'
import { AccountType } from '../types/config/account'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()

type SkinPreviewInfo = {
  skinUrl?: string | null
  skinModel?: string | null
  capeUrl?: string | null
}

const props = defineProps<{
  modelValue: boolean
  accountInfo: {
    username: string
    uuid: string
    type: AccountType
    avatarUrl?: string
    skinPreviewUrl?: string
    skins?: any[]
    capes?: any[]
  }
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()

const skinUrl = ref('')
const capeUrl = ref('')
const skinModel = ref<'classic' | 'slim'>('classic')

const skinViewerOptions = computed(() => {
  const model: 'slim' | 'default' = skinModel.value === 'slim' ? 'slim' : 'default'
  return { model }
})

const resetPreviewUrls = () => {
  skinUrl.value = ''
  capeUrl.value = ''
  skinModel.value = 'classic'
}

const fetchPreviewUrls = async () => {
  resetPreviewUrls()
  if (!props.accountInfo.uuid) return
  try {
    const preview = await invoke<SkinPreviewInfo>('get_player_skin_preview_url', { uuid: props.accountInfo.uuid })
    skinUrl.value = preview?.skinUrl ?? ''
    capeUrl.value = preview?.capeUrl ?? ''
    skinModel.value = preview?.skinModel === 'slim' ? 'slim' : 'classic'
  } catch (error) {
    console.error('Failed to load skin/cape preview', error)
  }
}

watch(
  () => props.modelValue,
  (visible) => {
    if (!visible) {
      resetPreviewUrls()
      return
    }
    fetchPreviewUrls()
  }
)

watch(
  () => props.accountInfo.uuid,
  (uuid, prevUuid) => {
    if (!props.modelValue || uuid === prevUuid) return
    fetchPreviewUrls()
  }
)

</script>

<style scoped>
.account-skin-preview {
  z-index: 1;
}

.cape-link {
  word-break: break-all;
  color: inherit;
  text-decoration: underline;
}

</style>
