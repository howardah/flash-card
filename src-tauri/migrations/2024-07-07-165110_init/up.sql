-- Your SQL goes here
CREATE TABLE `word`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`word` VARCHAR NOT NULL,
	`gender` VARCHAR(191) NOT NULL,
	`translation` VARCHAR(191),
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL
);

CREATE TABLE `word_set`(
	`word_id` INTEGER NOT NULL,
	`set_id` INTEGER NOT NULL,
	PRIMARY KEY(`word_id`, `set_id`)
);

CREATE TABLE `set`(
	`id` INTEGER NOT NULL PRIMARY KEY,
	`name` VARCHAR(191) NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL
);

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

