import { defineStore } from 'pinia'
import { defaultLauncherConfig, LauncherConfig } from '../types/config/launcher'
import { invoke } from '@tauri-apps/api/core'
import { ref } from 'vue'
import { AccountConfig, defaultAccountConfig } from '../types/config/account'

export const useLauncherConfigStore = defineStore('launcherconfig', () => {
  const config = ref<LauncherConfig>(defaultLauncherConfig())

  // 加载配置
  const loadConfig = async () => {
    try {
      config.value = await invoke<LauncherConfig>('get_launcher_config_command')
    } catch (error) {
      console.error('Failed to load launcher config:', error)
    }
  }

  // 保存配置
  const saveConfig = async () => {
    try {
      await invoke('save_launcher_config_command', { config: config.value })
      // config.value = newConfig
    } catch (error) {
      console.error('Failed to save launcher config:', error)
    }
  }

  return {
    config,
    loadConfig,
    saveConfig
  }
})

export const useAccountConfigStore = defineStore('accountconfig', () => {
  const config = ref<AccountConfig>(defaultAccountConfig())

  const loadConfig = async () => {
    try {
      config.value = await invoke<AccountConfig>('get_account_config_command')
    } catch (error) {
      console.error('Failed to load account config:', error)
    }
  }

  const saveConfig = async () => {
    try {
      await invoke('save_account_config_command', { config: config.value })
    } catch (error) {
      console.error('Failed to save account config:', error)
    }
  }
  return {
    config,
    loadConfig,
    saveConfig
  }
})