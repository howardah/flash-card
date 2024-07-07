-- This file should undo anything in `up.sql`
ALTER TABLE `set` DROP COLUMN `updated_at`;
ALTER TABLE `set` DROP COLUMN `created_at`;
ALTER TABLE `set` ADD COLUMN `createdat` DATETIME NOT NULL;
ALTER TABLE `set` ADD COLUMN `updatedat` DATETIME NOT NULL;

ALTER TABLE `word` DROP COLUMN `created_at`;
ALTER TABLE `word` DROP COLUMN `updated_at`;
ALTER TABLE `word` DROP COLUMN `word_value`;
ALTER TABLE `word` ADD COLUMN `createdat` DATETIME NOT NULL;
ALTER TABLE `word` ADD COLUMN `updatedat` DATETIME NOT NULL;
ALTER TABLE `word` ADD COLUMN `word` VARCHAR(191) NOT NULL;

CREATE TABLE `wordset`(
	`wordId` INTEGER NOT NULL,
	`setId` INTEGER NOT NULL,
	PRIMARY KEY(`wordId`, `setId`),
	FOREIGN KEY (`wordId`) REFERENCES `Word`(`id`),
	FOREIGN KEY (`setId`) REFERENCES `Set`(`id`)
);

DROP TABLE IF EXISTS `_prisma_migrations`;
DROP TABLE IF EXISTS `word_set`;
