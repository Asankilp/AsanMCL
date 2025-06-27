import { defineStore } from 'pinia'

export interface DownloadItem {
  id: string
  filename: string
  progress: number // 0~100
}

interface DownloadDialogState {
  items: DownloadItem[]
  visible: boolean
}

export const useDownloadDialogStore = defineStore('downloadDialog', {
  state: (): DownloadDialogState => ({
    items: [],
    visible: false,
  }),
  actions: {
    addOrUpdateItem(item: DownloadItem) {
      const idx = this.items.findIndex(i => i.id === item.id)
      if (idx !== -1) {
        this.items[idx] = item
      } else {
        this.items.push(item)
      }
      // 自动移除已完成的任务
      this.items = this.items.filter(i => i.progress < 100)
      this.visible = this.items.length > 0
    },
    removeItem(id: string) {
      this.items = this.items.filter(i => i.id !== id)
      this.visible = this.items.length > 0
    },
    clear() {
      this.items = []
      this.visible = false
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
