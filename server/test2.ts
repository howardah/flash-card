import { PrismaClient } from "@prisma/client";

const prisma = new PrismaClient();

// Create a new set
await prisma.set.create({
    data: {
        name: "Default",
    },
});

const words = await prisma.word.findMany();
const set = await prisma.set.findFirst({
    where: {
        name: "Default",
    },
});

if (!set) {
    throw new Error("Set not found");
}

// Add all existing words to the set
await prisma.wordSet.createMany({
    data: words.map((word) => ({
        wordId: word.id,
        setId: set.id,
    })),
});

// Print the number of words in the set
const wordCount = await prisma.wordSet.count({
    where: {
        setId: set.id,
    },
});
console.log(`Number of words in the set: ${wordCount}`);
 
// exit
process.exit(0);