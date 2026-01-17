<script setup lang="ts">
import {
  Sidebar,
  SidebarContent,
  SidebarGroup,
  SidebarGroupContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarRail,
  useSidebar,
} from '@/components/ui/sidebar'
import {
  History,
  Layers,
  LayoutGrid,
  Settings,
} from 'lucide-vue-next'
import { useRoute } from 'vue-router'
import XRestIcon from './XRestIcon.vue'

const route = useRoute()
const { state } = useSidebar()

const navItems = [
  {
    title: 'Services',
    url: '/services',
    icon: Layers,
  },
  {
    title: 'Collections',
    url: '/collections',
    icon: LayoutGrid,
  },
  {
    title: 'History',
    url: '/history',
    icon: History,
  },
  {
    title: 'Settings',
    url: '/settings',
    icon: Settings,
  },
]
</script>

<template>
  <Sidebar collapsible="icon">
    <SidebarHeader class="h-16 flex items-center px-4">
      <div class="flex items-center gap-2 font-bold text-xl overflow-hidden whitespace-nowrap">
        <div class="flex h-8 w-8 items-center justify-center rounded-lg bg-background text-primary shrink-0">
          <XRestIcon class="h-6 w-6" />
        </div>
        <span v-if="state === 'expanded'" class="truncate">xrest</span>
      </div>
    </SidebarHeader>


    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem v-for="item in navItems" :key="item.title">
              <SidebarMenuButton as-child :tooltip="item.title" :is-active="route.path === item.url">
                <router-link :to="item.url" class="flex items-center gap-2">
                  <component :is="item.icon" class="h-4 w-4" />
                  <span>{{ item.title }}</span>
                </router-link>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
    <SidebarRail />
  </Sidebar>
</template>
