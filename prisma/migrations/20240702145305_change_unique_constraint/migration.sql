/*
  Warnings:

  - A unique constraint covering the columns `[word,gender]` on the table `Word` will be added. If there are existing duplicate values, this will fail.

*/
-- DropIndex
DROP INDEX "Word_gender_key";

-- DropIndex
DROP INDEX "Word_word_key";

-- CreateIndex
CREATE UNIQUE INDEX "Word_word_gender_key" ON "Word"("word", "gender");
