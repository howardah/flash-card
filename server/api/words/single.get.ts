import { PrismaClient } from "@prisma/client";

const demoWord = {
    id: 0,
    word: "Homme",
    gender: "Masculin",
    translation: "Man",
    createdAt: new Date(),
    updatedAt: new Date(),
};

export default defineEventHandler(async (event) => {
  const prisma = new PrismaClient();
  
  // Get all ids
  const ids = await prisma.word.findMany({
    select: {
      id: true,
    },
  });

  // Get a random id
  const randomId = Math.floor(Math.random() * ids.length);

  // get a random word
  const word = await prisma.word.findUnique({
    where: {
      id: randomId,
    },
  });

  return word || demoWord;
});
