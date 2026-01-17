<script setup lang="ts">
import MainLayout from '@/layouts/MainLayout.vue'
import { useSettingsStore } from '@/stores/settings'
import { onMounted } from 'vue'

const settingsStore = useSettingsStore()

onMounted(async () => {
  // Try to load settings and manage splashscreen transition
  try {
    await settingsStore.loadSettings()
  } catch (e) {
    console.error('Failed to initialize settings:', e)
  } finally {
    // Guaranteed fallback: Show the window if it's still hidden
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    await getCurrentWindow().show()
  }
})
</script>

<template>
  <MainLayout />
</template>

<style>
/* Global styles if needed */
body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}
</style>