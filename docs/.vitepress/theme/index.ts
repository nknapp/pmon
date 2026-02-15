import DefaultTheme from "vitepress/theme";
import type { Theme } from "vitepress";
import DownloadList from "./components/DownloadList.vue";
import "./custom.css";

export default {
  extends: DefaultTheme,
  enhanceApp({ app }) {
    DefaultTheme.enhanceApp?.({ app });
    app.component("DownloadList", DownloadList);
  },
} satisfies Theme;
