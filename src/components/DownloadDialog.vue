<template>
  <v-dialog v-model="visible" max-width="420" persistent no-click-animation :scrim="true">
    <v-card class="download-dialog-card">
      <v-card-title class="text-h6">下载任务</v-card-title>
      <v-card-text class="download-dialog-content">
        <div v-if="items.length === 0" class="empty">暂无下载任务</div>
        <div v-else>
          <div v-for="item in items" :key="item.id" class="download-item">
            <div class="filename-row">
              <div class="filename">{{ item.path }}</div>
              <div class="progress-info">
                <span v-if="item.progress === 100" class="progress-label">完成</span>
                <span v-else-if="item.progress > -1" class="progress-label">{{ Math.floor(item.progress) }}%</span>
                <span v-if="item.progress < 100 && item.speed !== undefined" class="speed-label">{{ formatSpeed(item.speed) }}</span>
              </div>
            </div>
            <v-progress-linear
              :model-value="item.progress >= 0 ? item.progress : undefined"
              :indeterminate="item.progress === -1"
              height="8"
              rounded
              :color="item.progress === 100 ? 'success' : 'primary'"
            />
          </div>
        </div>
      </v-card-text>
      <v-card-actions v-if="items.length > 0" class="download-dialog-actions">
        <v-spacer />
        <v-btn size="small" color="error" variant="text" @click="cancelAll">取消</v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { storeToRefs } from 'pinia'
import { useDownloadDialogStore } from '../composables/useDownloadDialog'
import { invoke } from '@tauri-apps/api/core'
const store = useDownloadDialogStore()
const { items, visible } = storeToRefs(store)

function cancelAll() {
  for (const item of items.value) {
    invoke('cancel_download', { id: item.id })
  }
}

function formatSpeed(speed: number) {
  if (speed >= 1024) {
    return (speed / 1024).toFixed(2) + ' MB/s'
  } else if (speed > 0) {
    return speed.toFixed(1) + ' KB/s'
  } else {
    return '--'
  }
}

</script>

<style scoped>
.download-dialog-card {
  display: flex;
  flex-direction: column;
  min-height: 320px;
  max-height: 70vh;
}
.download-dialog-content {
  flex: 1 1 auto;
  overflow-y: auto;
}
.download-dialog-actions {
  flex-shrink: 0;
}
.download-item {
  margin-bottom: 22px;
  display: flex;
  flex-direction: column;
  gap: 4px;
  position: relative;
}
.filename-row {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 6px;
}
.filename {
  font-size: 15px;
  word-break: break-all;
  flex: 1 1 auto;
}
.progress-info {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12px;
  min-width: 90px;
  justify-content: flex-end;
}
.progress-label {
  font-size: 12px;
  color: #555;
  min-width: 40px;
  text-align: right;
  flex-shrink: 0;
}
.speed-label {
  font-size: 12px;
  color: #888;
  min-width: 60px;
  text-align: right;
  flex-shrink: 0;
}
.empty {
  text-align: center;
  color: #888;
  padding: 32px 0;
}
</style>
