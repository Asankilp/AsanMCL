<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    :max-width="maxWidth"
  >
    <v-card>
      <v-card-title class="text-h5">{{ title }}</v-card-title>
      <v-card-text>
        <v-tabs
          v-model="activeTab"
          color="primary"
          align-tabs="center"
        >
          <v-tab value="microsoft">
            <v-icon start>mdi-microsoft</v-icon>
            Microsoft
          </v-tab>
          <v-tab value="offline">
            <v-icon start>mdi-account-off</v-icon>
            离线模式
          </v-tab>
          <v-tab value="custom">
            <v-icon start>mdi-account-cog</v-icon>
            自定义
          </v-tab>
        </v-tabs>

        <v-window v-model="activeTab" class="mt-4">
          <!-- Microsoft 登录 -->
          <v-window-item value="microsoft">
            <v-alert
              type="info"
              variant="tonal"
              :text="'使用 Microsoft 账户登录可以：\n- 进入正版服务器\n- 使用皮肤功能\n- 使用跨平台同步'"
              class="mb-4"
            ></v-alert>
            <v-btn
              color="primary"
              block
              prepend-icon="mdi-microsoft"
              variant="elevated"
              @click="handleMicrosoftLogin"
              :loading="loading"
            >
              使用 Microsoft 账户登录
            </v-btn>
          </v-window-item>

          <!-- 离线模式 -->
          <v-window-item value="offline">
            <v-form ref="offlineForm" v-model="offlineFormValid">
              <v-text-field
                v-model="offlineData.playerName"
                label="玩家名称"
                :rules="[v => !!v || '请输入玩家名称']"
                required
                variant="outlined"
                class="mb-4"
                hide-details="auto"
              ></v-text-field>
              
              <v-text-field
                v-model="offlineData.uuid"
                label="UUID (可选)"
                variant="outlined"
                placeholder="不填将自动生成"
                :rules="[
                  v => !v || /^[0-9a-f]{8}-[0-9a-f]{4}-4[0-9a-f]{3}-[89ab][0-9a-f]{3}-[0-9a-f]{12}$/i.test(v) || 'UUID 格式不正确'
                ]"
                hide-details="auto"
              ></v-text-field>
            </v-form>
          </v-window-item>

          <!-- 自定义账户 -->
          <v-window-item value="custom">
            <v-form ref="customForm" v-model="customFormValid">
              <v-text-field
                v-model="customData.username"
                label="用户名"
                :rules="[v => !!v || '请输入用户名']"
                required
                variant="outlined"
                class="mb-4"
                hide-details="auto"
              ></v-text-field>
              
              <v-text-field
                v-model="customData.email"
                label="邮箱"
                type="email"
                :rules="[
                  v => !!v || '请输入邮箱',
                  v => /.+@.+\..+/.test(v) || '请输入有效的邮箱地址'
                ]"
                required
                variant="outlined"
                class="mb-4"
                hide-details="auto"
              ></v-text-field>
              
              <v-text-field
                v-model="customData.password"
                label="密码"
                :type="showPassword ? 'text' : 'password'"
                :append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'"
                @click:append="showPassword = !showPassword"
                :rules="[v => !!v || '请输入密码']"
                required
                variant="outlined"
                hide-details="auto"
              ></v-text-field>
            </v-form>
          </v-window-item>
        </v-window>
      </v-card-text>
      
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="error"
          variant="text"
          @click="handleCancel"
        >
          取消
        </v-btn>
        <v-btn
          color="primary"
          variant="tonal"
          @click="handleSubmit"
          :loading="loading"
          :disabled="!isFormValid"
        >
          {{ submitButtonText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'

interface CustomAccountData {
  username: string
  email: string
  password: string
}

interface OfflineAccountData {
  playerName: string
  uuid?: string
}

const props = withDefaults(defineProps<{
  modelValue: boolean
  title: string
  loading?: boolean
  maxWidth?: number | string
  initialData?: Partial<CustomAccountData>
}>(), {
  loading: false,
  maxWidth: '500px',
  title: '添加新账户'
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'submit', data: { type: 'microsoft' | 'offline' | 'custom', data: any }): void
  (e: 'cancel'): void
}>()

const activeTab = ref('microsoft')
const showPassword = ref(false)

// 表单状态
const customFormValid = ref(false)
const offlineFormValid = ref(false)

// 自定义账户数据
const customData = reactive<CustomAccountData>({
  username: props.initialData?.username || '',
  email: props.initialData?.email || '',
  password: props.initialData?.password || ''
})

// 离线模式数据
const offlineData = reactive<OfflineAccountData>({
  playerName: '',
  uuid: undefined
})

const isFormValid = computed(() => {
  switch (activeTab.value) {
    case 'microsoft':
      return true
    case 'offline':
      return offlineFormValid.value
    case 'custom':
      return customFormValid.value
    default:
      return false
  }
})

const submitButtonText = computed(() => {
  switch (activeTab.value) {
    case 'microsoft':
      return '开始登录'
    case 'offline':
      return '创建账户'
    case 'custom':
      return '确认'
    default:
      return '确认'
  }
})

const handleMicrosoftLogin = () => {
  emit('submit', { type: 'microsoft', data: null })
}

const handleSubmit = () => {
  switch (activeTab.value) {
    case 'microsoft':
      handleMicrosoftLogin()
      break
    case 'offline':
      if (offlineFormValid.value) {
        emit('submit', { type: 'offline', data: { ...offlineData } })
      }
      break
    case 'custom':
      if (customFormValid.value) {
        emit('submit', { type: 'custom', data: { ...customData } })
      }
      break
  }
}

const handleCancel = () => {
  emit('cancel')
  emit('update:modelValue', false)
  // 重置表单
  activeTab.value = 'microsoft'
  customData.username = ''
  customData.email = ''
  customData.password = ''
  offlineData.playerName = ''
  offlineData.uuid = undefined
  showPassword.value = false
}
</script>

<style scoped>
:deep(.v-alert) {
  white-space: pre-line;
}
</style>
