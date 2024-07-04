<template>
  <div>
    <div class="flex flex-wrap gap-4 m-auto justify-center bg-slate-200 p-4 lg:text-base md:text-sm text-xs">
      <button
        v-for="set in allSets"
        :key="set.id"
        @click="setId = set.id"
        class="px-4 py-2 bg-slate-300 text-slate-500 rounded hover:bg-slate-400 hover:text-slate-100 transition"
      >
        {{ set.name }}
      </button>
    </div>
    <div class="flex items-center justify-center min-h-screen">
      <FlashCardSet :key="setId" :set="set" />
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Set } from "@prisma/client";

const { data: allSets, pending: pendingSets } = await useAsyncData("allSets", () =>
  $fetch<Set[]>("/api/sets/all")
);

const setId = ref(0);

const set = computed(() => {
  if (pendingSets.value) return undefined;

  const selected = allSets.value?.find((set) => set.id === setId.value);
  if (!selected && allSets.value) {
    return allSets.value[0];
  }
  return selected;
});
</script>

<style scoped>
.min-h-screen {
  min-height: calc(100vh - 10rem);
}
</style>
