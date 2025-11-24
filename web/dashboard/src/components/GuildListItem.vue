<script lang="ts" setup>
import type { Guild } from '@/stores/guild.ts'
import { computed } from 'vue'

const props = defineProps<{ guild: Guild | null; selected?: boolean }>()

const iconUrl = computed(() => {
  if (!props.guild?.icon_hash) return undefined

  const isAnimated = props.guild.icon_hash.startsWith('a_')
  const extension = isAnimated ? 'gif' : 'png'

  return `https://cdn.discordapp.com/icons/${props.guild.id}/${props.guild.icon_hash}.${extension}`
})
</script>

<template>
  <div
    :class="selected ? 'surface-100' : 'surface-card shadow-5 hover:surface-50'"
    class="border-round p-2 transition-colors"
  >
    <div v-if="guild" class="flex align-items-center gap-3">
      <Avatar v-if="guild.icon_hash" :image="iconUrl" shape="circle" size="small" />
      <Avatar
        v-else
        :label="guild.name.charAt(0).toUpperCase()"
        class="bg-primary text-primary-contrast"
        shape="circle"
        size="small"
      />

      <span
        class="text-m font-semibold select-none white-space-nowrap overflow-hidden text-overflow-ellipsis flex-1"
        >{{ guild.name }}</span
      >
    </div>

    <div v-else class="flex align-items-center gap-3">
      <Skeleton shape="circle" size="2rem" />
      <Skeleton height="1rem" width="10rem" />
    </div>
  </div>
</template>
