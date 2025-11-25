import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useCsrfStore = defineStore('csrf', () => {
  const token = ref<string | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const lastFetch = ref<number | null>(null)

  const CACHE_DURATION = 50 * 60 * 1000
  const isStale = computed(() => {
    if (!lastFetch.value) return true
    return Date.now() - lastFetch.value > CACHE_DURATION
  })

  const fetchCsrf = async (force: boolean = false) => {
    if (loading.value || !force || (token.value && !isStale.value)) {
      return
    }

    loading.value = true
    error.value = null

    try {
      const response = await fetch('/api/csrf', { credentials: 'include' })
      if (!response.ok) {
        throw new Error(`Failed to fetch csrf token: ${response.statusText}`)
      }

      const data = await response.json()
      token.value = data.csrf_token
      lastFetch.value = Date.now()
      return
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'An unknown error occurred'
      return
    } finally {
      loading.value = false
    }
  }

  const ensureCsrf = async () => {
    if (!token.value || isStale.value) {
      await fetchCsrf(true)
    }
    return
  }

  return {
    token,
    ensureCsrf,
    error,
    loading,
  }
})
