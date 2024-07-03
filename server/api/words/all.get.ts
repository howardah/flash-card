import { PrismaClient } from "@prisma/client";

export default defineEventHandler(async (event) => {
  const prisma = new PrismaClient();
  const words = await prisma.word.findMany();
  // Randomize the words before returning
  return words.sort(() => Math.random() - 0.5);
});
