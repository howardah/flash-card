import { Set, Word } from "@prisma/client";
import { beginner1, beginner2, beginner3, WordBase } from "./datasets";

export function wordBaseToWord(wordBase: WordBase, id: number): Word {
    return {
        id: id,
        word: wordBase.word,
        gender: wordBase.gender,
        translation: wordBase.translation,
        createdAt: new Date(),
        updatedAt: new Date(),
    }
}

export function getWordsFromSet(set: Set): Word[] {
    switch (set.name) {
        case "Beginner 1":
            return beginner1.map((word, index) => wordBaseToWord(word, index));
        case "Beginner 2":
            return beginner2.map((word, index) => wordBaseToWord(word, index));
        case "Beginner 3":
            return beginner3.map((word, index) => wordBaseToWord(word, index));
        default:
            return [];
    }
}

export const testDataSets: Set[] = [
    {
        id: 1,
        name: "Beginner 1",
        createdAt: new Date(),
        updatedAt: new Date(),
    },
    {
        id: 2,
        name: "Beginner 2",
        createdAt: new Date(),
        updatedAt: new Date(),
    },
    {
        id: 3,
        name: "Beginner 3",
        createdAt: new Date(),
        updatedAt: new Date(),
    }
];