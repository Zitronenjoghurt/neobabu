<script lang="ts" setup>
import { useGuildStore } from '@/stores/guild.ts'
import { onMounted } from 'vue'
import GuildListItem from '@/components/GuildListItem.vue'

const guildStore = useGuildStore()
onMounted(() => guildStore.ensureGuilds())
</script>

<template>
  <div
    class="guild-list-wrapper flex-1 overflow-y-auto p-2 surface-card surface-border border-1 border-round-left"
  >
    <template v-if="guildStore.loading">
      <GuildListItem v-for="i in 20" :key="i" :guild="null" class="mb-2 cursor-pointer" />
    </template>

    <template v-else>
      <GuildListItem
        v-for="guild in guildStore.sortedGuilds"
        :key="guild.id"
        :guild="guild"
        :selected="guildStore.selectedGuild?.id === guild.id"
        class="mb-2 cursor-pointer"
        @click="guildStore.selectGuild(guild)"
      />
    </template>
  </div>
</template>

<style scoped>
.guild-list-wrapper {
  scrollbar-gutter: stable;
}
</style>
