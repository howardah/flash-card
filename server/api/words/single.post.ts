import { PrismaClient, Word } from "@prisma/client";

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
  const body = await readBody(event) as {word: Word | null, previous: boolean};

  // Get all ids
  const ids = await prisma.word.findMany({
    select: {
      id: true,
    },
  });

  let nextIdIndex = body.word?.id ? ids.findIndex((id) => id.id === body.word?.id) + (body.previous ? -1 : 1) : Math.floor(Math.random() * ids.length);
  if (nextIdIndex >= ids.length) nextIdIndex = 0;

  // Get a random id
  const nextId = ids[nextIdIndex]?.id;

  // get a random word
  const word = await prisma.word.findUnique({
    where: {
      id: nextId,
    },
  });

  return word || demoWord;
});
