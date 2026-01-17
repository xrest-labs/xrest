<script setup lang="ts">
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select'
import InterpolatedTextarea from './InterpolatedTextarea.vue'
import { computed } from 'vue';

const props = defineProps<{
  body: any
  variables: Record<string, string>
  environmentName: string
}>();

const body = computed(() => props.body);
const variables = computed(() => props.variables);
const environmentName = computed(() => props.environmentName);
</script>

<template>
  <div class="space-y-4">
    <div class="flex items-center gap-2">
      <span class="text-[11px] font-medium text-muted-foreground whitespace-nowrap">Content Type:</span>
      <Select v-model="body.type">
        <SelectTrigger class="w-48 h-7 text-[10px]">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value="application/json" class="text-[10px]">JSON (application/json)</SelectItem>
          <SelectItem value="application/xml" class="text-[10px]">XML (application/xml)</SelectItem>
          <SelectItem value="text/plain" class="text-[10px]">Text (text/plain)</SelectItem>
          <SelectItem value="application/x-www-form-urlencoded" class="text-[10px]">Form URL Encoded</SelectItem>
        </SelectContent>
      </Select>
    </div>
    <div class="relative group">
      <InterpolatedTextarea v-model="body.content" :variables="variables" :environment-name="environmentName"
        :language="body.type"
        class="w-full min-h-[150px] bg-muted/30 border rounded-md p-3 text-[9px] font-mono focus:outline-none focus:ring-1 focus:ring-primary resize-y"
        placeholder="Enter request body here..." />
      <div class="absolute bottom-2 right-2 opacity-0 group-hover:opacity-100 transition-opacity">
        <span class="text-[9px] px-1.5 py-0.5 rounded bg-background border text-muted-foreground">JSON</span>
      </div>
    </div>
  </div>
</template>
