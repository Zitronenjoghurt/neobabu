<script lang="ts" setup>
import { useGuildStore } from '@/stores/guild.ts'
import { onMounted } from 'vue'
import GuildListItem from '@/components/GuildListItem.vue'

const guildStore = useGuildStore()
onMounted(() => guildStore.ensureGuilds())
</script>

<template>
  <div
    class="guild-list-wrapper flex flex-1 flex-column overflow-y-auto p-2 surface-card surface-border border-1 border-round-left"
  >
    <template v-if="guildStore.loading">
      <GuildListItem v-for="i in 20" :key="i" :guild="null" class="mb-2 cursor-pointer" />
    </template>

    <template v-else-if="guildStore.sortedGuilds && guildStore.sortedGuilds.length > 0">
      <GuildListItem
        v-for="guild in guildStore.sortedGuilds"
        :key="guild.id"
        :guild="guild"
        :selected="guildStore.selectedGuild?.id === guild.id"
        class="mb-2 cursor-pointer"
        @click="guildStore.selectGuild(guild)"
      />
    </template>

    <div v-else class="p-2 flex flex-1 flex-column align-items-center justify-content-center">
      <p class="text-center text-color-secondary font-bold mb-4">No servers found</p>
      <p class="text-center text-color-secondary">
        Make sure there is at least one server you share with this bot or you got manage server
        permisions on
      </p>
    </div>
  </div>
</template>

<style scoped>
.guild-list-wrapper {
  scrollbar-gutter: stable;
}
</style>
