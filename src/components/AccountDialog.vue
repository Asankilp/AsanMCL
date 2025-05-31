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
              :loading="isLoading"
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
  <!-- 设备码对话框 -->
  <v-dialog v-model="showDeviceCode" max-width="400px" persistent>
    <v-card>
      <v-card-title class="text-h6">Microsoft 账户登录</v-card-title>
      <v-card-text class="text-center">
        <p class="mb-4">请访问以下网址并输入设备码：</p>
        <v-btn
          block
          color="primary"
          class="mb-4"
          :href="authUrl"
          target="_blank"
          rel="noopener noreferrer"
          @click="showSuccess('已在浏览器中打开授权页面')"
        >
          打开授权页面
        </v-btn>
        <v-card variant="outlined" class="mb-4 pa-4 cursor-pointer" @click="copyToClipboard(deviceCode)">
          <p class="text-h5 font-weight-bold">{{ deviceCode }}</p>
          <v-icon icon="mdi-content-copy" size="small" class="ms-2"></v-icon>
        </v-card>
        <p class="text-caption">点击上方的代码可复制到剪贴板</p>
        <p class="text-caption mt-2">正在等待授权完成...</p>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="error" variant="text" @click="showDeviceCode = false">
          取消
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>

  <!-- 玩家信息对话框 -->
  <v-dialog v-model="showPlayerInfo" max-width="500">
    <v-card>
      <v-card-title class="text-h6">玩家信息</v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="4" class="d-flex flex-column align-center">
              <v-avatar size="96" class="mb-2">
                <v-img :src="playerInfo.avatarUrl" alt="玩家头像"></v-img>
              </v-avatar>
              <v-img
                v-if="playerInfo.skinPreviewUrl"
                :src="playerInfo.skinPreviewUrl"
                width="128"
                height="128"
                class="mt-2"
                alt="皮肤预览"
              ></v-img>
            </v-col>
            <v-col cols="8">
              <v-list>
                <v-list-item>
                  <v-list-item-title>用户名</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.username }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <v-list-item-title>UUID</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.uuid }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="playerInfo.skins?.length">
                  <v-list-item-title>皮肤数量</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.skins.length }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="playerInfo.capes?.length">
                  <v-list-item-title>披风数量</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.capes.length }}</v-list-item-subtitle>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="primary"
          variant="text"
          @click="showPlayerInfo = false"
        >
          确定
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import { useSnackbar } from '../composables/useSnackbar'
import { 
  type CustomAccountData, 
  type OfflineAccountData, 
  type UserCodeResult,
  type MinecraftAuthResponse, 
} from '../types/account'
import { LoginEvent } from '../types/event'
import { Channel, invoke } from '@tauri-apps/api/core'
import { MinecraftProfile, SkinData, CapeData } from '../types/mojang'

const { showSuccess, showError } = useSnackbar()

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

// 加载状态
const isLoading = ref(false)

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

// Microsoft 登录相关
const showDeviceCode = ref(false)
const authUrl = ref('')
const deviceCode = ref('')

// 玩家信息对话框
const showPlayerInfo = ref(false)
const playerInfo = ref<{
  username: string
  uuid: string
  avatarUrl?: string
  skinPreviewUrl?: string
  skins?: any[]
  capes?: any[]
}>({
  username: '',
  uuid: ''
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

const copyToClipboard = async (text: string) => {
  try {
    // await writeText(text)
    showSuccess('已复制到剪贴板')
  } catch (err) {
    console.error('Failed to copy text:', err)
    showError('复制失败')
  }
}

const handleMicrosoftLogin = async () => {
  try {
    // 显示加载状态
    isLoading.value = true
    
    // 创建事件通道
    const onEvent = new Channel<LoginEvent>()
    
    // 监听事件
    onEvent.onmessage = async (message: LoginEvent) => {
      if (message.event === 'started') {
        authUrl.value = 'https://microsoft.com/link'
        deviceCode.value = message.data.code
        showDeviceCode.value = true
      } else if (message.event === 'finished') {
        showDeviceCode.value = false
        const result = message.data.response
        
        try {
          const profile: MinecraftProfile = await invoke("get_minecraft_profile", {
            accessToken: result.accessToken
          }) 
          const avatarUrl: string = await invoke('get_player_avatar_url', { uuid: profile.id })
          const skinPreviewUrl: string = await invoke('get_player_skin_preview_url', { uuid: profile.id })

          // 更新玩家信息
          playerInfo.value = {
            username: profile.name,
            uuid: profile.id,
            avatarUrl,
            skinPreviewUrl,
            skins: profile.skins,
            capes: profile.capes
          }
          // 显示玩家信息对话框
          showPlayerInfo.value = true
          
          showSuccess('登录成功！')
          emit('submit', {
            type: 'microsoft',
            data: {
              username: profile.name,
              accessToken: result.accessToken,
              tokenType: result.tokenType,
              expiresIn: result.expiresIn,
              uuid: profile.id,
              avatarUrl,
              skinPreviewUrl,
              skins: profile.skins,
              capes: profile.capes
            }
          })
        } catch (err) {
          console.error('获取玩家信息失败:', err)
          showError('获取玩家信息失败，但登录已成功')
          emit('submit', {
            type: 'microsoft',
            data: {
              username: result.username,
              accessToken: result.accessToken,
              tokenType: result.tokenType,
              expiresIn: result.expiresIn
            }
          })
        }
      }
    }

    // 启动登录流程
    await invoke('get_device_code', { onEvent })

  } catch (error) {
    console.error('获取设备码失败:', error)
    showError('获取登录代码失败，请稍后重试')
  } finally {
    isLoading.value = false
  }
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

.v-chip {
  cursor: pointer;
}
</style>
