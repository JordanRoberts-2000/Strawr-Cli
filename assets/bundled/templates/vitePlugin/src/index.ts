import type { Plugin } from "vite";

export default function vitePlugin(): Plugin {
  return {
    name: "vite-plugin",
    config() {
      console.log("hello world");
    },
  };
}
