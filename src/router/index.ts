import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../views/Home.vue')
  },  {
    path: '/accounts',
    name: 'Accounts',
    component: () => import('../views/Accounts.vue')
  },
  {
    path: '/versions',
    name: 'Versions',
    component: () => import('../views/Versions.vue')
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../views/Settings.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router
