import { ref, watch } from 'vue'
import { useTheme } from 'vuetify'
import { ColorTheme, type LauncherConfig } from '../types/config/launcher'
import { invoke } from '@tauri-apps/api/core'
import { usePreferredDark } from '@vueuse/core'

export const useAppTheme = () => {
  const theme = useTheme()
  const osTheme = usePreferredDark()
  const colorTheme = ref<ColorTheme>()

  // 加载主题设置
  const loadTheme = async () => {
    try {
      const config = await invoke<LauncherConfig>('get_launcher_config_command')
      colorTheme.value = config.colorTheme
      applyTheme(config.colorTheme)
    } catch (error) {
      console.error('Failed to load theme setting:', error)
    }
  }

  // 保存主题设置
  const saveTheme = async (newTheme: ColorTheme) => {
    try {
      const config = await invoke<LauncherConfig>('get_launcher_config_command')
      config.colorTheme = newTheme
      await invoke('save_launcher_config_command', { config })
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
    if ((await invoke<LauncherConfig>('get_launcher_config_command')).colorTheme === ColorTheme.FollowSystem) {
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