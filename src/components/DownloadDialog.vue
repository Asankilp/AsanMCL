<template>
  <v-dialog v-model="visible" max-width="420" persistent no-click-animation :scrim="true">
    <v-card>
      <v-card-title class="text-h6">下载任务</v-card-title>
      <v-card-text>
        <div v-if="items.length === 0" class="empty">暂无下载任务</div>
        <div v-else>
          <div v-for="item in items" :key="item.id" class="download-item">
            <div class="filename">{{ item.filename }}</div>
            <v-progress-linear :model-value="item.progress" height="8" rounded :color="item.progress === 100 ? 'success' : 'primary'">
              <template #default>
                <span v-if="item.progress === 100" class="progress-label">完成</span>
                <span v-else class="progress-label">{{ Math.floor(item.progress) }}%</span>
              </template>
            </v-progress-linear>
          </div>
        </div>
      </v-card-text>
      <v-card-actions v-if="items.length > 0">
        <v-spacer />
        <v-btn size="small" color="error" variant="text" @click="cancelAll">取消</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useDownloadDialogStore } from '../store/downloadDialog'
import { invoke } from '@tauri-apps/api/core'
import { watch } from 'vue'
const store = useDownloadDialogStore()
const { items, visible } = storeToRefs(store)

function cancelAll() {
  for (const item of items.value) {
    invoke('cancel_download', { id: item.id })
    // store.clear()
  }
}

// 自动关闭对话框：当 items 变为空时关闭
watch(items, (val) => {
  if (val.length === 0) {
    store.visible = false
  }
})
</script>

<style scoped>
.download-item {
  margin-bottom: 22px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  position: relative;
}
.filename {
  font-size: 15px;
  margin-bottom: 6px;
  word-break: break-all;
}
.empty {
  text-align: center;
  color: #888;
  padding: 32px 0;
}
.progress-label {
  font-size: 12px;
  color: #555;
  margin-left: 8px;
}
</style>
