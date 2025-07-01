<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="400px" persistent>
    <v-card>
      <v-card-title class="text-h6">{{ $t('account.login_with_microsoft') }}</v-card-title>
      <v-card-text class="text-center">
        <p class="mb-4">{{ $t('account.user_code_hint') }}</p>
        <v-btn
          block
          color="primary"
          class="mb-4"
          :href="authUrl"
          target="_blank"
          rel="noopener noreferrer"
          @click="showSuccess($t('account.open_authorization_page_hint'))"
        >
          {{ $t('account.open_authorization_page') }}
        </v-btn>
        <v-card variant="outlined" class="mb-4 pa-4 cursor-pointer" @click="copyToClipboard(userCode)">
          <p class="text-h5 font-weight-bold">{{ userCode }}</p>
          <v-icon icon="mdi-content-copy" size="small" class="ms-2"></v-icon>
        </v-card>
        <p class="text-caption">{{ $t('account.copy_code_hint') }}</p>
        <p class="text-caption mt-2">{{ $t('account.waiting_for_authorization') }}</p>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="error" variant="text" @click="handleCancel">
          {{ $t('general.cancel') }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useI18n } from 'vue-i18n';
import { useSnackbar } from '../composables/useSnackbar'
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const { showSuccess, showError } = useSnackbar()
const { t } = useI18n()
defineProps<{
  modelValue: boolean
  authUrl: string
  userCode: string
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'cancel'): void
  (e: 'loading', value: boolean): void
}>()

const copyToClipboard = async (text: string) => {
  try {
    await writeText(text)
    showSuccess(t('general.copy_to_clipboard_success'))
  } catch (err) {
    console.error('Failed to copy text:', err)
    showError(t('general.copy_to_clipboard_failed'))
  }
}

const handleCancel = () => {
  emit('cancel')
  emit('update:modelValue', false)
  emit('loading', false) // 关闭加载状态
}
</script>
