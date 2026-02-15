import { defineConfig } from "vitepress";

export default defineConfig({
  title: "pmon",
  description: "Your personal workflow monitor.",
  lang: "en-US",
  lastUpdated: true,
  head: [
    ["link", { rel: "icon", href: "/pmon-icon.svg" }],
    [
      "meta",
      {
        name: "theme-color",
        content: "#1b6a7a",
      },
    ],
  ],
  themeConfig: {
    logo: "/pmon-icon.svg",
    siteTitle: "pmon",
    nav: [
      { text: "Home", link: "/" },
      { text: "Downloads", link: "/downloads" },
    ],
    footer: {
      message: "Built for calm, confident monitoring.",
      copyright: "Copyright © 2026 pmon",
    },
  },
});
