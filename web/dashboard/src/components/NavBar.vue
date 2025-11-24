<script lang="ts" setup>
import { computed } from 'vue'
import { useAuthStore } from '@/stores/auth.ts'

const authStore = useAuthStore()

const items = computed(() => {
  const baseItems = [{ label: 'Home', icon: 'pi pi-home', route: '/' }]

  if (authStore.isAuthenticated) {
    baseItems.push({ label: 'Dashboard', icon: 'pi pi-chart-bar', route: '/dashboard' })
  }

  return baseItems
})
</script>

<template>
  <div class="mb-3">
    <Menubar :model="items" class="sticky navbar-wrapper">
      <template #start>
        <div class="flex align-items-center gap-2 mr-2">
          <Avatar image="/neobabu.png" shape="circle" size="large" />
          <span class="font-bold text-xl text-primary">NeoBabu</span>
        </div>
      </template>
      <template #item="{ item, props }">
        <router-link v-if="item.route" v-slot="{ href, navigate }" :to="item.route" custom>
          <a :href="href" v-bind="props.action" @click="navigate">
            <span :class="item.icon" />
            <span class="ml-2">{{ item.label }}</span>
          </a>
        </router-link>
        <a v-else :href="item.url" :target="item.target" v-bind="props.action">
          <span :class="item.icon" />
          <span class="ml-2">{{ item.label }}</span>
        </a>
      </template>
      <template #end>
        <UserMenu />
      </template>
    </Menubar>
  </div>
</template>

<style scoped>
.navbar-wrapper {
  position: relative;
  z-index: 1000;
}
</style>
