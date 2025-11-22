import { defineStore } from 'pinia'
import { ref } from 'vue'

interface User {
  id: string
  username?: string
  avatar_hash?: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const loading = ref(false)

  async function fetchUser() {
    loading.value = true
    try {
      const response = await fetch('/api/me', { credentials: 'include' })
      if (response.ok) {
        user.value = await response.json()
      } else {
        user.value = null
      }
    } catch (error) {
      user.value = null
    } finally {
      loading.value = false
    }
  }

  function login() {
    window.location.href = '/api/auth/login'
  }

  async function logout() {
    await fetch('/api/auth/logout', {
      credentials: 'include',
      method: 'GET',
    })
    user.value = null
    window.location.href = '/'
  }

  return {
    user,
    loading,
    fetchUser,
    login,
    logout,
  }
})
