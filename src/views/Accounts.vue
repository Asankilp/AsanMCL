<template>
  <v-container class="pa-0 fill-height" fluid>
    <!-- 顶部工具栏 -->
    <v-app-bar flat>
      <v-toolbar-title class="text-h6">我的账户</v-toolbar-title>
      <v-spacer></v-spacer>
      <v-btn
        icon
        color="primary"
        @click="accountDialogVisible = true"
        elevation="2"
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
        >
          <template v-slot:prepend>
            <v-avatar :image="getAvatarUrl(account.uuid)" size="40"></v-avatar>
          </template>
          
          <template v-slot:append>
            <v-btn
              variant="text"
              color="error"
              icon="mdi-delete"
              @click="handleDelete(account)"
            ></v-btn>
          </template>
        </v-list-item>

        <v-list-item v-if="accounts.length === 0" class="text-center">
          <v-list-item-title>暂无账户</v-list-item-title>
          <v-list-item-subtitle>点击右上角的加号添加账户</v-list-item-subtitle>
        </v-list-item>
      </v-list>
    </v-main>

    <!-- 账户表单对话框 -->
    <account-dialog
      v-model="accountDialogVisible"
      title="添加账户"
      :loading="loading"
      @cancel="handleCancel"
    />

    <!-- 确认删除对话框 -->
    <confirm-dialog
      v-model="deleteDialogVisible"
      title="确认删除"
      message="确定要删除此账户吗？此操作无法撤销。"
      confirm-text="删除"
      confirm-button-color="error"
      :loading="loading"
      @confirm="confirmDelete"
    />
  </v-container>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import AccountDialog from '../components/AccountDialog.vue'
import ConfirmDialog from '../components/ConfirmDialog.vue'
import { AccountConfig, AccountInfo, AccountType } from '../types/config/account'
import { useSnackbar } from '../composables/useSnackbar'
import { invoke } from '@tauri-apps/api/core'

const { showSuccess, showError } = useSnackbar()

const accounts = ref<AccountInfo[]>([])
const accountDialogVisible = ref(false)
const deleteDialogVisible = ref(false)
const loading = ref(false)
const accountToDelete = ref<AccountInfo | null>(null)

// 获取账户类型的显示文本
const getAccountTypeText = (type: AccountType): string => {
  const typeMap = {
    [AccountType.Microsoft]: '微软账户',
    [AccountType.Offline]: '离线账户',
    [AccountType.External]: '外置登录'
  }
  return typeMap[type]
}

// 获取头像URL
const getAvatarUrl = (uuid: string): string => {
  return `https://crafatar.com/avatars/${uuid}`
}

// 添加账户
// const handleSubmit = async (data: { type: "microsoft" | "offline" | "custom"; data: any }) => {
//   loading.value = true
//   try {
//     const account: AccountInfo = {
//       uuid: data.data.uuid,
//       name: data.data.name,
//       accountType: data.type as AccountType
//     }
    
//     // 检查是否已存在相同UUID的账户
//     if (accounts.value.some(a => a.uuid === account.uuid)) {
//       throw new Error('该账户已存在')
//     }
    
//     accounts.value.push(account)
//     await writeAccountConfig()
//     accountDialogVisible.value = false
//     showSuccess('账户添加成功')
//   } catch (error) {
//     showError((error as Error).message)
//   } finally {
//     loadAccounts()
//     loading.value = false
//   }
// }

const handleCancel = () => {
  accountDialogVisible.value = false
}

// 删除账户
const handleDelete = (account: AccountInfo) => {
  accountToDelete.value = account
  deleteDialogVisible.value = true
}

const confirmDelete = async () => {
  if (!accountToDelete.value) return
  
  loading.value = true
  try {
    const index = accounts.value.findIndex(a => a.uuid === accountToDelete.value?.uuid)
    if (index !== -1) {
      accounts.value.splice(index, 1)
      await writeAccountConfig()
      showSuccess('账户删除成功')
    }
    deleteDialogVisible.value = false
  } catch (error) {
    showError((error as Error).message)
  } finally {
    loading.value = false
    loadAccounts()
    accountToDelete.value = null
  }
}

onMounted(() => {
  // 初始化账户列表
  loadAccounts()
})

const loadAccounts = async () => {
  // 模拟加载账户数据
  const accountConfig = await invoke<AccountConfig>('get_account_config_command')
  accounts.value = accountConfig.accounts
}

const writeAccountConfig = async () => {
  // 将账户数据写入配置文件
  const accountConfig: AccountConfig = {
    accounts: accounts.value
  }
  await invoke('save_account_config_command', { config: accountConfig })
}
</script>

<style>
html {
  overflow-y: auto;
}
</style>
