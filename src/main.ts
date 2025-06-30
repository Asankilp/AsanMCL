import { createApp } from "vue";
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';
import { aliases, mdi } from 'vuetify/iconsets/mdi';
import router from './router';
import App from "./App.vue";

// Vuetify
import 'vuetify/styles';
import '@mdi/font/css/materialdesignicons.css';
import { createPinia } from "pinia";
import { useAccountConfigStore, useLauncherConfigStore } from "./composables/useConfig";
import zhs from './lang/zhs.json'
import en from './lang/en.json'
import { createI18n } from 'vue-i18n'
const vuetify = createVuetify({
  components,
  directives,
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi,
    },
  },
  theme: {
    // defaultTheme: window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light',
    themes: {
      light: {
        colors: {
          primary: '#6750A4',
          secondary: '#625B71',
          error: '#B3261E',
          background: '#FFFBFE',
          surface: '#FFFFFF',
          'surface-variant': '#E7E0EC',
          'on-surface': '#1C1B1F',
          'on-surface-variant': '#49454F',
        }
      },
      dark: {
        colors: {
          primary: '#BB86FC',
          secondary: '#03DAC6',
          error: '#CF6679',
          background: '#121212',
          surface: '#1E1E1E',
          'surface-variant': '#2B2B2B',
          'on-surface': '#FFFFFF',
          'on-surface-variant': '#E6E1E5',
        }
      }
    },
  }
});

// 监听系统主题变化
// const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
// const handleThemeChange = (e: MediaQueryListEvent | MediaQueryList) => {
//   if (vuetify) {
//     vuetify.theme.global.name.value = e.matches ? 'dark' : 'light';
//   }
// };

// // 初始化和监听主题变化
// handleThemeChange(mediaQuery);
// mediaQuery.addEventListener('change', handleThemeChange);

type MessageSchema = typeof zhs

const app = createApp(App);
const pinia = createPinia();
const i18n = createI18n<[MessageSchema], 'zhs' | 'en'>({
  locale: 'zhs',
  messages: {
    'zhs': zhs,
    'en': en
  }
})


app.use(vuetify);
app.use(router);
app.use(pinia);
app.use(i18n)
const launcherConfigStore = useLauncherConfigStore();
const accountConfigStore = useAccountConfigStore();
launcherConfigStore.loadConfig()
accountConfigStore.loadConfig()

app.mount("#app");
