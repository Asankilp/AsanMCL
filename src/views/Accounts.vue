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
      <v-list lines="two" select-strategy="false">
        <v-list-item
          v-if="currentAccount"
          :key="currentAccount.username"
          :title="currentAccount.username"
          :subtitle="currentAccount.email"
          rounded="lg"
        >
          <template v-slot:prepend>
            <v-avatar :image="currentAccount.avatar" size="40"></v-avatar>
          </template>
          
          <template v-slot:append>
            <v-btn
              variant="text"
              color="error"
              icon="mdi-delete"
              @click="handleDelete(currentAccount)"
            ></v-btn>
          </template>
        </v-list-item>
      </v-list>
    </v-main>

    <!-- 账户表单对话框 -->
    <account-dialog
      v-model="accountDialogVisible"
      :loading="loading"
      @submit="handleSubmit"
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
import { ref } from 'vue'
import AccountDialog from '../components/AccountDialog.vue'
import ConfirmDialog from '../components/ConfirmDialog.vue'

interface Account {
  username: string
  email: string
  avatar: string
}

const currentAccount = ref<Account | null>({
  username: "Player",
  email: "player@example.com",
  avatar: "https://crafatar.com/avatars/steve"
})

const accountDialogVisible = ref(false)
const deleteDialogVisible = ref(false)
const loading = ref(false)
const accountToDelete = ref<Account | null>(null)

const handleDelete = (account: Account) => {
  accountToDelete.value = account
  deleteDialogVisible.value = true
}

const confirmDelete = async () => {
  if (!accountToDelete.value) return
  
  loading.value = true
  try {
    // TODO: 实现删除逻辑
    await new Promise(resolve => setTimeout(resolve, 1000))
    deleteDialogVisible.value = false
  } finally {
    loading.value = false
  }
}

const handleSubmit = async (data: { type: 'microsoft' | 'offline' | 'custom', data: any }) => {
  loading.value = true
  try {
    switch (data.type) {
      case 'microsoft':
        console.log('处理 Microsoft 登录...')
        // TODO: 实现 Microsoft OAuth 登录
        break
      case 'offline':
        console.log('创建离线账户:', data.data)
        // TODO: 保存离线账户信息
        break
      case 'custom':
        console.log('添加自定义账户:', data.data)
        // TODO: 保存自定义账户信息
        break
    }
    await new Promise(resolve => setTimeout(resolve, 1000))
    accountDialogVisible.value = false
  } finally {
    loading.value = false
  }
}

const handleCancel = () => {
  // 可以在这里添加取消时的额外逻辑
}
</script>

<style>
html {
  overflow-y: auto;
}
</style>
