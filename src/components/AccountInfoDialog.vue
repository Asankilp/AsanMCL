<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="600">
    <v-card>
      <v-card-title class="text-h6">账户信息</v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="4" class="d-flex flex-column align-center">
<!--               <v-avatar size="96" class="mb-2">
                <v-img :src="accountInfo.avatarUrl" alt="玩家头像"></v-img>
              </v-avatar> -->
              <SkinView3d
                :skin-url="skinUrl"
                :width="256"
                :height="256"
                :global-light="3"
              ></SkinView3d>
            </v-col>
            <v-col cols="8">
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
import { ref, watch } from 'vue'
import { SkinView3d } from 'vue-skinview3d'
import { invoke } from '@tauri-apps/api/core'
import { AccountType } from '../types/config/account'

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

const resetSkinPreview = () => {
  skinUrl.value = ''
}

const fetchSkinPreview = async () => {
  resetSkinPreview()
  if (!props.accountInfo.uuid) return
  try {
    const url = await invoke<string>('get_player_skin_preview_url', { uuid: props.accountInfo.uuid })
    skinUrl.value = url ?? ''
  } catch (error) {
    console.error('Failed to load skin preview', error)
  }
}

watch(
  () => props.modelValue,
  (visible) => {
    if (!visible) {
      resetSkinPreview()
      return
    }
    fetchSkinPreview()
  }
)

watch(
  () => props.accountInfo.uuid,
  (uuid, prevUuid) => {
    if (!props.modelValue || uuid === prevUuid) return
    fetchSkinPreview()
  }
)

</script>
