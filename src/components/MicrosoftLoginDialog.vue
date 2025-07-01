<template>
  <v-alert type="info" variant="tonal" :text="$t('account.login_with_microsoft_hint')" class="mb-4"></v-alert>
  <v-btn color="primary" block prepend-icon="mdi-microsoft" variant="elevated" @click="handleMicrosoftLogin"
    :loading="isLoading">
    {{ t('account.login_with_microsoft') }}
  </v-btn>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useSnackbar } from '../composables/useSnackbar'
import { LoginEvent } from '../types/event'
import { Channel, invoke } from '@tauri-apps/api/core'
import { MinecraftProfile } from '../types/mojang'
import { AccountInfo, AccountType } from '../types/config/account'
import { useI18n } from 'vue-i18n'

const { showSuccess, showError } = useSnackbar()
const { t } = useI18n()
const emit = defineEmits<{
  (e: 'login-success', data: AccountInfo): void
  (e: 'show-user-code', data: { authUrl: string, userCode: string, close?: boolean }): void
  (e: 'show-account-info', data: any): void
}>()

const isLoading = ref(false)

const handleMicrosoftLogin = async () => {
  try {
    // 显示加载状态
    if (await invoke<boolean>('check_microsoft_login_availability') === false) {
      showError(t('account.microsoft_login_unavailable'))
      return
    }
    isLoading.value = true

    // 创建事件通道
    const onEvent = new Channel<LoginEvent>()

    // 监听事件
    onEvent.onmessage = async (message: LoginEvent) => {
      if (message.event === 'started') {
        emit('show-user-code', {
          authUrl: 'https://microsoft.com/link',
          userCode: message.data.code
        })
      } else if (message.event === 'finished') {
        const result = message.data.response
        try {
          const profile: MinecraftProfile = await invoke("get_minecraft_profile", {
              accessToken: result.accessToken
          })
          try {
            const avatarUrl: string = await invoke('get_player_avatar_url', { uuid: profile.id })
            const skinPreviewUrl: string = await invoke('get_player_skin_preview_url', { uuid: profile.id })

            // 更新玩家信息
            emit('show-account-info', {
              username: profile.name,
              uuid: profile.id,
              type: AccountType.Microsoft,
              avatarUrl,
              skinPreviewUrl,
              skins: profile.skins,
              capes: profile.capes
            })

            showSuccess(t('account.login_success'))
            // 成功后关闭代码对话框
            emit('show-user-code', {
              authUrl: '',
              userCode: '',
              close: true
            })
            emit('login-success', {
              accountType: AccountType.Microsoft,
              name: profile.name,
              uuid: profile.id,
              accessToken: result.accessToken,
              userId: result.userId,
              expiresIn: message.data.response.expiresIn,
            })
          } catch (err) {
            console.error('获取玩家信息失败:', err)
            showError(t('account.get_player_info_failed_but_login_success'))
            // 成功后关闭代码对话框
            emit('show-user-code', {
              authUrl: '',
              userCode: '',
              close: true
            })
            emit('login-success', {
              accountType: AccountType.Microsoft,
              name: profile.name,
              uuid: profile.id,
              accessToken: result.accessToken,
              userId: result.userId,
              expiresIn: message.data.response.expiresIn,
            })
          }
        } catch (error) {
          console.error('获取玩家信息失败:', error)
          showError(t('account.get_player_info_failed') + ':' + (error instanceof Error ? error.message : '未知错误'))
        }
      }
    }

    // 启动登录流程
    await invoke('get_device_code', { onEvent })

  } catch (error) {
    console.error('登录失败:', error)
    showError(t('account.login_failed') + ':' + (error instanceof Error ? error.message : '未知错误'))
    // 失败后关闭代码对话框
    emit('show-user-code', {
      authUrl: '',
      userCode: '',
      close: true
    })
  } finally {
    isLoading.value = false
  }
}

// 暴露方法给父组件
defineExpose({
  setLoading: (value: boolean) => {
    isLoading.value = value
  },
  handleMicrosoftLogin
})
</script>
