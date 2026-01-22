<script setup lang="ts">
import { SidebarInset } from "@/components/ui/sidebar";
import AppSidebar from "@/components/AppSidebar.vue";
import "vue-sonner/style.css";
import { Toaster } from "@/components/ui/sonner";
</script>

<template>
  <AppSidebar />
  <SidebarInset class="flex flex-col overflow-hidden" style="height: 100vh; padding-top: 40px;">
    <main class="flex-1 overflow-hidden">
      <router-view />
    </main>
  </SidebarInset>
  <Toaster />
</template>
