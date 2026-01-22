<script setup lang="ts">
import { SidebarProvider } from "@/components/ui/sidebar";
import MainLayout from "@/layouts/MainLayout.vue";
import { useSettingsStore } from "@/stores/settings";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { onMounted } from "vue";

const settingsStore = useSettingsStore();

const handleDrag = async (event: MouseEvent) => {
  // Don't start drag if clicking an interactive element
  const target = event.target as HTMLElement;
  if (target.closest('button') || target.closest('a') || target.closest('input')) {
    return;
  }

  // Only start drag on left mouse button and single click
  // This allows double-click to bubble up correctly
  if (event.button === 0 && event.detail === 1) {
    await getCurrentWindow().startDragging();
  }
};

const handleDoubleClick = async () => {
  const win = getCurrentWindow();
  if (await win.isMaximized()) {
    await win.unmaximize();
  } else {
    await win.maximize();
  }
};

onMounted(async () => {
  // Try to load settings and manage splashscreen transition
  try {
    await settingsStore.loadSettings();
  } catch (e) {
    console.error("Failed to initialize settings:", e);
  } finally {
    // Guaranteed fallback: Show the window if it's still hidden
    await getCurrentWindow().show();
  }
});
</script>

<template>
  <SidebarProvider>
    <div class="app-container">
      <div class="titlebar" data-tauri-drag-region @mousedown="handleDrag" @dblclick="handleDoubleClick">
        <!-- <SidebarTrigger class="ml-[72px] w-9 h-9 pointer-events-auto" /> -->
      </div>
      <MainLayout />
    </div>
  </SidebarProvider>
</template>

<style scoped>
.app-container {
  width: 100%;
  height: 100vh;
  display: flex;
  flex-direction: column;
  position: relative;
}

.titlebar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 40px;
  z-index: 9999;
  cursor: default;
  -webkit-user-select: none;
  user-select: none;
  display: flex;
  align-items: center;
  padding: 0 4px;
}
</style>

<style>
/* Global styles if needed */
body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>
