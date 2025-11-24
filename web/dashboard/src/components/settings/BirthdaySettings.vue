<script lang="ts" setup>
import { computed, onMounted, ref } from 'vue'
import { useSettingsStore } from '@/stores/settings'

const settingsStore = useSettingsStore()

const birthdate = ref({
  day: null as number | null,
  month: null as number | null,
  year: null as number | null,
})

const months = [
  { label: 'January', value: 1 },
  { label: 'February', value: 2 },
  { label: 'March', value: 3 },
  { label: 'April', value: 4 },
  { label: 'May', value: 5 },
  { label: 'June', value: 6 },
  { label: 'July', value: 7 },
  { label: 'August', value: 8 },
  { label: 'September', value: 9 },
  { label: 'October', value: 10 },
  { label: 'November', value: 11 },
  { label: 'December', value: 12 },
]

const days = computed(() => Array.from({ length: 31 }, (_, i) => i + 1))

const isLocked = computed(() => {
  if (!settingsStore.settings?.birthday?.updated_at) return false

  const daysPassed =
    (Date.now() / 1000 - settingsStore.settings.birthday.updated_at) / (60 * 60 * 24)
  return daysPassed < 180
})

const daysUntilUnlock = computed(() => {
  if (!settingsStore.settings?.birthday?.updated_at) return 0

  const daysPassed =
    (Date.now() / 1000 - settingsStore.settings.birthday.updated_at) / (60 * 60 * 24)
  return Math.ceil(180 - daysPassed)
})

onMounted(() => {
  if (settingsStore.settings?.birthday) {
    const bd = settingsStore.settings.birthday
    birthdate.value = {
      day: bd.day,
      month: bd.month,
      year: bd.year || null,
    }
  }
})
</script>

<template>
  <h2 class="text-3xl font-bold mb-2">Birthday</h2>
  <p class="text-color-secondary mb-5">
    Configure your birthdate and birthday notification preferences
  </p>

  <div class="field mb-4">
    <label class="block font-semibold mb-2">Birthday Date</label>
    <div class="flex gap-2">
      <Dropdown
        v-model="birthdate.month"
        :disabled="isLocked"
        :options="months"
        class="flex-1"
        optionLabel="label"
        optionValue="value"
        placeholder="Month"
      />
      <Dropdown
        v-model="birthdate.day"
        :disabled="isLocked"
        :options="days"
        class="flex-1"
        placeholder="Day"
      />
      <InputNumber
        v-model="birthdate.year"
        :disabled="isLocked"
        :useGrouping="false"
        class="flex-1"
        placeholder="Year (optional)"
      />
    </div>
    <small v-if="isLocked" class="text-color-secondary mt-2 block">
      <i class="pi pi-lock mr-1"></i>
      Can't change for {{ daysUntilUnlock }} more days
    </small>
  </div>
</template>
