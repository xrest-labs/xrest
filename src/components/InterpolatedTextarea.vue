<script setup lang="ts">
import { computed, ref, onMounted, watch, nextTick } from "vue";
import { parseInterpolation } from "@/lib/placeholders";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { cn } from "@/lib/utils";
import Prism from "prismjs";
import "prismjs/components/prism-json";

const props = defineProps<{
  modelValue: string;
  variables?: Record<string, string>;
  environmentName?: string;
  placeholder?: string;
  class?: string;
  language?: string;
}>();

const emit = defineEmits(["update:modelValue"]);

const textareaRef = ref<HTMLTextAreaElement | null>(null);
const highlightRef = ref<HTMLDivElement | null>(null);
const textRef = ref<HTMLDivElement | null>(null);
const hotspotRef = ref<HTMLDivElement | null>(null);

const tokens = computed(() =>
  parseInterpolation(props.modelValue || "", props.variables || {}),
);

const internalValue = computed({
  get: () => props.modelValue,
  set: (val) => emit("update:modelValue", val),
});

const syncScroll = () => {
  if (textareaRef.value) {
    const scrollTop = textareaRef.value.scrollTop;
    const scrollLeft = textareaRef.value.scrollLeft;

    if (highlightRef.value) {
      highlightRef.value.scrollTop = scrollTop;
      highlightRef.value.scrollLeft = scrollLeft;
    }
    if (textRef.value) {
      textRef.value.scrollTop = scrollTop;
      textRef.value.scrollLeft = scrollLeft;
    }
    if (hotspotRef.value) {
      hotspotRef.value.scrollTop = scrollTop;
      hotspotRef.value.scrollLeft = scrollLeft;
    }
  }
};

const handleHotspotClick = () => {
  textareaRef.value?.focus();
};

onMounted(() => {
  syncScroll();
});

watch(
  () => props.modelValue,
  () => {
    nextTick(syncScroll);
  },
);

const highlightedContent = computed(() => {
  const code = props.modelValue || "";
  const lang = props.language === "application/json" ? "json" : props.language;
  if (!lang) return null;
  const grammar = (Prism.languages as any)[lang];
  if (!grammar) return null;

  const variablePattern = {
    pattern: /\{\{.*?\}\}/,
    alias: "interpolated-variable",
  };

  // Use a local grammar so we don't pollute the global Prism state
  // We need to handle strings and properties specifically for JSON to allow variables inside them
  const grammarWithVariables = Prism.util.clone(grammar);

  const addVariableToTokens = (tokens: any) => {
    if (!tokens) return;
    const keys = ["string", "property", "attr-value", "url"];
    keys.forEach((key) => {
      if (tokens[key]) {
        if (typeof tokens[key] === "object" && !Array.isArray(tokens[key])) {
          if (!tokens[key].inside) tokens[key].inside = {};
          tokens[key].inside["interpolated-variable"] = variablePattern;
        } else if (Array.isArray(tokens[key])) {
          tokens[key].forEach((t: any) => {
            if (typeof t === "object") {
              if (!t.inside) t.inside = {};
              t.inside["interpolated-variable"] = variablePattern;
            }
          });
        } else {
          // It's just a regex, wrap it
          tokens[key] = {
            pattern: tokens[key],
            inside: {
              "interpolated-variable": variablePattern,
            },
          };
        }
      }
    });
  };

  addVariableToTokens(grammarWithVariables);

  // Also add it at the top level
  const finalGrammar = {
    "interpolated-variable": variablePattern,
    ...grammarWithVariables,
  };

  return Prism.highlight(code, finalGrammar, lang);
});
</script>

<template>
  <div
    class="relative w-full group overflow-hidden rounded-md border bg-background transition-shadow focus-within:ring-1 focus-within:ring-primary"
  >
    <!-- 1. Text Layer -->
    <div
      ref="textRef"
      class="absolute inset-0 pointer-events-none px-3 py-2 font-mono whitespace-pre-wrap break-all overflow-hidden"
      :style="{ fontSize: 'inherit' }"
    >
      <template v-if="highlightedContent">
        <div v-html="highlightedContent" class="prism-code-inline"></div>
      </template>
      <template v-else v-for="(token, i) in tokens" :key="i">
        <span v-if="token.type === 'text'" class="text-foreground">{{
          token.content
        }}</span>
        <span v-else class="text-transparent selection:text-transparent">{{
          token.content
        }}</span>
      </template>
    </div>

    <!-- 2. Highlight Layer -->
    <div
      ref="highlightRef"
      class="absolute inset-0 pointer-events-none px-3 py-2 font-mono whitespace-pre-wrap break-all overflow-hidden"
      :style="{ fontSize: 'inherit' }"
    >
      <template v-for="(token, i) in tokens" :key="i">
        <span v-if="token.type === 'text'" class="text-transparent">{{
          token.content
        }}</span>
        <span
          v-else
          :class="
            cn(
              'px-0.5 rounded font-bold transition-all mx-px',
              token.isValid
                ? 'text-primary bg-primary/20'
                : 'text-destructive bg-destructive/20 underline decoration-destructive/50 underline-offset-2',
            )
          "
        >
          {{ token.content }}
        </span>
      </template>
    </div>

    <!-- 3. Real Textarea -->
    <textarea
      ref="textareaRef"
      v-model="internalValue"
      :placeholder="placeholder"
      :class="
        cn(
          'w-full min-h-[150px] bg-transparent border-none p-2 focus:outline-none resize-y relative z-10 font-mono',
          'text-transparent caret-foreground selection:bg-primary/30',
          props.class,
        )
      "
      :style="{ fontSize: 'inherit' }"
      @scroll="syncScroll"
      @input="syncScroll"
      spellcheck="false"
      autocapitalize="off"
      autocorrect="off"
    ></textarea>

    <!-- 4. Hotspot Layer -->
    <div
      ref="hotspotRef"
      class="absolute inset-0 pointer-events-none px-3 py-2 font-mono whitespace-pre-wrap break-all overflow-hidden z-20"
      :style="{ fontSize: 'inherit' }"
    >
      <template v-for="(token, i) in tokens" :key="i">
        <span v-if="token.type === 'text'" class="text-transparent">{{
          token.content
        }}</span>
        <template v-else>
          <TooltipProvider :delay-duration="0">
            <Tooltip>
              <TooltipTrigger as-child>
                <span
                  class="px-0.5 mx-px rounded font-bold text-transparent pointer-events-auto cursor-help"
                  @mousedown.prevent="handleHotspotClick"
                >
                  {{ token.content }}
                </span>
              </TooltipTrigger>
              <TooltipContent
                side="top"
                class="max-w-xs break-all z-[100] shadow-xl border-primary/20"
              >
                <div class="space-y-1.5 p-0.5">
                  <div v-if="token.isValid" class="flex flex-col gap-1">
                    <div class="flex items-center justify-between gap-4">
                      <div class="flex items-center gap-2">
                        <div
                          class="h-1.5 w-1.5 rounded-full bg-primary animate-pulse"
                        ></div>
                        <span
                          class="uppercase font-bold tracking-wider text-muted-foreground"
                          >Active Value</span
                        >
                      </div>
                      <span
                        v-if="environmentName"
                        class="bg-muted px-1.5 py-0.5 rounded text-muted-foreground font-bold border border-border/50"
                        >{{ environmentName }}</span
                      >
                    </div>
                    <span
                      class="text-primary bg-primary/5 p-1.5 rounded border border-primary/10 break-all select-all"
                      >{{ token.resolvedValue }}</span
                    >
                  </div>
                  <div v-else class="flex flex-col gap-1 text-destructive">
                    <div class="flex items-center justify-between gap-4">
                      <div class="flex items-center gap-2">
                        <div
                          class="h-1.5 w-1.5 rounded-full bg-destructive animate-bounce"
                        ></div>
                        <span class="uppercase font-bold tracking-wider"
                          >Missing Variable</span
                        >
                      </div>
                      <span
                        v-if="environmentName"
                        class="bg-destructive/10 px-1.5 py-0.5 rounded text-destructive font-bold border border-destructive/20"
                        >{{ environmentName }}</span
                      >
                    </div>
                    <span class="leading-relaxed"
                      >The variable
                      <code class="font-bold">{{ token.content }}</code> is not
                      defined in the current environment.</span
                    >
                  </div>
                </div>
              </TooltipContent>
            </Tooltip>
          </TooltipProvider>
        </template>
      </template>
    </div>
  </div>
</template>

<style scoped>
textarea,
div {
  font-family:
    "Roboto Mono Variable", ui-monospace, SFMono-Regular, Menlo, Monaco,
    Consolas, "Liberation Mono", "Courier New", monospace;
  font-size: 12pt;
  letter-spacing: normal;
  line-height: normal;
}

div::-webkit-scrollbar {
  display: none;
}

div {
  -ms-overflow-style: none;
  scrollbar-width: none;
}

/* Prism inline styles for the editor */
:deep(.prism-code-inline) {
  color: hsl(var(--foreground));
}

:deep(.token.interpolated-variable) {
  color: transparent !important;
}

:deep(.token.comment),
:deep(.token.prolog),
:deep(.token.doctype),
:deep(.token.cdata) {
  color: #5c6370;
}

:deep(.token.punctuation) {
  color: #abb2bf;
}

:deep(.token.property),
:deep(.token.keyword),
:deep(.token.tag) {
  color: #e06c75;
}

:deep(.token.boolean),
:deep(.token.constant),
:deep(.token.number) {
  color: #d19a66;
}

:deep(.token.string),
:deep(.token.char),
:deep(.token.builtin),
:deep(.token.inserted),
:deep(.token.attr-value) {
  color: #98c379;
}

:deep(.token.operator),
:deep(.token.entity),
:deep(.token.url),
:deep(.token.function) {
  color: #61afef;
}

:deep(.token.regex),
:deep(.token.important),
:deep(.token.atrule) {
  color: #c678dd;
}
</style>
