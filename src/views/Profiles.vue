<template>
    <v-container class="pa-0 fill-height" fluid>
        <!-- 顶部工具栏 -->
        <v-app-bar flat>
            <v-toolbar-title class="text-h6">{{ $t('profile.profiles') }}</v-toolbar-title>
            <v-spacer></v-spacer>
            <v-btn icon color="grey lighten-1" @click=openDialog>
                <v-icon>mdi-plus</v-icon>
            </v-btn>
        </v-app-bar>

        <!-- 配置列表 -->
        <v-main>
            <v-list lines="two">
                <v-list-item v-for="profile in profiles" :key="profile.id" :title="profile.name"
                    :subtitle="profile.description" rounded="lg" @click="editProfile(profile)">
                    <template v-slot:prepend>
                        <v-avatar color="primary" size="40">
                            <v-icon>mdi-gamepad-variant</v-icon>
                        </v-avatar>
                    </template>
                    <template v-slot:append>
                        <div class="d-flex align-center">
                            <v-menu offset-y>
                                <template v-slot:activator="{ props }">
                                    <v-btn icon color="lighten-1" v-bind="props">
                                        <v-icon>mdi-dots-vertical</v-icon>
                                    </v-btn>
                                </template>
                                <v-list>
                                    <v-list-item @click.stop="editProfile(profile)">
                                        <v-list-item-title>编辑</v-list-item-title>
                                    </v-list-item>
                                    <v-list-item @click.stop="deleteProfile(profile.id)">
                                        <v-list-item-title class="text-error">删除</v-list-item-title>
                                    </v-list-item>
                                </v-list>
                            </v-menu>
                        </div>
                    </template>
                </v-list-item>
                <v-list-item v-if="profiles.length === 0" class="text-center">
                    <v-list-item-title>{{ $t('profile.no_profile') }}</v-list-item-title>
                    <v-list-item-subtitle>{{ $t('profile.add_profile_hint') }}</v-list-item-subtitle>
                </v-list-item>
            </v-list>
        </v-main>

        <!-- 配置编辑/新建对话框 -->
        <v-dialog v-model="dialog" max-width="500">
            <v-card>
                <v-card-title>{{ editingProfile ? '编辑配置' : '新建配置' }}</v-card-title>
                <v-divider></v-divider>
                <v-card-text>
                    <v-text-field v-model="form.name" label="配置名称" required></v-text-field>
                    <v-textarea v-model="form.description" label="描述" rows="2"></v-textarea>
                </v-card-text>
                <v-card-actions>
                    <v-spacer />
                    <v-btn text @click="closeDialog">{{ $t('general.cancel') }}</v-btn>
                    <v-btn color="primary" @click="saveProfile">{{ $t('general.save') }}</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Profile {
    id: string
    name: string
    description: string
}

const profiles = ref<Profile[]>([])
const dialog = ref(false)
const editingProfile = ref<Profile | null>(null)
const form = ref({
    name: '',
    description: ''
})

const openDialog = () => {
    editingProfile.value = null
    form.value = { name: '', description: '' }
    dialog.value = true
}

const closeDialog = () => {
    dialog.value = false
}

const saveProfile = async () => {
    if (!form.value.name) return
    if (editingProfile.value) {
        // 编辑
        editingProfile.value.name = form.value.name
        editingProfile.value.description = form.value.description
    } else {
        // 新建
        profiles.value.push({
            id: Date.now().toString(),
            name: form.value.name,
            description: form.value.description
        })
    }
    dialog.value = false
}

const editProfile = (profile: Profile) => {
    editingProfile.value = profile
    form.value = { name: profile.name, description: profile.description }
    dialog.value = true
}

const deleteProfile = (id: string) => {
    profiles.value = profiles.value.filter(p => p.id !== id)
}
</script>

<style scoped>
.v-list-item {
    margin-inline: 12px;
    border-radius: 8px !important;
}

.v-list-item--active {
    background: rgb(var(--v-theme-primary), 0.15) !important;
}

.v-list-item:hover {
    background: rgb(var(--v-theme-primary), 0.05) !important;
}
</style>
