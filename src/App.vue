<template>
  <div>
    <div class="flex flex-wrap gap-4 m-auto justify-center bg-slate-200 p-4 lg:text-base md:text-sm text-xs">
      <button
        v-for="set in allSets"
        :key="set.id"
        @click="setId = set.id"
        class="px-4 py-2 bg-slate-300 text-slate-500 rounded hover:bg-slate-400 hover:text-slate-100 transition"
        :class="{ 'bg-slate-400 text-slate-100': setId === set.id }"
      >
        {{ set.name }}
      </button>
    </div>
    <div class="flex items-center justify-center min-h-screen">
      <div>
        <FlashCardSet :key="setId" :set="set" />
        <div class="grid grid-cols-2 p-4 gap-4 items-center justify-center" :class="{ 'input-container': isInputFocused }">
          <div class="col-span-2">
            <input 
              v-model="testWord" 
              @focus="isInputFocused = true" 
              @blur="isInputFocused = false" 
              class="px-4 py-2 border rounded shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500" 
            />
          </div>
          <button 
            @click="loadWords" 
            class="px-2 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 transition"
          >
            Save Word
          </button>
          <button 
            @click="getWord" 
            class="px-2 py-1 bg-green-500 text-white rounded hover:bg-green-600 transition"
          >
            Get Word
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { Set, Word } from "@prisma/client";
import { computed, ref } from "vue";
import { testDataSets } from "./assets/testData";
import FlashCardSet from "./components/flashCardSet.vue";
import { invoke } from "@tauri-apps/api/core";
import Database from "@tauri-apps/plugin-sql";

// const { data: allSets, pending: pendingSets } = await useAsyncData("allSets", () =>
//   $fetch<Set[]>("/api/sets/all")
// );

const allSets = ref<Set[]>(testDataSets);
const pendingSets = ref(false);
const testWord = ref("");
const setId = ref(0);
const isInputFocused = ref(false);

const loadWords = async () => {
  // invoke("save_word", { word: testWord.value });
  // console.log(testWord.value);
  // Use a proxy server to bypass CORS
  const proxyUrl = 'https://cors-anywhere.herokuapp.com/';
  const targetUrl = 'https://gender-flash-card.netlify.app/api/words/all';
  const allWords = await (await fetch(proxyUrl + targetUrl)).json() as Word[];
  
  const db = await Database.load('sqlite:flashcard.db');
  for (const word of allWords) {
    // Check if word exists
    const result = await db.select('SELECT * FROM word WHERE word = ?', [word.word]) as null | Word[];
    if (result?.length === 0) {
      await db.execute('INSERT INTO word (word, gender, created_at, updated_at) VALUES ($1, $2, $3, $4)', [word.word, word.gender, new Date(), new Date()]);
    } else {
      // Update the translation
      await db.execute('UPDATE word SET translation = ? WHERE word = ?', [word.translation, word.word]);
    }
  }
  // await db.execute('INSERT INTO word (word, gender, created_at, updated_at) VALUES ($1, $2, $3, $4)', [testWord.value, "Masculin", new Date(), new Date()]);
};

const getWord = async () => {
  const db = await Database.load('sqlite:flashcard.db');
  const result = await db.select('SELECT * FROM word');
  console.log(result);
};

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

@media (max-width: 768px) {
  .input-container {
    padding-bottom: 50vh; /* Adjust this value as needed */
  }
}
</style>
