<template>
    <div class="w-full max-w-xs m-auto p-4">
      <FlashCard ref="flashCard" :word="!pending ? formattedWord : undefined" />
      <div class="grid grid-cols-2 gap-4">
        <button
          @click="loadPreviousWord"
          class="mt-4 p-2 bg-slate-500 text-white rounded w-full"
        >
          Previous Card
        </button>
        <button
          @click="loadNextWord"
          class="mt-4 p-2 bg-slate-500 text-white rounded w-full"
        >
          Next Card
        </button>
      </div>
    </div>
  </template>
  
  <script setup lang="ts">
  import { onMounted, onBeforeUnmount, ref } from "vue";
  import type { Set, Word } from "@prisma/client";
  import FlashCard from "@/components/flashCard.vue";
  
  interface APIWord {
    id: number;
    word: string;
    gender: string;
    translation: string | null;
    createdAt: string;
    updatedAt: string;
  }

  const props = defineProps<{
    set?: Set;
  }>();
  
  const flashCard = ref<typeof FlashCard | null>(null);
  const words = ref<Word[]>([]);
  const currentIndex = ref(0);
  const formattedWord = ref<Word | undefined>(undefined);
  
  const formatWord = (word: Word | APIWord): Word => {
    return {
      ...word,
      createdAt: new Date(word.createdAt),
      updatedAt: new Date(word.updatedAt),
    };
  };
  
  // Fetch all words using useAsyncData
  const { data: allWords, pending } = await useAsyncData("allWords", () =>
    $fetch<APIWord[]>(props.set ? `/api/words/set/${props.set.id}` : "/api/words/set")
  );
  
  const initFirstWord = () => {
    if (!pending.value && allWords.value) {
      words.value = allWords.value
        .filter((word: APIWord) => word !== null)
        .map((word: APIWord) => formatWord(word)) as Word[];
      const randomIndex = Math.floor(Math.random() * words.value.length);
      currentIndex.value = randomIndex;
      formattedWord.value = words.value[randomIndex];
      return;
    }
  
    setTimeout(() => {
      initFirstWord();
    }, 200);
  };
  
  const loadNewWord = (previous?: boolean) => {
    if (previous) {
      currentIndex.value =
        (currentIndex.value - 1 + words.value.length) % words.value.length;
    } else {
      currentIndex.value = (currentIndex.value + 1) % words.value.length;
    }
    formattedWord.value = words.value[currentIndex.value];
    flashCard.value?.toggleView(false, true);
  };
  
  const loadNextWord = () => {
    loadNewWord(false);
  };
  
  const loadPreviousWord = () => {
    loadNewWord(true);
  };
  
  // Handle keydown events to navigate through cards
  // ArrowRight: Next Card
  // ArrowLeft: Previous Card
  // Space: Toggle View
  const handleKeydown = (event: KeyboardEvent) => {
    if (event.key === "ArrowRight") {
      loadNextWord();
    } else if (event.key === "ArrowLeft") {
      loadPreviousWord();
    } else if (event.key === " ") {
      flashCard.value?.toggleView();
    }
  };
  
  onMounted(() => {
    initFirstWord();
    window.addEventListener("keydown", handleKeydown);
  });
  
  onBeforeUnmount(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
  </script>
  
  <style scoped></style>
  