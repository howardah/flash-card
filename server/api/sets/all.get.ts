import { PrismaClient } from "@prisma/client";

export default defineEventHandler(async (event) => {
  const prisma = new PrismaClient();
  const sets = await prisma.set.findMany();
  // We'll remove the default set from the list of sets for now
  return sets.filter((set) => set.name !== "Default");
});
