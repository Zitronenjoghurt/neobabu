<script lang="ts" setup>
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()
</script>

<template>
  <h2 class="text-3xl font-bold mb-2">Birthday</h2>
  <p class="text-color-secondary mb-5">
    Configure your birthdate and birthday notification preferences
  </p>
</template>
