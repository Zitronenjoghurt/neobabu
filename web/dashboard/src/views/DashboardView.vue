<script lang="ts" setup>
import { useGuildStore } from '@/stores/guild.ts'
import { onMounted } from 'vue'

const guildStore = useGuildStore()
onMounted(() => guildStore.ensureGuilds())
</script>

<template>
  <main></main>
</template>
