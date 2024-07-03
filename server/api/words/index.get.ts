import { PrismaClient } from "@prisma/client";

export default defineEventHandler(async (event) => {
  const prisma = new PrismaClient();
  const count = await prisma.word.count();
  return `There are ${count} words in the database.`;
});
