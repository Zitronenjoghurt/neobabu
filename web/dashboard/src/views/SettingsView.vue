<script lang="ts" setup>
import { ref } from 'vue'
import BirthdaySettings from '@/components/settings/BirthdaySettings.vue'

const settingsCategories = ref([{ label: 'Birthday', value: 'birthday', icon: 'pi-gift' }])

const selectedCategory = ref('user')
</script>

<template>
  <main class="flex h-full overflow-hidden">
    <aside class="w-15rem flex flex-column surface-card surface-border border-1 border-round-left">
      <div class="p-3 border-bottom-1 surface-border">
        <h3 class="m-0 text-xl font-semibold">Settings</h3>
      </div>

      <div class="flex-1 overflow-y-auto p-2">
        <div
          v-for="category in settingsCategories"
          :key="category.value"
          :class="[
            'p-2 border-round cursor-pointer transition-colors transition-duration-150 mb-2 flex align-items-center gap-3',
            selectedCategory === category.value
              ? 'bg-primary text-primary-contrast'
              : 'hover:surface-100',
          ]"
          @click="selectedCategory = category.value"
        >
          <i :class="['pi', category.icon]"></i>
          <span class="font-medium">{{ category.label }}</span>
        </div>
      </div>
    </aside>

    <!-- Main Content -->
    <div class="flex-1 flex flex-column surface-card surface-border border-1 border-round-right">
      <div class="flex-1 overflow-y-auto">
        <div class="max-w-4xl mx-auto p-5">
          <BirthdaySettings v-if="selectedCategory === 'birthday'" />
          <div class="mt-5 pt-4 border-top-1 surface-border">
            <Button class="mr-2" label="Save Changes" />
            <Button label="Reset" severity="secondary" text />
          </div>
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.settings-section {
  animation: fadeIn 0.2s ease-in;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
