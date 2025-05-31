<template>
  <v-dialog
    :model-value="modelValue"
    @update:model-value="$emit('update:modelValue', $event)"
    :max-width="maxWidth"
  >
    <v-card>
      <v-card-title class="text-h5">{{ title }}</v-card-title>
      <v-card-text>{{ message }}</v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          :color="cancelButtonColor"
          :variant="cancelButtonVariant"
          @click="$emit('update:modelValue', false)"
        >
          {{ cancelText }}
        </v-btn>
        <v-btn
          :color="confirmButtonColor"
          :variant="confirmButtonVariant"
          @click="handleConfirm"
          :loading="loading"
        >
          {{ confirmText }}
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
  (e: 'confirm'): void
}>()

withDefaults(defineProps<{
  modelValue: boolean
  title: string
  message: string
  loading?: boolean
  maxWidth?: number | string
  cancelText?: string
  confirmText?: string
  cancelButtonColor?: string
  confirmButtonColor?: string
  cancelButtonVariant?: 'flat' | 'text' | 'elevated' | 'tonal' | 'outlined' | 'plain'
  confirmButtonVariant?: 'flat' | 'text' | 'elevated' | 'tonal' | 'outlined' | 'plain'
}>(), {
  loading: false,
  maxWidth: '400px',
  cancelText: '取消',
  confirmText: '确认',
  cancelButtonColor: 'primary',
  confirmButtonColor: 'primary',
  cancelButtonVariant: 'text',
  confirmButtonVariant: 'tonal'
})

const handleConfirm = () => {
  emit('confirm')
}
</script>
