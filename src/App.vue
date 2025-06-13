<template>
  <v-app>
    <!-- 欢迎页 -->
    <Hello
      v-if="showWelcome"
      @close="showWelcome = false"
    />

    <!-- 侧边导航栏 -->
    <v-navigation-drawer
      v-model="drawer"
      :rail="isRail"
      permanent
      color="surface"
    >
      <div class="px-2">
        <v-list>
          <v-list-item
            :prepend-avatar="currentUser.avatar"
            :title="currentUser.name"
            nav
            :active="$route.path === '/accounts'"
            class="account-item"
            @click="router.push('/accounts')"
          >
            <template v-slot:append>
              <v-btn
                variant="text"
                icon="mdi-chevron-left"
                @click.stop="isRail = !isRail"
              ></v-btn>
            </template>
          </v-list-item>
        </v-list>
      </div>

      <v-divider class="my-2"></v-divider>

      <!-- 导航菜单 -->
      <v-list density="compact" nav>
        <v-list-item
          v-for="item in menuItems"
          :key="item.to"
          :value="item.to"
          :to="item.to"
          :prepend-icon="item.icon"
          :title="item.title"
          :active="$route.path === item.to"
          rounded="lg"
          class="mx-2"
        ></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <!-- 主要内容区域 -->
    <v-main>
      <router-view />
    </v-main>

    <!-- 全局提示组件 -->
    <v-snackbar
      v-model="snackbar"
      :timeout="snackbarTimeout"
      location="top"
      :color="snackbarType"
    >
      {{ snackbarMessage }}
      <template v-slot:actions>
        <v-btn
          variant="text"
          @click="snackbar = false"
        >
          关闭
        </v-btn>
      </template>
    </v-snackbar>
  </v-app>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useSnackbar } from './composables/useSnackbar'
import Hello from './components/Hello.vue'
import { useAppTheme } from './composables/useTheme'

const router = useRouter()
const drawer = ref(true)
const isRail = ref(true)
const showWelcome = ref(false)

const currentUser = ref({
  name: 'Player',
  avatar: 'https://crafatar.com/avatars/steve'
})

const menuItems = [
  {
    title: '主页',
    icon: 'mdi-home',
    to: '/'
  },
  {
    title: '版本管理',
    icon: 'mdi-package-variant',
    to: '/versions'
  },
  {
    title: '设置',
    icon: 'mdi-cog',
    to: '/settings'
  }
]

const {
  snackbar,
  snackbarMessage,
  snackbarType,
  snackbarTimeout
} = useSnackbar()

// 初始化主题
const { loadTheme } = useAppTheme()
onMounted(() => {
  loadTheme()
})

onMounted(async () => {
  // const config = await invoke<LauncherConfig>("get_launcher_config_command");
  // showWelcome.value = true
})
</script>

<style>
:root {
  color-scheme: light dark;
}

html {
  overflow-y: auto !important;
}

.v-application {
  background: rgb(var(--v-theme-background)) !important;
}

:deep(.v-navigation-drawer),
:deep(.v-card) {
  background: rgb(var(--v-theme-surface)) !important;
}

.v-list-item {
  margin-inline: 12px;
  border-radius: 8px !important;
}

:deep(.v-list-item__overlay) {
  border-radius: 8px !important;
}

.v-list-item--active {
  background: rgb(var(--v-theme-primary), 0.15) !important;
}

.account-item {
  margin: 4px 12px !important;
  border-radius: 8px !important;
}

.account-item:hover {
  background: rgb(var(--v-theme-primary), 0.05) !important;
}

.account-item.v-list-item--active {
  background: rgb(var(--v-theme-primary), 0.15) !important;
}

:deep(.v-list-item__content) {
  padding: 8px 0;
}

/* 主题切换动画 */
.v-application,
.v-card,
.v-navigation-drawer,
.v-list-item {
  transition: background-color 0.3s ease, color 0.3s ease !important;
}
</style>