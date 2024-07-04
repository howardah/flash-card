// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: ["@nuxtjs/tailwindcss", "@nuxtjs/google-fonts"],

  googleFonts: {
    families: {
      Montserrat: [100, 200, 400, 600, 800],
    },
    subsets: ["latin"],
    display: "swap",
  },
})