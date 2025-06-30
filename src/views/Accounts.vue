<template>
  <v-container class="pa-0 fill-height" fluid>
    <!-- 顶部工具栏 -->
    <v-app-bar flat>
      <v-toolbar-title class="text-h6">{{ $t('account.my_accounts') }}</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn
        icon
        color="grey lighten-1"
        @click="accountDialogVisible = true"
      >
        <v-icon>mdi-plus</v-icon>
      </v-btn>
    </v-app-bar>

    <!-- 账户列表 -->
    <v-main>
      <v-list lines="two">
        <v-list-item
          v-for="account in accounts"
          :key="account.uuid"
          :title="account.name"
          :subtitle="getAccountTypeText(account.accountType)"
          rounded="lg"
          @click="selectedAccount && (selectedAccount = account); handleSelectionChange()"
          :class="{ 'selected-account': selectedAccount && selectedAccount.uuid === account.uuid }"
        >
          <template v-slot:prepend>
            <v-radio-group v-model="selectedAccount" hide-details @change="handleSelectionChange"> 
              <v-radio
                name="account-radio"
                v-model="selectedAccount"
                :value="account"
                color="primary"
                hide-details
              />
            </v-radio-group>
            <v-avatar :image="getAvatarUrl(account.uuid)" size="40"></v-avatar>
          </template>
          <template v-slot:append>
            <v-btn
              icon
              color="info"
              variant="text"
              @click.stop="showAccountInfo(account)"
            >
              <v-icon>mdi-account-circle</v-icon>
            </v-btn>
            <v-btn
              variant="text"
              color="error"
              icon="mdi-delete"
              @click.stop="handleDelete(account)"
            ></v-btn>
          </template>
        </v-list-item>

        <v-list-item v-if="accounts.length === 0" class="text-center">
          <v-list-item-title>{{ $t('account.no_account') }}</v-list-item-title>
          <v-list-item-subtitle>{{ $t('account.add_account_hint') }}</v-list-item-subtitle>
        </v-list-item>
      </v-list>
    </v-main>

    <!-- 账户表单对话框 -->
    <account-dialog
      v-model="accountDialogVisible"
      :title="$t('account.add_account')"
      :loading="loading"
      @cancel="handleCancel"
      @submit="handleSubmit"
    />

    <!-- 确认删除对话框 -->
    <confirm-dialog
      v-model="deleteDialogVisible"
      :title="$t('account.confirm_delete')"
      :message="$t('account.confirm_delete_hint')"
      :confirm-text="$t('general.delete')"
      confirm-button-color="error"
      :loading="loading"
      @confirm="confirmDelete"
    />

    <!-- 玩家信息对话框 -->
    <account-info-dialog
      v-model="accountInfoDialogVisible"
      :accountInfo="accountInfo"
    />
  </v-container>
</template>

<script setup lang="ts">
import { onMounted, ref, inject } from 'vue'
import { useI18n } from 'vue-i18n'
import AccountDialog from '../components/AccountDialog.vue'
import ConfirmDialog from '../components/ConfirmDialog.vue'
import AccountInfoDialog from '../components/AccountInfoDialog.vue'
import { AccountConfig, AccountInfo, AccountType } from '../types/config/account'
import { useSnackbar } from '../composables/useSnackbar'
import { invoke } from '@tauri-apps/api/core'
import { convertToCompactUUID } from '../utils/converter'
import { LauncherConfig } from '../types/config/launcher'
import { useAccountConfigStore, useLauncherConfigStore } from '../composables/useConfig'

const { showSuccess, showError } = useSnackbar()
const launcherConfigStore = useLauncherConfigStore()
const accountConfigStore = useAccountConfigStore()

const accounts = ref<AccountInfo[]>([])
const accountDialogVisible = ref(false)
const deleteDialogVisible = ref(false)
const loading = ref(false)
const accountToDelete = ref<AccountInfo | null>(null)
const selectedAccount = ref<AccountInfo | null>(null)
const accountInfoDialogVisible = ref(false)
const accountInfo = ref({
  username: '',
  uuid: '',
  type: AccountType.Offline,
  avatarUrl: '',
  skinPreviewUrl: '',
  skins: [],
  capes: []
})

const loadAvatar = inject('loadAvatar') as () => Promise<void>
const { t } = useI18n()

// 获取账户类型的显示文本
const getAccountTypeText = (type: AccountType): string => {
  const typeMap = {
    [AccountType.Microsoft]: t('account.microsoft'),
    [AccountType.Offline]: t('account.offline'),
    [AccountType.External]: t('account.external')
  }
  return typeMap[type]
}


// 获取头像URL
const getAvatarUrl = (uuid: string): string => {
  return `https://crafatar.com/avatars/${uuid}`
}

const handleSubmit = (account: AccountInfo) => {
  // 检查是否已存在相同UUID的账户
  if (accounts.value.some(a => convertToCompactUUID(a.uuid) === convertToCompactUUID(account.uuid))) {
    showError(t('account.account_existed'))
    throw new Error(t('account.account_existed'))
  } else {
    accounts.value.push(account)
    accountConfigStore.config.accounts = accounts.value
    writeAccountConfig()
  }
}

const handleCancel = () => {
  accountDialogVisible.value = false
}

// 删除账户
const handleDelete = (account: AccountInfo) => {
  accountToDelete.value = account
  deleteDialogVisible.value = true
}

const handleSelectionChange = async () => {
  const selectedUUID = selectedAccount.value?.uuid
  launcherConfigStore.config.selectedAccount = selectedUUID
  launcherConfigStore.saveConfig()
  await loadAvatar()
}

const confirmDelete = async () => {
  if (!accountToDelete.value) return
  loading.value = true
  try {
    const index = accounts.value.findIndex(a => a.uuid === accountToDelete.value?.uuid)
    const deletedIsSelected = selectedAccount.value?.uuid === accountToDelete.value?.uuid
    if (index !== -1) {
      accounts.value.splice(index, 1)
      accountConfigStore.config.accounts = accounts.value
      await writeAccountConfig()
      showSuccess(t('account.account_delete_success'))
    }
    deleteDialogVisible.value = false
    // 如果删除的是当前选中账户，自动选择第一个
    if (deletedIsSelected) {
      selectedAccount.value = accounts.value.length > 0 ? accounts.value[0] : null
      await handleSelectionChange()
    }
  } catch (error: string | any) {
    if (error !== undefined) {
      showError(error)
    }
  } finally {
    loading.value = false
    accountToDelete.value = null
  }
}

const showAccountInfo = async (account: AccountInfo) => {
  accountInfo.value = {
    username: account.name,
    uuid: account.uuid,
    type: account.accountType,
    avatarUrl: getAvatarUrl(account.uuid),
    skinPreviewUrl: await invoke('get_player_skin_preview_url', { uuid: account.uuid }), 
    skins: [],
    capes: []
  }
  accountInfoDialogVisible.value = true
}

onMounted(() => {
  // 初始化账户列表
  loadAccounts()
})

const loadAccounts = async () => {
  accounts.value = accountConfigStore.config.accounts
  selectedAccount.value = accountConfigStore.config.accounts.find(a => a.uuid === launcherConfigStore.config.selectedAccount) ?? null
}

const writeAccountConfig = async () => {
  // 将账户数据写入配置文件
  accountConfigStore.saveConfig()
}
</script>

<style>
html {
  overflow-y: auto;
}

/* .selected-account {
  background-color: rgba(0, 0, 255, 0.1) !important;
} */
</style>