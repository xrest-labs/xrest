<script setup lang="ts">
import { computed } from "vue";
import Prism from "prismjs";
import "prismjs/components/prism-json";
import { cn } from "@/lib/utils";

const props = defineProps<{
  code: string;
  language?: string;
  class?: string;
}>();

const highlightedCode = computed(() => {
  if (!props.code) return "";
  try {
    const lang = props.language || "json";
    const grammar = Prism.languages[lang];
    if (!grammar) return props.code;

    // Attempt to format JSON if it's not already formatted
    let codeToHighlight = props.code;
    if (lang === "json") {
      try {
        const parsed = JSON.parse(props.code);
        codeToHighlight = JSON.stringify(parsed, null, 2);
      } catch (e) {
        // Not valid JSON, just highlight as is
      }
    }

    return Prism.highlight(codeToHighlight, grammar, lang);
  } catch (error) {
    console.error("Highlighting error:", error);
    return props.code;
  }
});
</script>

<template>
  <pre
    :class="
      cn(
        'prism-code font-mono whitespace-pre p-3 rounded-md bg-background shadow-sm leading-relaxed overflow-auto border',
        props.class,
      )
    "
  ><code v-html="highlightedCode"></code></pre>
</template>

<style>
/**
 * One Dark theme for Prism.js
 * Based on Atom's One Dark theme
 */
.prism-code {
  color: hsl(var(--foreground));
  text-shadow: 0 1px rgba(0, 0, 0, 0.3);
  direction: ltr;
  text-align: left;
  white-space: pre;
  word-spacing: normal;
  word-break: normal;
  line-height: 1.5;
  tab-size: 2;
  hyphens: none;
}

.token.comment,
.token.prolog,
.token.doctype,
.token.cdata {
  color: #5c6370;
}

.token.punctuation {
  color: #abb2bf;
}

.token.namespace {
  opacity: 0.7;
}

.token.property,
.token.keyword,
.token.tag {
  color: #e06c75;
}

.token.class-name {
  color: #e5c07b;
  text-decoration: underline;
}

.token.boolean,
.token.constant {
  color: #d19a66;
}

.token.symbol,
.token.deleted {
  color: #e06c75;
}

.token.number {
  color: #d19a66;
}

.token.selector,
.token.attr-name,
.token.string,
.token.char,
.token.builtin,
.token.inserted {
  color: #98c379;
}

.token.variable {
  color: #e06c75;
}

.token.operator {
  color: #56b6c2;
}

.token.entity,
.token.url {
  color: #61afef;
}

.token.attr-value,
.token.atrule {
  color: #98c379;
}

.token.function {
  color: #61afef;
}

.token.regex,
.token.important {
  color: #c678dd;
}

.token.important,
.token.bold {
  font-weight: bold;
}

.token.italic {
  font-style: italic;
}

.token.entity {
  cursor: help;
}

/* Adjust colors for dark mode if needed using shadcn vars */
.dark .token.string {
  color: #98c379;
}

.dark .token.number {
  color: #d19a66;
}

.dark .token.boolean {
  color: #d19a66;
}

.dark .token.property {
  color: #e06c75;
}

.dark .token.punctuation {
  color: #abb2bf;
}

/* JSON specific */
.token.property {
  color: #e06c75;
}

.token.string {
  color: #98c379;
}

.token.number {
  color: #d19a66;
}

.token.boolean {
  color: #56b6c2;
}

.token.null {
  color: #56b6c2;
}
</style>
