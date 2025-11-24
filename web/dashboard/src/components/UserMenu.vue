<script lang="ts" setup>
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'
import { computed } from 'vue'

const authStore = useAuthStore()
const router = useRouter()

const avatarUrl = computed(() => {
  if (!authStore.user?.avatar_hash) return null
  return `https://cdn.discordapp.com/avatars/${authStore.user.id}/${authStore.user.avatar_hash}.png`
})

const userMenuItems = [
  {
    label: 'Settings',
    icon: 'pi pi-cog',
    class: 'p-ripple',
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
</script>

<template>
  <div v-if="authStore.user" class="flex align-items-center gap-2">
    <SplitButton :label="authStore.user.username" :model="userMenuItems" rounded text>
      <template #icon>
        <img
          v-if="avatarUrl"
          :alt="authStore.user.username"
          :src="avatarUrl"
          class="w-2rem h-2rem border-circle"
        />
        <i v-else class="pi pi-user"></i>
      </template>
    </SplitButton>
  </div>
  <Button v-else icon="pi pi-sign-in" label="Login" @click="authStore.login" />
</template>
