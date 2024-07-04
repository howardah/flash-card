import { PrismaClient } from "@prisma/client";

export default defineEventHandler(async (event) => {
  const prisma = new PrismaClient();
  const setId = getRouterParam(event, "setId");
  const wordIds = (await prisma.wordSet.findMany({
    where: {
      setId: Number(setId),
    },
  })).map((word) => word.wordId);

  const words = await prisma.word.findMany({
    where: {
      id: { in: wordIds },
    },
  });
  
  // Randomize the order of the words
  return words.sort(() => Math.random() - 0.5);
});
