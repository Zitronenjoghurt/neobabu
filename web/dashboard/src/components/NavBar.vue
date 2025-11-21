<script lang="ts" setup>
import { ref } from 'vue'

const items = ref([
  { label: 'Home', icon: 'pi pi-home', route: '/' },
  { label: 'About', icon: 'pi pi-info-circle', route: '/' },
])
</script>

<template>
  <div class="mb-4">
    <Menubar :model="items" class="sticky">
      <template #start>
        <span class="font-bold text-xl">NeoBabu</span>
      </template>
      <template #end>
        <UserMenu />
      </template>
    </Menubar>
  </div>
</template>
