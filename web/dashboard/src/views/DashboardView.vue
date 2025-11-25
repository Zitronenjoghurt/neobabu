<script lang="ts" setup>
import { ref } from 'vue'
import BirthdayModule from '@/components/dashboard/BirthdayModule.vue'
import { useGuildStore } from '@/stores/guild.ts'

const modules = ref([
  { label: 'APOD', value: 'apod' },
  { label: 'Birthday', value: 'birthday' },
])

const selectedModule = ref('dashboard')
const guildStore = useGuildStore()
</script>

<template>
  <main class="flex h-full overflow-hidden">
    <aside class="w-2 flex flex-column">
      <GuildList />
    </aside>

    <div
      v-if="guildStore.selectedGuild?.has_bot"
      class="flex-1 flex flex-column surface-card surface-border border-1 border-round-right"
    >
      <div class="p-2 surface-card border-bottom-1 surface-border flex align-items-center">
        <Dropdown
          v-model="selectedModule"
          :options="modules"
          class="w-15rem"
          optionLabel="label"
          optionValue="value"
          placeholder="Select a module"
          size="small"
        />
      </div>
      <div class="flex-1 p-4 overflow-y-auto">
        <component :is="selectedModule" />
        <BirthdayModule v-if="selectedModule === 'birthday'" />
      </div>
    </div>
    <div
      v-else-if="guildStore.selectedGuild"
      class="p-4 flex-1 flex flex-column surface-card surface-border border-1 border-round-right align-items-center justify-content-center text-center select-none"
    >
      <div class="mb-4">
        <i class="pi pi-exclamation-triangle text-7xl text-primary"></i>
      </div>
      <h1 class="text-5xl md:text-6xl font-bold mb-3">Bot is not on this server</h1>
      <p class="text-xl text-color-secondary mb-4">
        Once development reached a certain point you might be able to add it to your own server
        right here
      </p>
    </div>
    <div
      v-else
      class="p-4 flex-1 flex flex-column surface-card surface-border border-1 border-round-right align-items-center justify-content-center text-center select-none"
    >
      <div class="mb-4">
        <i class="pi pi-question-circle text-7xl text-primary"></i>
      </div>
      <h1 class="text-5xl md:text-6xl font-bold mb-3">No server selected</h1>
      <p class="text-xl text-color-secondary mb-4">
        Select a server on the left side to get started
      </p>
    </div>
  </main>
</template>
