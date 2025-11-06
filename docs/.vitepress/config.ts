import { defineConfig, HeadConfig, resolveSiteDataByRoute } from "vitepress";
import llmstxt from "vitepress-plugin-llms";

export default defineConfig({
  srcDir: "docs",

  lastUpdated: true,
  cleanUrls: true,
  metaChunk: true,

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
        items: [
          { text: "Routes", link: "/explanations/routes" },
          {
            text: "Project Structure",
            link: "/explanations/project-structure",
          },
          { text: "Responses", link: "/explanations/responses" },
        ],
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
  vite: {
    plugins: [
      llmstxt({
        workDir: "docs",
        ignoreFiles: ["index.md"],
      }),
    ],
  },
  transformPageData: (pageData, ctx) => {
    const site = resolveSiteDataByRoute(
      ctx.siteConfig.site,
      pageData.relativePath,
    );
    const title = `${pageData.title || site.title} | ${pageData.description || site.description}`;
    ((pageData.frontmatter.head ??= []) as HeadConfig[]).push(
      ["meta", { property: "og:locale", content: site.lang }],
      ["meta", { property: "og:title", content: title }],
    );
  },
});
