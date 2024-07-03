import { PrismaClient } from "@prisma/client";

interface AddWordBody {
  word: string;
  gender: string;
  translation?: string;
}

export default defineEventHandler(async (event) => {
  const prisma = new PrismaClient();
  const body = await readBody(event) as AddWordBody;
  const word = await prisma.word.create({
    data: body,
  });
  return word;
});
