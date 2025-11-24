<script lang="ts" setup>
import { RouterView } from 'vue-router'
import { useAuthStore } from '@/stores/auth.ts'
import { onMounted } from 'vue'

const authStore = useAuthStore()
onMounted(() => authStore.fetchUser())
</script>

<template>
  <div class="flex flex-column h-screen p-3">
    <NavBar />
    <RouterView class="flex-1 overflow-hidden" />
  </div>
</template>
