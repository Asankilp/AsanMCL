<template>
  <v-dialog :model-value="modelValue" @update:model-value="$emit('update:modelValue', $event)" max-width="560">
    <v-card>
      <v-card-title class="text-h6">玩家信息</v-card-title>
      <v-card-text>
        <v-container>
          <v-row>
            <v-col cols="4" class="d-flex flex-column align-center">
              <v-avatar size="96" class="mb-2">
                <v-img :src="playerInfo.avatarUrl" alt="玩家头像"></v-img>
              </v-avatar>
              <v-img
                v-if="playerInfo.skinPreviewUrl"
                :src="playerInfo.skinPreviewUrl"
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
                  <v-list-item-subtitle>{{ playerInfo.username }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item>
                  <v-list-item-title>UUID</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.uuid }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="playerInfo.skins?.length">
                  <v-list-item-title>皮肤数量</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.skins.length }}</v-list-item-subtitle>
                </v-list-item>
                <v-list-item v-if="playerInfo.capes?.length">
                  <v-list-item-title>披风数量</v-list-item-title>
                  <v-list-item-subtitle>{{ playerInfo.capes.length }}</v-list-item-subtitle>
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
defineProps<{
  modelValue: boolean
  playerInfo: {
    username: string
    uuid: string
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
