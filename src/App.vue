<template>
  <v-app>
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
  </v-app>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()
const drawer = ref(true)
const isRail = ref(true)

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
</script>

<style>
:root {
  color-scheme: dark;
}

html {
  overflow-y: auto !important;
}

.v-application {
  background: rgb(18, 18, 18) !important;
}

:deep(.v-navigation-drawer),
:deep(.v-card) {
  background: rgb(30, 30, 30) !important;
}

.v-list-item {
  margin-inline: 12px;
  border-radius: 8px !important;
}

:deep(.v-list-item__overlay) {
  border-radius: 8px !important;
}

.v-list-item--active {
  background: rgba(var(--v-theme-primary), 0.15) !important;
}

.account-item {
  margin: 4px 12px !important;
  border-radius: 8px !important;
}

.account-item:hover {
  background: rgba(var(--v-theme-primary), 0.05) !important;
}

.account-item.v-list-item--active {
  background: rgba(var(--v-theme-primary), 0.15) !important;
}

:deep(.v-list-item__content) {
  padding: 8px 0;
}
</style>