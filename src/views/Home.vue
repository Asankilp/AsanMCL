<template>
  <div class="home">
    <div class="content">
      <!-- 主要内容区域 -->
      <h2>欢迎使用 Minecraft 启动器</h2>
      <v-btn @click="testDownload">下载测试！！！！！！！！！！！</v-btn>
    </div>
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';



const testDownload = async () => {
  await invoke<void>('download_files', { files: { "https://d.taptap.cn/pc/latest/rep-rep_typxbuxvnpi?sid=250627YjqCrFCCbhZF": "G:\\test.exe", "https://d.taptap.cn/latest/rep-rep_typxbuxvnpi?sid=250627YjqCrFCCbhZF": "G:\\test.apk" } })
}
</script>

<style scoped>
.home {
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.content {
  text-align: center;
}

h2 {
  margin-bottom: 2rem;
  color: var(--el-text-color-primary);
}
</style>
