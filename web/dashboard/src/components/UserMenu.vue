<script lang="ts" setup>
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

const authStore = useAuthStore()
const router = useRouter()

const userMenuItems = [
  {
    label: 'Profile',
    icon: 'pi pi-user',
    command: () => router.push('/profile'),
  },
  {
    label: 'Settings',
    icon: 'pi pi-cog',
    command: () => router.push('/settings'),
  },
  {
    separator: true,
  },
  {
    label: 'Logout',
    icon: 'pi pi-sign-out',
    command: () => authStore.logout(),
  },
]

function handleLogin() {
  router.push('/login')
}
</script>

<template>
  <div v-if="authStore.isAuthenticated" class="flex align-items-center gap-2">
    <span class="text-sm">{{ authStore.userName }}</span>
    <SplitButton :model="userMenuItems" icon="pi pi-user" label="Account" rounded text />
  </div>
  <Button v-else icon="pi pi-sign-in" label="Login" @click="handleLogin" />
</template>
