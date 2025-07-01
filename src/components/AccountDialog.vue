<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" :max-width="maxWidth">
    <v-card>
      <v-card-title class="text-h5">{{ title }}</v-card-title>
      <v-card-text>
        <v-tabs v-model="activeTab" color="primary" align-tabs="center">
          <v-tab value="microsoft">
            <v-icon start>mdi-microsoft</v-icon>
            {{ $t('account.option.microsoft') }}
          </v-tab>
          <v-tab value="offline">
            <v-icon start>mdi-account-off</v-icon>
            {{ $t('account.option.offline') }}
          </v-tab>
          <v-tab value="custom">
            <v-icon start>mdi-account-cog</v-icon>
            {{ $t('account.option.external') }}
          </v-tab>
        </v-tabs>
        <v-window v-model="activeTab" class="mt-4">
          <!-- Microsoft 登录 -->
          <v-window-item value="microsoft">
            <microsoft-login-dialog ref="microsoftLoginRef" @login-success="handleLoginSuccess"
              @show-user-code="handleShowUserCode" @show-account-info="handleShowAccountInfo" />
          </v-window-item>

          <!-- 离线模式 -->
          <v-window-item value="offline">
            <v-form ref="offlineForm" v-model="offlineFormValid">
              <v-text-field v-model="offlineData.playerName" :label="$t('account.player_name')" :rules="[v => !!v || $t('account.player_name_required')]" required
                variant="outlined" class="mb-4" hide-details="auto"></v-text-field>

              <v-text-field v-model="offlineData.uuid" :label="$t('account.offline_uuid')" variant="outlined" :placeholder="$t('account.offline_uuid_placeholder')"
                :rules="[
                  v => !v || /^([0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}|[0-9a-f]{32})$/i.test(v) || $t('account.offline_uuid_invalid')
                ]" hide-details="auto"></v-text-field>
            </v-form>
          </v-window-item>

          <!-- 自定义账户 -->
          <v-window-item value="custom">
            <v-form ref="customForm" v-model="customFormValid">
              <v-text-field v-model="customData.username" :label="$t('general.username')" :rules="[v => !!v || $t('general.username_required')]" required
                variant="outlined" class="mb-4" hide-details="auto"></v-text-field>

              <v-text-field v-model="customData.email" :label="$t('general.email')" type="email" :rules="[
                v => !!v || '请输入邮箱',
                v => /.+@.+\..+/.test(v) || '请输入有效的邮箱地址'
              ]" required variant="outlined" class="mb-4" hide-details="auto"></v-text-field>

              <v-text-field v-model="customData.password" :label="$t('general.password')" :type="showPassword ? 'text' : 'password'"
                :append-icon="showPassword ? 'mdi-eye' : 'mdi-eye-off'" @click:append="showPassword = !showPassword"
                :rules="[v => !!v || '请输入密码']" required variant="outlined" hide-details="auto"></v-text-field>
            </v-form>
          </v-window-item>
        </v-window>
      </v-card-text>

      <v-card-actions>
        <v-spacer></v-spacer> <v-btn color="error" variant="text" @click="handleCancel">
          {{ $t('general.cancel') }}
        </v-btn>
        <v-btn color="primary" variant="tonal" @click="handleSubmit" @loading="loading" :disabled="!isFormValid">
          {{ submitButtonText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
  <!-- 设备码对话框 -->
  <user-code-dialog v-model="showUserCode" :auth-url="authUrl" :user-code="userCode" @cancel="handleUserCodeCancel"
    @loading="value => microsoftLoginRef?.setLoading(value)" />

  <!-- 玩家信息对话框 -->
  <account-info-dialog v-model="showAccountInfo" :account-info="accountInfo" />
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue'
import {
  type CustomAccountData,
  type OfflineAccountData,
} from '../types/account'
import MicrosoftLoginDialog from './MicrosoftLoginDialog.vue'
import UserCodeDialog from './UserCodeDialog.vue'
import AccountInfoDialog from './AccountInfoDialog.vue'
import { load } from '@tauri-apps/plugin-store'
import { AccountInfo, AccountType } from '../types/config/account'
import { v4 as uuidv4, parse as uuidParse } from 'uuid';
import { useI18n } from 'vue-i18n'

const microsoftLoginRef = ref<InstanceType<typeof MicrosoftLoginDialog> | null>(null)
const { t } = useI18n()
// Microsoft 登录相关的状态
const showUserCode = ref(false)
const authUrl = ref('')
const userCode = ref('')
const showAccountInfo = ref(false)
const accountInfo = ref<{
  username: string
  uuid: string
  type: AccountType
  avatarUrl?: string
  skinPreviewUrl?: string
  skins?: any[]
  capes?: any[]
}>({
  username: '',
  uuid: '',
  type: AccountType.Offline
})

const props = withDefaults(defineProps<{
  modelValue: boolean
  title: string
  loading: boolean
  maxWidth?: number | string
  initialData?: Partial<CustomAccountData>
}>(), {
  loading: false,
  maxWidth: '500px',
  title: 'Add Account',
})

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'update:loading', value: boolean): void
  (e: 'submit', data: AccountInfo): void
  (e: 'cancel'): void
}>()

// UI 状态
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
  uuid: undefined,
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
      return t('account.login')
    case 'offline':
      return t('account.create')
    case 'custom':
      return t('general.confirm')
    default:
      return t('general.confirm')
  }
})

const handleLoginSuccess = (data: AccountInfo) => {
  emit('submit', data)
  emit('update:modelValue', false)
  console.log('登录成功:', data)
}

const handleShowUserCode = (data: { authUrl: string, userCode: string, close?: boolean }) => {
  if (data.close) {
    showUserCode.value = false
    return
  }
  authUrl.value = data.authUrl
  userCode.value = data.userCode
  showUserCode.value = true
}

const handleUserCodeCancel = () => {
  showUserCode.value = false
  emit('update:loading', false)
}

const handleShowAccountInfo = (data: any) => {
  accountInfo.value = data
  showAccountInfo.value = true
}
const handleSubmit = async () => {
  emit('update:loading', true)
  try {
    switch (activeTab.value) {
      case 'microsoft':
        if (microsoftLoginRef.value) {
          await microsoftLoginRef.value.handleMicrosoftLogin()
        }
        break
      case 'offline':
        if (offlineFormValid.value) {
          if (offlineData.uuid === "") {
            offlineData.uuid = uuidv4()
          }
          const offlineInfo: AccountInfo = {
            accountType: AccountType.Offline,
            name: offlineData.playerName,
            uuid: offlineData.uuid?? uuidv4(),
          }
          console.log(offlineInfo)
          emit('submit', offlineInfo)
          emit('update:modelValue', false)
          resetForm()
        }
        break
      case 'custom':
        if (customFormValid.value) {
          // emit('submit', { type: 'custom', data: { ...customData } })
          emit('update:modelValue', false)
          resetForm()
        }
        break
    }
  } catch (error) {
    console.error('登录失败:', error)
  } finally {
    emit('update:loading', false)
  }
}

const resetForm = () => {
  activeTab.value = 'microsoft'
  customData.username = ''
  customData.email = ''
  customData.password = ''
  offlineData.playerName = ''
  offlineData.uuid = undefined
  showPassword.value = false
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
