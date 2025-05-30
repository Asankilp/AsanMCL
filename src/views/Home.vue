<template>
  <div class="home">
    <div class="content">
      <!-- 主要内容区域 -->
      <h2>欢迎使用 Minecraft 启动器</h2>
      <el-button type="primary" size="large">启动游戏</el-button>
    </div>
  </div>
</template>

<script setup lang="ts">
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
