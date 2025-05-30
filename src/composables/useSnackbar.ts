import { ref } from 'vue'

export interface SnackbarOptions {
  message: string
  type?: 'success' | 'error' | 'warning' | 'info'
  timeout?: number
}

const snackbar = ref(false)
const snackbarMessage = ref('')
const snackbarType = ref<'success' | 'error' | 'warning' | 'info'>('info')
const snackbarTimeout = ref(3000)

export function useSnackbar() {
  const showSnackbar = (options: SnackbarOptions) => {
    snackbarMessage.value = options.message
    snackbarType.value = options.type || 'info'
    snackbarTimeout.value = options.timeout || 3000
    snackbar.value = true
  }

  const showSuccess = (message: string, timeout?: number) => {
    showSnackbar({ message, type: 'success', timeout })
  }

  const showError = (message: string, timeout?: number) => {
    showSnackbar({ message, type: 'error', timeout })
  }

  const showWarning = (message: string, timeout?: number) => {
    showSnackbar({ message, type: 'warning', timeout })
  }

  const showInfo = (message: string, timeout?: number) => {
    showSnackbar({ message, type: 'info', timeout })
  }

  return {
    snackbar,
    snackbarMessage,
    snackbarType,
    snackbarTimeout,
    showSnackbar,
    showSuccess,
    showError,
    showWarning,
    showInfo
  }
}
