use tauri_plugin_sql::{Migration, MigrationKind};

pub const INIT_UP: Migration = Migration {
    version: 1,
    description: "Initial migration",
    sql: include_str!("../migrations/2024-07-07-165110_init/up.sql"),
    kind: MigrationKind::Up,
};

pub const INIT_DOWN: Migration = Migration {
    version: 1,
    description: "Initial migration down",
    sql: include_str!("../migrations/2024-07-07-165110_init/down.sql"),
    kind: MigrationKind::Down,
};

pub const REMOVE_PRISMA_UP: Migration = Migration {
    version: 2,
    description: "Remove Prisma up",
    sql: include_str!("../migrations/2024-07-07-165444_remove_prisma/up.sql"),
    kind: MigrationKind::Up,
};

pub const REMOVE_PRISMA_DOWN: Migration = Migration {
    version: 2,
    description: "Remove Prisma down",
    sql: include_str!("../migrations/2024-07-07-165444_remove_prisma/down.sql"),
    kind: MigrationKind::Down,
};

pub const ADD_TEST_DATA_UP: Migration = Migration {
    version: 3,
    description: "Add test data up",
    sql: include_str!("../migrations/2024-07-09-083432_add_test_data/up.sql"),
    kind: MigrationKind::Up,
};

pub const ADD_TEST_DATA_DOWN: Migration = Migration {
    version: 3,
    description: "Add test data down",
    sql: include_str!("../migrations/2024-07-09-083432_add_test_data/down.sql"),
    kind: MigrationKind::Down,
};

pub const ADD_DEFAULTS_UP: Migration = Migration {
    version: 4,
    description: "Add defaults up",
    sql: include_str!("../migrations/2024-07-09-084652_add_defaults/up.sql"),
    kind: MigrationKind::Up,
};

pub const ADD_DEFAULTS_DOWN: Migration = Migration {
    version: 4,
    description: "Add defaults down",
    sql: include_str!("../migrations/2024-07-09-084652_add_defaults/down.sql"),
    kind: MigrationKind::Down,
};

pub const CONSTRAIN_WORDS_UP: Migration = Migration {
    version: 5,
    description: "Constrain words up",
    sql: include_str!("../migrations/2024-07-09-085853_constrain_words/up.sql"),
    kind: MigrationKind::Up,
};

pub const CONSTRAIN_WORDS_DOWN: Migration = Migration {
    version: 5,
    description: "Constrain words down",
    sql: include_str!("../migrations/2024-07-09-085853_constrain_words/down.sql"),
    kind: MigrationKind::Down,
};