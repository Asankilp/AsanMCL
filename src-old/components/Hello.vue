<template>
  <div class="welcome-overlay">
    <v-card class="welcome-card" elevation="0">
      <div class="card-content">
        <!-- 欢迎页头部 -->
        <div class="text-center" v-if="currentStep === 0">
          <v-img src="/tauri.svg" class="mx-auto mb-6" width="120" />
          <h1 class="text-h4 mb-4">欢迎使用 AsanMCL</h1>
          <p class="text-medium-emphasis mb-8">
            喵喵喵
          </p>
        </div>

        <!-- 向导步骤内容 -->
        <v-window v-model="currentStep">
          <!-- 欢迎步骤 -->
          <v-window-item value="0">
            <div class="text-center">
              <v-btn color="primary" size="large" min-width="200" @click="nextStep">
                开始设置
              </v-btn>
            </div>
          </v-window-item>

          <!-- 登录设置步骤 -->
          <v-window-item value="1">
            <div class="text-center">
              <h2 class="text-h5 mb-4">登录设置</h2>
              <p class="text-medium-emphasis mb-6">
                是否现在添加一个 Minecraft 账号？你可以稍后在账号管理中添加。
              </p>
            </div>
          </v-window-item>

          <!-- JRE设置步骤 -->
          <v-window-item value="2">
            <div class="text-center">
              <h2 class="text-h5 mb-4">Java 运行环境</h2>
              <p class="text-medium-emphasis mb-6">
                是否让启动器自动搜索和管理 Java 运行环境？
              </p>
              
            </div>
          </v-window-item>

          <!-- 完成步骤 -->
          <v-window-item value="3">
            <div class="text-center">
              <h2 class="text-h5 mb-4">设置完成</h2>
              <p class="text-medium-emphasis mb-6">
                基本设置已经完成，你可以开始使用启动器了！
              </p>
              <v-btn color="primary" size="large" min-width="200" @click="complete">
                开始使用
              </v-btn>
            </div>
          </v-window-item>
        </v-window>
      </div>

      <!-- 底部导航栏 -->
      <div class="card-actions">
        <v-progress-linear v-if="currentStep > 0" :model-value="(currentStep / 3) * 100" color="primary" height="4"
          rounded></v-progress-linear>

        <div class="d-flex justify-space-between align-center pa-4">
          <v-btn v-if="currentStep > 0" variant="text" @click="prevStep">
            <v-icon start>mdi-arrow-left</v-icon>
            上一步
          </v-btn>
          <v-spacer></v-spacer>
          <div class="d-flex gap-2">
            <v-btn v-if="currentStep > 0 && currentStep < 3" variant="text" @click="skipAll">
              跳过设置
            </v-btn>
            <v-btn v-if="currentStep < 3" color="primary" @click="nextStep">
              下一步
              <v-icon end>mdi-arrow-right</v-icon>
            </v-btn>
          </div>
        </div>
      </div>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const emit = defineEmits(['close'])
const currentStep = ref(0)

// 向导导航方法
const nextStep = () => {
  if (currentStep.value < 3) {
    currentStep.value++
  }
}

const prevStep = () => {
  if (currentStep.value > 0) {
    currentStep.value--
  }
}

const skipAll = () => {
  complete()
}



const complete = () => {
  localStorage.setItem('welcomed', 'true')
  emit('close')
}
</script>

<style scoped>
.welcome-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background: rgb(var(--v-theme-surface));
  z-index: 9999;
  display: flex;
  align-items: center;
  justify-content: center;
}

.welcome-card {
  max-width: 600px;
  margin: 24px;
  background: transparent !important;
  display: flex;
  flex-direction: column;
}

.card-content {
  flex: 1;
  padding: 48px 48px 24px 48px;
  min-height: 400px;
}

.card-actions {
  border-top: thin solid rgba(var(--v-border-color), var(--v-border-opacity));
}

/* 步骤切换动画 */
.v-window {
  overflow: hidden;
}

.v-window-item {
  transition: all 0.3s ease;
}

/* 进入动画 */
.welcome-overlay {
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from {
    opacity: 0;
  }

  to {
    opacity: 1;
  }
}
</style>
