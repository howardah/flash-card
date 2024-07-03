import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

// Remove all words
await prisma.word.deleteMany();

const addWord = async (word: string, gender: string, translation?: string) => {
    // create a new word
    await prisma.word.create({
        data: {
            word: word,
            gender: gender,
            translation: translation,
        },
    });
}

const words = [
    { word: "livre", gender: "Masculin", translation: "book" },
    { word: "pomme", gender: "Féminin", translation: "apple" },
    { word: "chien", gender: "Masculin", translation: "dog" },
    { word: "maison", gender: "Féminin", translation: "house" },
    { word: "chat", gender: "Masculin", translation: "cat" },
    { word: "voiture", gender: "Féminin", translation: "car" },
    { word: "stylo", gender: "Masculin", translation: "pen" },
    { word: "fleur", gender: "Féminin", translation: "flower" },
    { word: "jour", gender: "Masculin", translation: "day" },
    { word: "nuit", gender: "Féminin", translation: "night" },
    { word: "garçon", gender: "Masculin", translation: "boy" },
    { word: "fille", gender: "Féminin", translation: "girl" },
    { word: "père", gender: "Masculin", translation: "father" },
    { word: "mère", gender: "Féminin", translation: "mother" },
    { word: "frère", gender: "Masculin", translation: "brother" },
    { word: "sœur", gender: "Féminin", translation: "sister" },
    { word: "roi", gender: "Masculin", translation: "king" },
    { word: "reine", gender: "Féminin", translation: "queen" },
    { word: "soleil", gender: "Masculin", translation: "sun" },
    { word: "lune", gender: "Féminin", translation: "moon" },
    { word: "travail", gender: "Masculin", translation: "work" },
    { word: "fête", gender: "Féminin", translation: "party" },
    { word: "pain", gender: "Masculin", translation: "bread" },
    { word: "salade", gender: "Féminin", translation: "salad" },
    { word: "cheval", gender: "Masculin", translation: "horse" },
    { word: "vache", gender: "Féminin", translation: "cow" },
    { word: "poisson", gender: "Masculin", translation: "fish" },
    { word: "mer", gender: "Féminin", translation: "sea" },
    { word: "vent", gender: "Masculin", translation: "wind" },
    { word: "pluie", gender: "Féminin", translation: "rain" },
];

for (const word of words) {
    await addWord(word.word, word.gender, word.translation);
}

// count the number of words
const count = await prisma.word.count();
console.log(`There are ${count} words in the database.`);