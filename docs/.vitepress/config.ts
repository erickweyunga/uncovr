import { defineConfig } from "vitepress";

export default defineConfig({
  srcDir: "docs",

  title: "Uncovr",
  description:
    "A modular microbackend framework for building type-safe REST APIs with automatic documentation and minimal boilerplate",

  themeConfig: {
    nav: [
      { text: "Get Started", link: "/get-started" },
      { text: "Tutorials", link: "/tutorials/by-example" },
      { text: "Explanations", link: "/explanations/routes" },
    ],

    sidebar: [
      {
        text: "Sections",
        items: [
          // { text: "Get Started", link: "/get-started/index" },
          { text: "Installation", link: "/get-started/installation" },
        ],
      },
      {
        text: "Tutorials",
        items: [
          { text: "Uncovr By Example", link: "/tutorials/by-example" },
          { text: "API Dev Quickstart", link: "/tutorials/quickstart" },
        ],
      },
      {
        text: "Explanations",
        items: [{ text: "Routes", link: "/explanations/routes" }],
      },
    ],

    socialLinks: [
      { icon: "github", link: "https://github.com/erickweyunga/uncovr" },
    ],

    footer: {
      message: "Released under the MIT License.",
      copyright: "Copyright © 2024-present Erick Weyunga",
    },

    search: {
      provider: "local",
    },
  },

  head: [
    ["link", { rel: "icon", href: "/favicon.ico" }],
    ["meta", { name: "theme-color", content: "#c96442" }],
    ["meta", { name: "og:type", content: "website" }],
    ["meta", { name: "og:title", content: "Uncovr" }],
    [
      "meta",
      {
        name: "og:description",
        content:
          "A modular microbackend framework for building type-safe REST APIs",
      },
    ],
  ],
});
