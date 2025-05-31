<template>
  <v-window-item value="microsoft">
    <v-alert
      type="info"
      variant="tonal"
      :text="'使用 Microsoft 账户登录可以：\n- 进入正版服务器\n- 使用皮肤功能\n- 使用跨平台同步'"
      class="mb-4"
    ></v-alert>
    <v-btn
      color="primary"
      block
      prepend-icon="mdi-microsoft"
      variant="elevated"
      @click="handleMicrosoftLogin"
      :loading="isLoading"
    >
      使用 Microsoft 账户登录
    </v-btn>
  </v-window-item>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useSnackbar } from '../composables/useSnackbar'
import { LoginEvent } from '../types/event'
import { Channel, invoke } from '@tauri-apps/api/core'
import { MinecraftProfile } from '../types/mojang'

const { showSuccess, showError } = useSnackbar()

const emit = defineEmits<{
  (e: 'login-success', data: any): void
  (e: 'show-user-code', data: { authUrl: string, userCode: string, close?: boolean }): void
  (e: 'show-player-info', data: any): void
}>()

const isLoading = ref(false)

// 暴露 isLoading 以便父组件可以修改它
defineExpose({
  setLoading: (value: boolean) => {
    isLoading.value = value
  }
})

const handleMicrosoftLogin = async () => {
  try {
    // 显示加载状态
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
          const avatarUrl: string = await invoke('get_player_avatar_url', { uuid: profile.id })
          const skinPreviewUrl: string = await invoke('get_player_skin_preview_url', { uuid: profile.id })

          // 更新玩家信息
          emit('show-player-info', {
            username: profile.name,
            uuid: profile.id,
            avatarUrl,
            skinPreviewUrl,
            skins: profile.skins,
            capes: profile.capes
          })
            showSuccess('登录成功！')
          // 成功后关闭代码对话框
          emit('show-user-code', {
            authUrl: '',
            userCode: '',
            close: true
          })
          emit('login-success', {
            type: 'microsoft',
            data: {
              username: profile.name,
              accessToken: result.accessToken,
              tokenType: result.tokenType,
              expiresIn: result.expiresIn,
              uuid: profile.id,
              avatarUrl,
              skinPreviewUrl,
              skins: profile.skins,
              capes: profile.capes
            }
          })
        } catch (err) {
          console.error('获取玩家信息失败:', err)
          showError('获取玩家信息失败，但登录已成功')
          // 成功后关闭代码对话框
          emit('show-user-code', {
            authUrl: '',
            userCode: '',
            close: true
          })
          emit('login-success', {
            type: 'microsoft',
            data: {
              username: result.username,
              accessToken: result.accessToken,
              tokenType: result.tokenType,
              expiresIn: result.expiresIn
            }
          })
        }
      }
    }

    // 启动登录流程
    await invoke('get_device_code', { onEvent })

  } catch (error) {
    console.error('登录失败:', error)
    showError('登录失败：' + (error instanceof Error ? error.message : '未知错误'))
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
</script>
