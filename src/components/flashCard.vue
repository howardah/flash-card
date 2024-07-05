<template>
  <div
    @click="() => toggleView()"
    :class="{ [backgroundColor]: true, transitions }"
    class="flash-card p-4 border rounded shadow-md cursor-pointer aspect-square relative"
  >
    <div v-if="word" class="absolute inset-0 flex items-center justify-center pointer-events-none">
      <div>
        <p class="text-lg font-bold">
          <span
            class="border-b-2"
            :class="{ [articleBorderColor]: true }"
          >
            <span :class="{ 'opacity-0': !showGender }">{{ article }}</span>
          </span>
          {{ word.word }}
        </p>
        <p class="text-sm text-gray-500" :class="{ 'opacity-0': !showGender }">
          {{ word.gender }}
        </p>
      </div>
    </div>
    <div v-else class="absolute inset-0 flex items-center justify-center pointer-events-none">
      <p>Loading words...</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Word } from "@prisma/client";
import { computed, ref } from "vue";

const props = defineProps<{
  word?: Word;
}>();

const article = computed(() => {
  let article = "Le";
  if (props.word?.gender.toLowerCase() === "féminin") {
    article = "La";
  }

  // If word starts with a vowel or an accented vowel, use "l'"
  if (props.word?.word.match(/^h?[aàâeéèêiîoôuùûy]/i)) {
    article = "L'";
  }

  return article;
});

const showGender = ref(false);
const transitions = ref(true);

const toggleView = (value?: boolean, transition?: boolean) => {
  if (transition) transitions.value = false;
  
  showGender.value = value ?? !showGender.value;

  if (!transition) return;

  setTimeout(() => {
    transitions.value = true;
  }, 100);
};

const backgroundColor = computed(() => {
  if (!showGender.value) {
    return "bg-white";
  }
  return props.word?.gender.toLowerCase() === "masculin" ? "bg-blue-100" : "bg-pink-100";
});

const articleBorderColor = computed(() => {
  if (!showGender.value) {
    return "border-gray-500";
  }
  return props.word?.gender.toLowerCase() === "masculin" ? "border-blue-500" : "border-pink-500";
});

defineExpose({
  toggleView
});

</script>

<style scoped lang="scss">
.flash-card.transitions {
  &,
  div,
  span,
  p {
    transition: opacity 0.25s, border-color 0.25s, background-color 0.25s;
  }
}
</style>
