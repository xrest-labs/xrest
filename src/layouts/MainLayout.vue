<script setup lang="ts">
import { SidebarProvider, SidebarInset, SidebarTrigger } from '@/components/ui/sidebar'
import AppSidebar from '@/components/AppSidebar.vue'
import { Separator } from '@/components/ui/separator'
import { useRoute } from 'vue-router'
import { computed } from 'vue';
import 'vue-sonner/style.css';
import { Toaster } from '@/components/ui/sonner'

const route = useRoute()

const pageTitle = computed(() => {
  const path = route.path
  if (path === '/') return 'xrest'
  const segment = path.split('/').filter(Boolean).pop() || ''
  return segment.charAt(0).toUpperCase() + segment.slice(1)
})
</script>

<template>
  <SidebarProvider>
    <AppSidebar />
    <SidebarInset class="flex flex-col h-screen overflow-hidden">
      <header class="flex h-12 shrink-0 items-center justify-between border-b px-4">
        <div class="flex items-center gap-2">
          <SidebarTrigger class="-ml-1" />
          <Separator orientation="vertical" class="mr-2 h-4" />
          <h2 class="text-xs font-medium text-muted-foreground uppercase tracking-wider">{{ pageTitle }}</h2>
        </div>
      </header>
      
      <main class="flex-1 overflow-hidden">
        <router-view />
      </main>
    </SidebarInset>
    <Toaster />
  </SidebarProvider>
</template>
