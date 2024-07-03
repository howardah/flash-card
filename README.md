# Nuxt 3 Flashcard App

This is a flashcard application built with Nuxt 3, leveraging Tailwind CSS for styling and Prisma for database management. The app allows users to navigate through flashcards, displaying words along with their gender and translation.

## Technologies Used

- **Nuxt 3**: A modern web framework for building fast and performant web applications.
- **Tailwind CSS**: A utility-first CSS framework for rapid UI development.
- **Prisma**: An ORM for Node.js and TypeScript that provides a type-safe database client.
- **Vue 3**: The progressive JavaScript framework used by Nuxt 3.

## Setup

Make sure to install the dependencies:

```bash
# npm
npm install

# pnpm
pnpm install

# yarn
yarn install

# bun
bun install
```

## Development Server

Start the development server on `http://localhost:3000`:

```bash
# npm
npm run dev

# pnpm
pnpm run dev

# yarn
yarn dev

# bun
bun run dev
```

## Production

Build the application for production:

```bash
# npm
npm run build

# pnpm
pnpm run build

# yarn
yarn build

# bun
bun run build
```

Locally preview production build:

```bash
# npm
npm run preview

# pnpm
pnpm run preview

# yarn
yarn preview

# bun
bun run preview
```

## Features

- **Flashcard Navigation**: Use the "Next Card" and "Previous Card" buttons to navigate through the flashcards.
- **Keyboard Shortcuts**: 
  - ArrowRight: Next Card
  - ArrowLeft: Previous Card
  - Space: Toggle View
- **Randomized Flashcards**: The app fetches and displays words randomly from the database.

## API Endpoints

- **Get All Words**: Fetches all words from the database.
- **Add Word**: Adds a new word to the database.
- **Get Single Word**: Fetches a single word, either randomly or based on the current word and navigation direction.

## Database

The app uses Prisma to manage the database, which is configured to use MySQL. The schema is defined in `prisma/schema.prisma`.

## Deployment

Check out the [deployment documentation](https://nuxt.com/docs/getting-started/deployment) for more information.

## Additional Resources

- [Nuxt 3 Documentation](https://nuxt.com/docs/getting-started/introduction)
- [Tailwind CSS Documentation](https://tailwindcss.com/docs)
- [Prisma Documentation](https://www.prisma.io/docs)