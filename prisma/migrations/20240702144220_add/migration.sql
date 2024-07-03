/*
  Warnings:

  - Added the required column `updatedAt` to the `Word` table without a default value. This is not possible if the table is not empty.

*/
-- RedefineTables
PRAGMA defer_foreign_keys=ON;
PRAGMA foreign_keys=OFF;
CREATE TABLE "new_Word" (
    "id" INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    "word" TEXT NOT NULL,
    "gender" TEXT NOT NULL,
    "translation" TEXT,
    "createdAt" DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "updatedAt" DATETIME NOT NULL
);
INSERT INTO "new_Word" ("gender", "id", "word") SELECT "gender", "id", "word" FROM "Word";
DROP TABLE "Word";
ALTER TABLE "new_Word" RENAME TO "Word";
CREATE UNIQUE INDEX "Word_word_key" ON "Word"("word");
CREATE UNIQUE INDEX "Word_gender_key" ON "Word"("gender");
PRAGMA foreign_keys=ON;
PRAGMA defer_foreign_keys=OFF;
