<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="600">
    <v-card>
      <v-card-title class="text-h6">账户信息</v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="4" class="d-flex flex-column align-center">
              <v-avatar size="96" class="mb-2">
                <v-img :src="accountInfo.avatarUrl" alt="玩家头像"></v-img>
              </v-avatar>
              <v-img
                v-if="accountInfo.skinPreviewUrl"
                :src="accountInfo.skinPreviewUrl"
                width="128"
                height="128"
                class="mt-2"
                alt="皮肤预览"
              ></v-img>
            </v-col>
            <v-col cols="8">
              <v-list>
                <v-list-item>
                  <v-list-item-title>用户名</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.username }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <v-list-item-title>UUID</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.uuid }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <v-list-item-title>账户类型</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.type }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="accountInfo.skins?.length">
                  <v-list-item-title>皮肤数量</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.skins.length }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="accountInfo.capes?.length">
                  <v-list-item-title>披风数量</v-list-item-title>
                  <v-list-item-subtitle>{{ accountInfo.capes.length }}</v-list-item-subtitle>
                </v-list-item>
              </v-list>
            </v-col>
          </v-row>
        </v-container>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn
          color="primary"
          variant="text"
          @click="emit('update:modelValue', false)"
        >
          确定
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { AccountType } from '../types/config/account';

defineProps<{
  modelValue: boolean
  accountInfo: {
    username: string
    uuid: string
    type: AccountType
    avatarUrl?: string
    skinPreviewUrl?: string
    skins?: any[]
    capes?: any[]
  }
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void
}>()
</script>
