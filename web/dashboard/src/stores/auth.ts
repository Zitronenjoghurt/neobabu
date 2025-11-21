import { defineStore } from 'pinia'
import { computed, ref } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const user = ref<{ name: string; email: string } | null>(null)
  const token = ref<string | null>(null)

  const isAuthenticated = computed(() => !!user.value)
  const userName = computed(() => user.value?.name ?? '')

  function login(credentials: { email: string; password: string }) {
    // TODO
    user.value = { name: 'John Doe', email: credentials.email }
    token.value = 'fake-jwt-token'
    localStorage.setItem('token', token.value)
  }

  function logout() {
    user.value = null
    token.value = null
    localStorage.removeItem('token')
  }

  function checkAuth() {
    const storedToken = localStorage.getItem('token')
    if (storedToken) {
      token.value = storedToken
      user.value = { name: 'John Doe', email: 'john@example.com' }
    }
  }

  return {
    user,
    isAuthenticated,
    userName,
    login,
    logout,
    checkAuth,
  }
})
