import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export interface Guild {
  id: string
  name: string
  icon_hash?: string
  has_bot: boolean
  is_active: boolean
  can_add_bot: boolean
}

export const useGuildStore = defineStore('guilds', () => {
  const guilds = ref<Guild[] | null>(null)
  const selectedGuild = ref<Guild | null>(null)
  const loading = ref(false)
  const error = ref<string | null>(null)
  const lastFetch = ref<number | null>(null)

  const CACHE_DURATION = 5 * 60 * 1000
  const isStale = computed(() => {
    if (!lastFetch.value) return true
    return Date.now() - lastFetch.value > CACHE_DURATION
  })

  const sortedGuilds = computed(() => {
    if (!guilds.value) return null

    return [...guilds.value].sort((a, b) => {
      const aScore = [a.has_bot, a.is_active, a.can_add_bot].filter(Boolean).length
      const bScore = [b.has_bot, b.is_active, b.can_add_bot].filter(Boolean).length

      if (bScore !== aScore) {
        return bScore - aScore
      }

      return a.name.localeCompare(b.name)
    })
  })

  const fetchGuilds = async (force = false) => {
    if (loading.value || (!force && guilds.value && !isStale.value)) {
      return
    }

    loading.value = true
    error.value = null

    try {
      const response = await fetch('/api/guilds', { credentials: 'include' })
      if (!response.ok) {
        throw new Error(`Failed to fetch guilds: ${response.statusText}`)
      }

      const data = await response.json()
      guilds.value = data.guilds
      lastFetch.value = Date.now()
      return
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'An unknown error occurred'
      return
    } finally {
      loading.value = false
    }
  }

  const ensureGuilds = async () => {
    if (!guilds.value || isStale.value) {
      await fetchGuilds(true)
    }
    return
  }

  const selectGuild = (guild: Guild) => {
    if (guild === selectedGuild.value) {
      selectedGuild.value = null
    } else {
      selectedGuild.value = guild
    }
  }

  return {
    guilds,
    sortedGuilds,
    error,
    loading,
    ensureGuilds,
    selectGuild,
    selectedGuild,
  }
})
