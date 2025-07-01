import { createApp, ref, watch } from "vue";
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
import zhHans from './lang/zh-hans.json'
import en from './lang/en.json'
import ja from './lang/ja.json'
import { createI18n } from 'vue-i18n'

export const appLocale = ref('en') // 先导出，初始值为'en'
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
const init = async () => {
  const app = createApp(App);
  const pinia = createPinia();

  app.use(vuetify);
  app.use(router);
  app.use(pinia);

  const launcherConfigStore = useLauncherConfigStore();
  const accountConfigStore = useAccountConfigStore();
  await launcherConfigStore.loadConfig();
  await accountConfigStore.loadConfig();

  appLocale.value = launcherConfigStore.config.language || 'en';

  type MessageSchema = typeof en;
  const i18n = createI18n<[MessageSchema], 'en' | 'zh-hans' | 'ja'>({
    locale: appLocale.value,
    messages: {
      'zh-hans': zhHans,
      'en': en,
      'ja': ja
    }
  });
  app.use(i18n);

  // 监听 appLocale 的变化并同步到 i18n
  appLocale.value && watch(appLocale, (newLocale) => {
    i18n.global.locale = newLocale as 'en' | 'zh-hans' | 'ja';
  });

  app.mount("#app");
};

init();
