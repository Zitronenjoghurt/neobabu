<script lang="ts" setup>
import { RouterView } from 'vue-router'
import { useAuthStore } from '@/stores/auth.ts'
import { onMounted } from 'vue'

const authStore = useAuthStore()
onMounted(() => authStore.fetchUser())
</script>

<template>
  <main class="p-2">
    <NavBar />
    <RouterView />
  </main>
</template>
