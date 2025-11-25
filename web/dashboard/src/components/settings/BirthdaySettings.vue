<script lang="ts" setup>
import { Form } from '@primevue/forms'
import { useSettingsStore } from '@/stores/settings'
import { computed, onMounted, ref } from 'vue'

const settingsStore = useSettingsStore()

const birthDate = ref<Date | null>(null)
const includeYear = ref(false)

const canChangeDate = computed(() => {
  return settingsStore.settings?.birthday?.updatable ?? true
})

onMounted(() => {
  const birthday = settingsStore.settings?.birthday

  if (birthday) {
    birthDate.value = new Date(birthday.year ?? 2000, birthday.month - 1, birthday.day)
    includeYear.value = !!birthday.year
  }
})
</script>

<template>
  <div class="text-center justify-content-center align-items-center">
    <h2 class="text-3xl font-bold mb-2">Birthday</h2>
    <p class="text-color-secondary">
      Configure your birthdate and birthday notification preferences
    </p>
    <Divider class="p-2" />
  </div>
  <div class="flex flex-1 justify-content-center">
    <Form>
      <div>
        <FloatLabel variant="on">
          <DatePicker
            v-model="birthDate"
            :disabled="!canChangeDate"
            date-format="dd/mm/yy"
            input-id="birthday"
            name="birthday"
            show-icon
          />
          <label class="surface-0 border-round" for="birthday">Birthday</label>
        </FloatLabel>
        <div v-if="!canChangeDate" class="flex-row flex gap-1 align-items-center">
          <i class="pi pi-exclamation-circle text-color-secondary" />
          <p class="text-color-secondary text-sm">
            To prevent abuse, you can only change your birthday roughly once every year
          </p>
        </div>
      </div>
      <div class="mt-5">
        <div class="flex justify-items-center align-items-center mt-2 gap-2">
          <Checkbox
            v-model="includeYear"
            inputId="include_year"
            name="include_year"
            value="Normal"
          />
          <label for="include_year">Include birth year?</label>
        </div>
        <div class="flex-row flex gap-1 align-items-center">
          <i class="pi pi-exclamation-circle text-color-secondary"></i>
          <p class="text-color-secondary text-sm">
            When your birthday is announced on a server, it will include your age
          </p>
        </div>
      </div>
    </Form>
  </div>
</template>
