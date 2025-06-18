<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="400px" persistent>
    <v-card>
      <v-card-title class="text-h6">Microsoft 账户登录</v-card-title>
      <v-card-text class="text-center">
        <p class="mb-4">请访问以下网址并输入代码：</p>
        <v-btn
          block
          color="primary"
          class="mb-4"
          :href="authUrl"
          target="_blank"
          rel="noopener noreferrer"
          @click="showSuccess('已在浏览器中打开授权页面')"
        >
          打开授权页面
        </v-btn>
        <v-card variant="outlined" class="mb-4 pa-4 cursor-pointer" @click="copyToClipboard(userCode)">
          <p class="text-h5 font-weight-bold">{{ userCode }}</p>
          <v-icon icon="mdi-content-copy" size="small" class="ms-2"></v-icon>
        </v-card>
        <p class="text-caption">点击上方的代码可复制到剪贴板</p>
        <p class="text-caption mt-2">正在等待授权完成...</p>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="error" variant="text" @click="handleCancel">
          取消
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useSnackbar } from '../composables/useSnackbar'
import { writeText } from '@tauri-apps/plugin-clipboard-manager';

const { showSuccess, showError } = useSnackbar()

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
    showSuccess('已复制到剪贴板')
  } catch (err) {
    console.error('Failed to copy text:', err)
    showError('复制失败')
  }
}

const handleCancel = () => {
  emit('cancel')
  emit('update:modelValue', false)
  emit('loading', false) // 关闭加载状态
}
</script>
