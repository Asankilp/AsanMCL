import { defineStore } from 'pinia'
import { DownloadProgress } from '../types/event'

interface DownloadDialogState {
  items: DownloadProgress[]
}

export const useDownloadDialogStore = defineStore('downloadDialog', {
  state: (): DownloadDialogState => ({
    items: [],
  }),
  getters: {
    visible(state): boolean {
      return state.items.length > 0
    }
  },
  actions: {
    addOrUpdateItem(item: DownloadProgress) {
      const idx = this.items.findIndex(i => i.id === item.id)
      if (idx !== -1) {
        this.items[idx] = item
      } else {
        this.items.push(item)
      }
    },
    removeItem(id: string) {
      this.items = this.items.filter(i => i.id !== id)
    },
    clear() {
      this.items = []
    },
    // 供外部监听对话框开关
    onVisibleChange(cb: (visible: boolean) => void) {
      let old = this.visible
      this.$subscribe(() => {
        if (this.visible !== old) {
          old = this.visible
          cb(this.visible)
        }
      })
    }
  }
})
