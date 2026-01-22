<script setup lang="ts">
import { Layers, AlertCircle, RefreshCw } from "lucide-vue-next";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import { computed } from "vue";

const props = defineProps<{
  versions: any[];
}>();

const versions = computed(() => props.versions);

const emit = defineEmits<{
  (e: "restore", version: any): void;
}>();

const getMethodColor = (method: string) => {
  switch (method?.toUpperCase()) {
    case "GET":
      return "text-green-600";
    case "POST":
      return "text-orange-600";
    case "PUT":
      return "text-blue-600";
    case "DELETE":
      return "text-red-600";
    case "PATCH":
      return "text-purple-600";
    default:
      return "text-muted-foreground";
  }
};
</script>

<template>
  <div class="space-y-4 animate-in fade-in slide-in-from-bottom-2 duration-300">
    <div class="flex items-center justify-between border-b pb-2">
      <div class="flex items-center gap-2">
        <Layers class="h-4 w-4 text-primary" />
        <h3 class="font-semibold uppercase tracking-wider">
          Request Version History
        </h3>
      </div>
      <span class="text-muted-foreground bg-muted px-2 py-0.5 rounded-full">
        {{ versions?.length || 0 }} Versions
      </span>
    </div>

    <div
      v-if="!versions || versions.length === 0"
      class="flex flex-col items-center justify-center py-12 text-muted-foreground space-y-2"
    >
      <AlertCircle class="h-8 w-8 opacity-20" />
      <p class="">No version history available for this endpoint yet.</p>
      <p class="opacity-70">
        New versions are created automatically when you save changes.
      </p>
    </div>

    <div v-else class="border rounded-md overflow-hidden bg-card">
      <Table>
        <TableHeader>
          <TableRow class="hover:bg-transparent border-b">
            <TableHead class="font-bold h-8 w-16">Ver</TableHead>
            <TableHead class="font-bold h-8">Last Updated</TableHead>
            <TableHead class="font-bold h-8">Request Summary</TableHead>
            <TableHead class="font-bold h-8 w-20 text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          <TableRow
            v-for="v in [...versions].reverse()"
            :key="v.version"
            class="group h-10"
          >
            <TableCell class="font-bold">{{ v.version }}</TableCell>
            <TableCell class="text-muted-foreground whitespace-nowrap">
              {{
                v.lastUpdated
                  ? new Date(v.lastUpdated * 1000).toLocaleString()
                  : "Unknown"
              }}
            </TableCell>
            <TableCell>
              <div class="flex items-center gap-2">
                <span
                  :class="[
                    ' font-bold px-1.5 py-0.5 rounded bg-muted',
                    getMethodColor(v.config.method),
                  ]"
                >
                  {{ v.config.method }}
                </span>
                <span class="truncate max-w-[200px]" :title="v.config.url">
                  {{ v.config.url }}
                </span>
              </div>
            </TableCell>
            <TableCell class="text-right">
              <Button
                variant="ghost"
                size="sm"
                class="h-7 px-2 gap-1 hover:text-primary"
                @click="emit('restore', v)"
              >
                <RefreshCw class="h-3 w-3" /> Restore
              </Button>
            </TableCell>
          </TableRow>
        </TableBody>
      </Table>
    </div>
  </div>
</template>
