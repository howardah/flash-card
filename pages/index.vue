<template>
    <div class="w-full max-w-xs m-auto p-4">
        <FlashCard ref="flashCard" v-if="formattedWord" :word="formattedWord" />
        <div class="grid grid-cols-2 gap-4">
            <button 
                v-if="formattedWord" 
                @click="loadPreviousWord" 
                class="mt-4 p-2 bg-slate-500 text-white rounded w-full"
            >
                Previous Card
            </button>
            <button 
                v-if="formattedWord" 
                @click="loadNextWord" 
                class="mt-4 p-2 bg-slate-500 text-white rounded w-full"
            >
                Next Card
            </button>
        </div>
    </div>
</template>

<script setup lang="ts">
import { onMounted, onBeforeUnmount, ref } from 'vue';
import type { Word } from '@prisma/client';
import FlashCard from '@/components/flashCard.vue';

interface APIWord {
    id: number;
    word: string;
    gender: string;
    translation: string | null;
    createdAt: string;
    updatedAt: string;
}

const { data: word } = await useFetch('/api/words/single')

const flashCard = ref<typeof FlashCard | null>(null);

const formatWord = (word: Word | APIWord | null) => {
    if (!word) {
        return null;
    }
    return {
        ...word,
        createdAt: new Date(word.createdAt),
        updatedAt: new Date(word.updatedAt)
    };
};

// Convert string dates to Date objects
const formattedWord = ref(formatWord(word.value));

const loadNewWord = async (previous?: boolean) => {
  const { data: newWord } = await useFetch('/api/words/single', {
    method: 'POST',
    body: JSON.stringify({word: formattedWord.value, previous: previous}),
  });
  formattedWord.value = formatWord(newWord.value);
  flashCard.value?.toggleView(false, true);
};

const loadNextWord = async () => {
    await loadNewWord(false);
};

const loadPreviousWord = async () => {
    await loadNewWord(true);
};

// Handle keydown events to navigate through cards
// ArrowRight: Next Card
// ArrowLeft: Previous Card
// Space: Toggle View
const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === 'ArrowRight') {
        loadNextWord();
    } else if (event.key === 'ArrowLeft') {
        loadPreviousWord();
    } else if (event.key === ' ') {
        flashCard.value?.toggleView();
    }
};

onMounted(() => {
    window.addEventListener('keydown', handleKeydown);
});

onBeforeUnmount(() => {
    window.removeEventListener('keydown', handleKeydown);
});
</script>

<style scoped>

</style>