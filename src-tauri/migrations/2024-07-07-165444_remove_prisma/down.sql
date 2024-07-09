-- This file should undo anything in `up.sql`
CREATE TABLE `_prisma_migrations`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`checksum` TEXT NOT NULL,
	`finished_at` TIMESTAMP,
	`migration_name` TEXT NOT NULL,
	`logs` TEXT,
	`rolled_back_at` TIMESTAMP,
	`started_at` TIMESTAMP NOT NULL,
	`applied_steps_count` INTEGER NOT NULL
);




