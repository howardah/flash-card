-- Your SQL goes here
ALTER TABLE `set` DROP COLUMN `createdAt`;
ALTER TABLE `set` DROP COLUMN `updatedAt`;
ALTER TABLE `set` ADD COLUMN `updated_at` TIMESTAMP NOT NULL;
ALTER TABLE `set` ADD COLUMN `created_at` TIMESTAMP NOT NULL;

ALTER TABLE `word` DROP COLUMN `createdAt`;
ALTER TABLE `word` DROP COLUMN `updatedAt`;
ALTER TABLE `word` DROP COLUMN `word`;
ALTER TABLE `word` ADD COLUMN `created_at` TIMESTAMP NOT NULL;
ALTER TABLE `word` ADD COLUMN `updated_at` TIMESTAMP NOT NULL;
ALTER TABLE `word` ADD COLUMN `word_value` VARCHAR NOT NULL;

DROP TABLE IF EXISTS `wordset`;
CREATE TABLE `_prisma_migrations`(
	`id` VARCHAR(36) NOT NULL PRIMARY KEY,
	`checksum` VARCHAR(64) NOT NULL,
	`finished_at` TIMESTAMP,
	`migration_name` VARCHAR(255) NOT NULL,
	`logs` TEXT,
	`rolled_back_at` TIMESTAMP,
	`started_at` TIMESTAMP NOT NULL,
	`applied_steps_count` INTEGER NOT NULL
);

CREATE TABLE `word_set`(
	`word_id` INTEGER NOT NULL,
	`set_id` INTEGER NOT NULL,
	PRIMARY KEY(`word_id`, `set_id`)
);

