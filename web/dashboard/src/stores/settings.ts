import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface UserSettings {
  birthday?: UserBirthdaySettings
}

export interface UserBirthdaySettings {
  day: number
  month: number
  year?: number
  updatable: boolean
}

export const useSettingsStore = defineStore('settings', () => {
  const settings = ref<UserSettings | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)

  const fetchSettings = async () => {
    if (loading.value) {
      return
    }

    loading.value = true
    error.value = null

    try {
      const response = await fetch('/api/me/settings', { credentials: 'include' })
      if (!response.ok) {
        throw new Error(`Failed to fetch settings: ${response.statusText}`)
      }

      const data = await response.json()
      settings.value = data.settings
      return
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'An unknown error occurred'
      return
    } finally {
      loading.value = false
    }
  }

  const saveBirthday = async (birthdayData: UserBirthdaySettings) => {
    loading.value = true
    try {
      await new Promise((resolve) => setTimeout(resolve, 1000))

      if (settings.value) {
        settings.value.birthday = birthdayData
      }

      return true
    } catch (err) {
      error.value = 'Failed to save settings'
      return false
    } finally {
      loading.value = false
    }
  }

  return {
    fetchSettings,
    saveBirthday,
    settings,
    error,
    loading,
  }
})
