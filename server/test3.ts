import { PrismaClient } from "@prisma/client";
import { beginner1, beginner2, beginner3 } from './dataSets';

const prisma = new PrismaClient();

const main = async () => {

    const sets = {
        "Beginner Set 1": new Set(beginner1),
        "Beginner Set 2": new Set(beginner2),
        "Beginner Set 3": new Set(beginner3),
    };

    for (const setEntry of Object.entries(sets)) {
        const setName = setEntry[0];
        const setWords = setEntry[1];

        console.log(`Processing set: ${setName}`);
        // Create a new set if it doesn't exist
        let set = await prisma.set.findFirst({
            where: {
                name: setName,
            },
        });

        console.log(`Set: ${set}`);
        
        if (!set) {
            console.log(`Set not found, creating new set: ${setName}`);
            set = await prisma.set.create({
                data: {
                name: setName,
                },
            });
        }
        if (!set) continue;

        const wordIds: number[] = [];

        for (const word of setWords) {
            const oldWord = await prisma.word.findFirst({
                where: {
                    word: word.word,
                    gender: word.gender,
                },
            });

            if (oldWord) {
                wordIds.push(oldWord.id);
                continue;
            }

            const newWord = await prisma.word.create({
                data: {
                    word: word.word,
                    gender: word.gender,
                    translation: word.translation,
                },
            });

            wordIds.push(newWord.id);
        }

        // Find words already in set and remove them from the wordIds array
        const wordsInSet = await prisma.wordSet.findMany({
            where: {
                setId: set.id,
                wordId: {
                    in: wordIds,
                },
            },
        });

        for (const word of wordsInSet) {
            wordIds.splice(wordIds.indexOf(word.wordId), 1);
        }

        console.log(`Word IDs: ${wordIds.length} vs ${setWords.size}`);
        
        // Ensure wordIds contains unique values
        const uniqueWordIds = [...new Set(wordIds)];

        // Print the difference between the two sets
        console.log(`Word IDs: ${wordIds.length} vs ${uniqueWordIds.length}`);

        await prisma.wordSet.createMany({
            data: uniqueWordIds.map((wordId) => ({
                wordId,
                setId: set.id,
            })),
        });

        const wordCount = await prisma.wordSet.count({
            where: {
                setId: set.id,
            },
        });
        console.log(`Number of words in the set (${setName}): ${wordCount}`);
    };

    // exit
    process.exit(0);
};

main();