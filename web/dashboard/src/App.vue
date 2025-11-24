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
    <RouterView v-slot="{ Component }">
      <Transition mode="out-in" name="fade">
        <component :is="Component" class="flex-1 overflow-hidden" />
      </Transition>
    </RouterView>
  </div>
</template>

<style>
.fade-enter-active {
  transition: all 0.2s ease-out;
}

.fade-leave-active {
  transition: all 0.2s ease-in;
}

.fade-enter-from {
  opacity: 0;
  transform: translateY(10px) scale(0.98);
}

.fade-leave-to {
  opacity: 0;
  transform: scale(1.02);
}
</style>
