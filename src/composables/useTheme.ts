import { ref, watch } from 'vue'
import { useTheme } from 'vuetify'
import { ColorTheme, type LauncherConfig } from '../types/config/launcher'
import { invoke } from '@tauri-apps/api/core'
import { usePreferredDark } from '@vueuse/core'
import { useLauncherConfigStore } from './useConfig'

export const useAppTheme = () => {
  const theme = useTheme()
  const osTheme = usePreferredDark()
  const colorTheme = ref<ColorTheme>(ColorTheme.FollowSystem)
  const launcherConfigStore = useLauncherConfigStore()

  // 加载主题设置
  const loadTheme = async () => {
    try {
      colorTheme.value = launcherConfigStore.config.colorTheme
      applyTheme(colorTheme.value)
    } catch (error) {
      console.error('Failed to load theme setting:', error)
    }
  }

  // 保存主题设置
  const saveTheme = async (newTheme: ColorTheme) => {
    try {
      launcherConfigStore.config.colorTheme = newTheme
      launcherConfigStore.saveConfig()
    } catch (error) {
      console.error('Failed to save theme setting:', error)
    }
  }

  // 应用主题
  const applyTheme = (themeValue: ColorTheme) => {
    if (themeValue === ColorTheme.FollowSystem) {
      theme.global.name.value = osTheme.value ? 'dark' : 'light'
    } else {
      theme.global.name.value = themeValue
    }
  }

  // 监听系统主题变化
  watch(osTheme, async () => {
    console.log(colorTheme.value)
    if (launcherConfigStore.config.colorTheme === ColorTheme.FollowSystem) {
      console.log('System theme changed:', osTheme.value)
      applyTheme(ColorTheme.FollowSystem)
    }
  })

  // 监听主题设置变化
  watch(colorTheme, (newValue) => {
    console.log('Theme setting changed:', newValue)
    if (newValue !== undefined) {
      applyTheme(newValue)
      saveTheme(newValue)
    }
  })

  return {
    colorTheme,
    loadTheme,
  }
}