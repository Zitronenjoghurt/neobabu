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
  <div class="mb-4">
    <Menubar :model="items" class="sticky">
      <template #start>
        <span class="font-bold text-xl text-primary">NeoBabu</span>
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
