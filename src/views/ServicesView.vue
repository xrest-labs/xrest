<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
import { toast } from 'vue-sonner'
import { invoke } from '@tauri-apps/api/core'
import { useServicesStore } from '@/stores/services'
import { useCollectionsStore } from '@/stores/collections'
import { open, ask } from '@tauri-apps/plugin-dialog'
import { Search, Plus, X, Layers, PlusCircle, Folder, Play, Globe, Download, Settings2 } from 'lucide-vue-next'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import ServiceTree from '@/components/ServiceTree.vue'
import { syncVariableValue, syncVariableName, removeVariable, addVariableToAll } from '@/lib/environment-utils'
import { useDialogState } from '@/composables/useDialogState'
import { useEnvironmentVariables } from '@/composables/useEnvironmentVariables'
import { useGitIntegration } from '@/composables/useGitIntegration'
import { useTabManager } from '@/composables/useTabManager'
import { useRequestExecution } from '@/composables/useRequestExecution'
import { Popover, PopoverContent, PopoverTrigger } from '@/components/ui/popover'
import RequestUrlBar from '@/components/RequestUrlBar.vue'
import RequestParameters from '@/components/RequestParameters.vue'
import RequestAuth from '@/components/RequestAuth.vue'
import RequestBody from '@/components/RequestBody.vue'
import RequestHistory from '@/components/RequestHistory.vue'
import ResponseViewer from '@/components/ResponseViewer.vue'
import ServiceSettingsView from '@/components/ServiceSettingsView.vue'
import AddServiceDialog from '@/components/dialogs/AddServiceDialog.vue'
import AddEndpointDialog from '@/components/dialogs/AddEndpointDialog.vue'
import SwaggerImportDialog from '@/components/dialogs/SwaggerImportDialog.vue'
import ShareRequestDialog from '@/components/dialogs/ShareRequestDialog.vue'
import UnsafeEnvironmentDialog from '@/components/dialogs/UnsafeEnvironmentDialog.vue'

const sharingTabData = ref<any>(null)
const searchQuery = ref('')
const searchInput = ref<HTMLInputElement | null>(null)

const servicesStore = useServicesStore()
const {
  isServiceDialogOpen,
  isEndpointDialogOpen,
  isSwaggerDialogOpen,
  isShareDialogOpen,
  isUnsafeDialogOpen
} = useDialogState()

const {
  allActiveVariables,
  activeEnvironments,
  getTabVariables,
  isUnsafeEnv,
  getEnvName
} = useEnvironmentVariables()

const {
  gitStatuses,
  fetchGitStatus,
  handleSyncGit,
  handleInitGit
} = useGitIntegration()

const {
  tabs,
  activeTab,
  addTab,
  closeTab,
  updateTabSnapshot,
  initializeTabsFromSavedState
} = useTabManager()

const {
  isSending,
  unsafeTabToProceed,
  unsafeCountdown,
  handleSendRequest,
  proceedWithUnsafeRequest: proceedUnsafe,
  cancelUnsafeRequest
} = useRequestExecution(isUnsafeDialogOpen)

const proceedWithUnsafeRequest = () => {
  proceedUnsafe(handleSendRequest)
}


onMounted(async () => {
  await servicesStore.loadServices()
  // Also load collections so variable interpolation works for collection-based tabs
  await useCollectionsStore().loadCollections()
  await initializeTabsFromSavedState()
  window.addEventListener('keydown', handleGlobalKeydown)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
})

const handleGlobalKeydown = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault()
    searchInput.value?.focus()
  }
}

const filteredServices = computed(() => {
  if (!searchQuery.value) return servicesStore.services

  const query = searchQuery.value.toLowerCase()
  return servicesStore.services
    .map(service => {
      // Check if service name matches
      const serviceMatches = service.name.toLowerCase().includes(query)

      // Filter endpoints that match
      const matchingEndpoints = service.endpoints.filter(endpoint =>
        endpoint.name.toLowerCase().includes(query) ||
        endpoint.url.toLowerCase().includes(query) ||
        endpoint.method.toLowerCase().includes(query)
      )

      // If service matches, show all endpoints or just the service?
      // Usually, it's better to show the service and all its endpoints if the service itself matches
      // but if the service doesn't match, only show matching endpoints.
      if (serviceMatches) {
        return service
      }

      if (matchingEndpoints.length > 0) {
        return {
          ...service,
          endpoints: matchingEndpoints,
          isOpen: true // Force open if we found matches inside
        }
      }

      return null
    })
    .filter((s): s is any => s !== null)
})


const handleImportService = async () => {
  const directory = await open({
    directory: true,
    multiple: false,
    title: 'Select Service Directory'
  })
  if (!directory) return

  const dirPath = Array.isArray(directory) ? directory[0] : directory

  try {
    const service: any = await invoke('import_service', { directory: dirPath })
    await servicesStore.loadServices()
    toast.success('Service Imported', {
      description: `Service "${service.name}" has been imported successfully.`
    })
  } catch (error) {
    console.error('Failed to import service:', error)
    toast.error('Import Failed', {
      description: String(error)
    })
  }
}

const handleServiceCreated = (service: any) => {
  servicesStore.addService(service)
}

const handleEndpointCreated = (endpoint: any, serviceId: string) => {
  const serviceIndex = servicesStore.services.findIndex(s => s.id === serviceId)
  if (serviceIndex !== -1) {
    const service = servicesStore.services[serviceIndex]
    const updatedService = {
      ...service,
      endpoints: [...service.endpoints, endpoint]
    }
    servicesStore.updateService(serviceIndex, updatedService)
    handleSelectEndpoint(endpoint)
  }
}

const handleSwaggerImportComplete = async () => {
  await servicesStore.loadServices()
}

const handleSaveRequest = async (tab: any) => {
  if (!tab.endpointId) {
    toast.error('Cannot save: This tab is not linked to an endpoint')
    return
  }

  // Find the service and endpoint
  let serviceIndex = -1
  let endpointIndex = -1

  for (let i = 0; i < servicesStore.services.length; i++) {
    const idx = servicesStore.services[i].endpoints.findIndex(e => e.id === tab.endpointId)
    if (idx !== -1) {
      serviceIndex = i
      endpointIndex = idx
      break
    }
  }

  if (serviceIndex === -1 || endpointIndex === -1) {
    toast.error('Endpoint not found in any service')
    return
  }

  const service = servicesStore.services[serviceIndex]
  const endpoint = service.endpoints[endpointIndex]

  // Update endpoint details
  // Note: Tab URL might include base URL, we should try to extract path if possible
  // or just save the URL as is if it's what the user wants. 
  // Professional clients usually save the path relative to service.

  let newPath = tab.url
  if (service.environments.length > 0) {
    for (const envGroup of service.environments) {
      for (const variable of envGroup.variables) {
        if (variable.name === 'BASE_URL' && variable.value && newPath.startsWith(variable.value)) {
          newPath = newPath.replace(variable.value, '')
          break
        }
      }
    }
  }
  if (!newPath.startsWith('/') && !newPath.includes('://') && !newPath.startsWith('{{')) {
    newPath = '/' + newPath
  }

  // Consolidate preflight structure
  const preflight = {
    enabled: tab.preflight.enabled,
    method: tab.preflight.method,
    url: tab.preflight.url,
    body: tab.preflight.body,
    headers: tab.preflight.headers || [],
    cacheToken: tab.preflight.cacheToken,
    cacheDuration: tab.preflight.cacheDuration,
    cacheDurationKey: tab.preflight.cacheDurationKey,
    cacheDurationUnit: tab.preflight.cacheDurationUnit,
    tokenKey: tab.preflight.tokenKey,
    tokenHeader: tab.preflight.tokenHeader
  }

  const updatedEndpoint = {
    ...endpoint,
    method: tab.method,
    url: newPath,
    params: tab.params.filter((p: any) => p.name).map((p: any) => ({ name: p.name, value: p.value })),
    headers: tab.headers.filter((h: any) => h.name).map((h: any) => ({ name: h.name, value: h.value })),
    body: tab.body.content,
    preflight: preflight,
    metadata: {
      ...endpoint.metadata,
      lastUpdated: Date.now()
    }
  }

  const updatedService = {
    ...service,
    endpoints: [...service.endpoints]
  }
  updatedService.endpoints[endpointIndex] = updatedEndpoint

  try {
    await servicesStore.updateService(serviceIndex, updatedService)

    // Sync versions from store after backend update
    const finalService = servicesStore.services[serviceIndex]
    const finalEndpoint = finalService?.endpoints.find(e => e.id === tab.endpointId)
    if (finalEndpoint) {
      tab.versions = finalEndpoint.versions || []
    }

    toast.success('Endpoint saved', {
      description: `Changes to "${endpoint.name}" have been persisted.`,
    })

    // Update snapshot after successful save
    updateTabSnapshot(tab)
  } catch (error) {
    toast.error('Failed to save endpoint')
  }
}

const handleShareRequest = (tab: any) => {
  sharingTabData.value = tab
  isShareDialogOpen.value = true
}



const handleSelectEndpoint = (endpoint: any) => {
  // Find service that owns this endpoint
  const service = servicesStore.services.find(s => s.endpoints.some(e => e.id === endpoint.id))
  // Determine if service has BASE_URL defined to use placeholders
  const hasBaseUrl = service?.environments?.some(e => e.variables.some(v => v.name === 'BASE_URL'))

  const existingTab = tabs.value.find(t => t.id === `endpoint-${endpoint.id}`)
  if (existingTab) {
    activeTab.value = existingTab.id
  } else {
    // Prepare initial state from endpoint and service
    const initialParams = endpoint.params?.length > 0
      ? endpoint.params.map((p: any) => ({ ...p, enabled: true }))
      : [{ enabled: true, name: '', value: '' }]

    const initialHeaders = endpoint.headers?.length > 0
      ? endpoint.headers.map((h: any) => ({ ...h, enabled: true }))
      : [{ enabled: true, name: '', value: '' }]

    const path = endpoint.url.startsWith('/') ? endpoint.url : `/${endpoint.url}`
    // Use placeholder instead of literal value for better experience
    const fullUrl = hasBaseUrl && endpoint.url.startsWith('/') ? `{{BASE_URL}}${path}` : endpoint.url

    // Initialize preflight from endpoint.preflight (for UI editing)
    const preflightConfig = endpoint.preflight || {
      enabled: false,
      method: 'POST',
      url: '',
      body: '',
      headers: [],
      cacheToken: true,
      cacheDuration: 'derived',
      cacheDurationKey: 'expires_in',
      cacheDurationUnit: 'seconds',
      tokenKey: 'access_token',
      tokenHeader: 'Authorization'
    }

    addTab({
      id: `endpoint-${endpoint.id}`,
      type: 'request',
      endpointId: endpoint.id,
      serviceId: service?.id,
      title: endpoint.name,
      method: endpoint.method,
      url: fullUrl,
      params: initialParams,
      headers: initialHeaders,
      body: {
        type: 'application/json',
        content: endpoint.body || ''
      },
      auth: {
        type: service?.isAuthenticated ? (service.authType?.toLowerCase() || 'none') : 'none',
        bearerToken: '',
        basicUser: '',
        basicPass: '',
        apiKeyName: '',
        apiKeyValue: '',
        apiKeyLocation: 'header'
      },
      preflight: preflightConfig,
      versions: endpoint.versions || []
    })
  }
}

const handleSelectServiceSettings = (service: any) => {
  const tabId = `settings-${service.id}`
  const existingTab = tabs.value.find(t => t.id === tabId)
  if (existingTab) {
    activeTab.value = tabId
  } else {
    addTab({
      id: tabId,
      title: `${service.name}`,
      type: 'settings',
      serviceId: service.id,
      serviceData: JSON.parse(JSON.stringify(service)) // Deep copy for editing
    })
    fetchGitStatus(service.id, service.directory)
  }
}

const handleUpdateServiceSettings = async (tab: any) => {
  const serviceIndex = servicesStore.services.findIndex(s => s.id === tab.serviceId)
  if (serviceIndex !== -1) {
    try {
      await servicesStore.updateService(serviceIndex, tab.serviceData)
      tab.title = tab.serviceData.name
      updateTabSnapshot(tab)
      toast.success('Service updated successfully')
    } catch (error) {
      toast.error('Failed to update service')
    }
  }
}

const handleReloadService = async () => {
  try {
    await servicesStore.loadServices()
    toast.success('Services reloaded from disk')
  } catch (error) {
    toast.error('Failed to reload services')
  }
}

const handleDeleteService = async (serviceId: string, tabId: string) => {
  const index = servicesStore.services.findIndex(s => s.id === serviceId)
  if (index !== -1) {
    const confirmation = await ask(
      `Are you sure you want to delete service "${servicesStore.services[index].name}"? This will remove it from your workspace settings.`,
      { title: 'Confirm Deletion', kind: 'warning' }
    )

    if (confirmation) {
      await servicesStore.deleteService(index)
      closeTab(tabId)
      toast.success('Service removed from workspace')
    }
  }
}

const restoreVersion = (tab: any, version: any) => {
  tab.method = version.config.method
  tab.url = version.config.url
  tab.auth.type = version.config.authType
  tab.params = version.config.params.map((p: any) => ({ ...p, enabled: true }))
  tab.headers = version.config.headers.map((h: any) => ({ ...h, enabled: true }))
  tab.body.content = version.config.body
  tab.preflight = JSON.parse(JSON.stringify(version.config.preflight))

  toast.success(`Restored to version ${version.version}`)
}
</script>

<template>
  <div class="flex-1 overflow-hidden h-full">
    <ResizablePanelGroup direction="horizontal">
      <!-- Left Resizable Sidebar -->
      <ResizablePanel :default-size="20" :min-size="15" class="bg-muted/5 flex flex-col h-full border-r">
        <div class="p-3 border-b flex items-center justify-between">
          <div class="relative flex-1 mr-2">
            <Search class="absolute left-2 top-2.5 h-3.5 w-3.5 text-muted-foreground" />
            <input ref="searchInput" v-model="searchQuery" type="text" placeholder="Search services... (Cmd+K)"
              class="w-full bg-background border rounded-md py-1.5 pl-7 pr-8 text-xs focus:outline-none focus:ring-1 focus:ring-primary" />
            <button v-if="searchQuery" @click="searchQuery = ''"
              class="absolute right-2 top-2 p-0.5 hover:bg-muted rounded-sm transition-colors">
              <X class="h-3 w-3 text-muted-foreground" />
            </button>
          </div>

          <div class="flex items-center gap-1">
            <Popover>
              <PopoverTrigger as-child>
                <button class="p-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground"
                  title="Import options">
                  <Download class="h-4 w-4" />
                </button>
              </PopoverTrigger>
              <PopoverContent class="w-48 p-1" align="start">
                <div class="flex flex-col">
                  <button @click="handleImportService"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <Folder class="h-3.5 w-3.5 text-blue-500" />
                    <span>From Directory</span>
                  </button>
                  <button @click="isSwaggerDialogOpen = true"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <Globe class="h-3.5 w-3.5 text-orange-500" />
                    <span>Swagger / OpenAPI</span>
                  </button>
                </div>
              </PopoverContent>
            </Popover>

            <Popover>
              <PopoverTrigger as-child>
                <button class="p-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground">
                  <Plus class="h-4 w-4" />
                </button>
              </PopoverTrigger>
              <PopoverContent class="w-48 p-1" align="end">
                <div class="flex flex-col">
                  <button @click="isServiceDialogOpen = true"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <Layers class="h-3.5 w-3.5 text-primary" />
                    <span>Add New Service</span>
                  </button>
                  <button @click="handleImportService"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <Download class="h-3.5 w-3.5 text-blue-500" />
                    <span>Import from Directory</span>
                  </button>
                  <button @click="isSwaggerDialogOpen = true"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <Globe class="h-3.5 w-3.5 text-orange-500" />
                    <span>Import from Swagger</span>
                  </button>
                  <button @click="isEndpointDialogOpen = true"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <PlusCircle class="h-3.5 w-3.5 text-green-500" />
                    <span>Add New Endpoint</span>
                  </button>
                </div>
              </PopoverContent>
            </Popover>
          </div>
        </div>
        <div class="flex-1 overflow-auto p-2">
          <ServiceTree :services="filteredServices" @select-endpoint="handleSelectEndpoint"
            @select-service-settings="handleSelectServiceSettings" @env-change="servicesStore.setSelectedEnvironment" />
        </div>
      </ResizablePanel>

      <ResizableHandle with-handle />

      <ResizablePanel :default-size="80" class="flex flex-col h-full">
        <Tabs v-model="activeTab" class="flex-1 flex flex-col overflow-hidden">
          <div class="flex items-center border-b bg-muted/20 px-2 pt-1 gap-1">
            <TabsList class="h-9 bg-transparent p-0 gap-px">
              <div v-for="tab in tabs" :key="tab.id" class="group relative flex items-center">
                <TabsTrigger :value="tab.id" :class="[
                  'h-8 px-3 rounded-t-md rounded-b-none border-b-0 data-[state=active]:bg-background data-[state=active]:border data-[state=active]:border-b-background -mb-px text-xs gap-2 relative overflow-hidden',
                  isUnsafeEnv(tab) ? 'bg-destructive/5' : ''
                ]">
                  <div v-if="isUnsafeEnv(tab)"
                    class="absolute top-0 left-0 right-0 h-[2px] bg-destructive shadow-[0_0_8px_rgba(239,68,68,0.5)]">
                  </div>
                  <template v-if="tab.type === 'settings'">
                    <Settings2 class="h-3 w-3 text-primary/70" />
                  </template>
                  <template v-else>
                    <span :class="[
                      'font-bold text-[10px]',
                      tab.method === 'GET' ? 'text-green-600' :
                        tab.method === 'POST' ? 'text-orange-600' :
                          tab.method === 'PUT' ? 'text-blue-600' : 'text-red-600'
                    ]">{{ tab.method || 'GET' }}</span>
                  </template>
                  <span class="max-w-[120px] truncate">{{ tab.title }}</span>
                  <div v-if="tab.isEdited" class="w-1.5 h-1.5 rounded-full bg-primary animate-pulse shrink-0"></div>
                  <button @click.stop="closeTab(tab.id)"
                    class="ml-1 p-0.5 rounded-sm hover:bg-muted opacity-0 group-hover:opacity-100 transition-opacity">
                    <X class="h-3 w-3" />
                  </button>
                </TabsTrigger>
              </div>
            </TabsList>
            <button @click="addTab()"
              class="p-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground mb-1">
              <Plus class="h-4 w-4" />
            </button>
          </div>

          <div class="flex-1 overflow-hidden">
            <TabsContent v-for="tab in (tabs as any)" :key="tab.id" :value="tab.id"
              class="h-full mt-0 focus-visible:ring-0">
              <!-- Request View -->
              <ResizablePanelGroup v-if="tab.type !== 'settings'" direction="vertical">
                <ResizablePanel :default-size="50" :min-size="20">
                  <div class="h-full p-4 overflow-auto">
                    <!-- URL Bar component -->
                    <RequestUrlBar v-model:method="tab.method" v-model:url="tab.url" :is-sending="isSending"
                      :is-unsafe="isUnsafeEnv(tab)" :variables="getTabVariables(tab)"
                      :environment-name="tab.serviceId ? activeEnvironments[tab.serviceId] : ''"
                      @send="handleSendRequest(tab)" @save="handleSaveRequest(tab)" @share="handleShareRequest(tab)" />

                    <div class="flex gap-4 border-b pb-0 mb-4 text-xs font-medium text-muted-foreground">
                      <button @click="tab.activeSubTab = 'params'"
                        :class="['pb-2 -mb-px px-1 transition-colors relative', tab.activeSubTab === 'params' ? 'text-primary' : 'hover:text-foreground']">
                        Params
                        <div v-if="tab.activeSubTab === 'params'"
                          class="absolute bottom-0 left-0 right-0 h-0.5 bg-primary rounded-t-full"></div>
                      </button>
                      <button @click="tab.activeSubTab = 'auth'"
                        :class="['pb-2 -mb-px px-1 transition-colors relative', tab.activeSubTab === 'auth' ? 'text-primary' : 'hover:text-foreground']">
                        Authorization
                        <div v-if="tab.activeSubTab === 'auth'"
                          class="absolute bottom-0 left-0 right-0 h-0.5 bg-primary rounded-t-full"></div>
                      </button>
                      <button @click="tab.activeSubTab = 'headers'"
                        :class="['pb-2 -mb-px px-1 transition-colors relative', tab.activeSubTab === 'headers' ? 'text-primary' : 'hover:text-foreground']">
                        Headers
                        <div v-if="tab.activeSubTab === 'headers'"
                          class="absolute bottom-0 left-0 right-0 h-0.5 bg-primary rounded-t-full"></div>
                      </button>
                      <button @click="tab.activeSubTab = 'body'"
                        :class="['pb-2 -mb-px px-1 transition-colors relative', tab.activeSubTab === 'body' ? 'text-primary' : 'hover:text-foreground']">
                        Body
                        <div v-if="tab.activeSubTab === 'body'"
                          class="absolute bottom-0 left-0 right-0 h-0.5 bg-primary rounded-t-full"></div>
                      </button>
                      <button @click="tab.activeSubTab = 'versions'"
                        :class="['pb-2 -mb-px px-1 transition-colors relative', tab.activeSubTab === 'versions' ? 'text-primary' : 'hover:text-foreground']">
                        Versions
                        <div v-if="tab.activeSubTab === 'versions'"
                          class="absolute bottom-0 left-0 right-0 h-0.5 bg-primary rounded-t-full"></div>
                      </button>
                    </div>

                    <!-- Sub-tabs Content -->
                    <div class="min-h-[200px]">
                      <!-- Params and Headers Table -->
                      <RequestParameters v-if="tab.activeSubTab === 'params'" v-model:items="tab.params"
                        title="Query Parameters" :variables="getTabVariables(tab)"
                        :environment-name="getEnvName(tab)" />
                      <RequestParameters v-else-if="tab.activeSubTab === 'headers'" v-model:items="tab.headers"
                        title="Request Headers" :variables="getTabVariables(tab)" :environment-name="getEnvName(tab)" />

                      <RequestAuth v-else-if="tab.activeSubTab === 'auth'" v-model:auth="tab.auth"
                        v-model:preflight="tab.preflight" :variables="getTabVariables(tab)"
                        :environment-name="getEnvName(tab)" />

                      <RequestBody v-else-if="tab.activeSubTab === 'body'" v-model:body="tab.body"
                        :variables="getTabVariables(tab)" :environment-name="getEnvName(tab)" />

                      <RequestHistory v-else-if="tab.activeSubTab === 'versions'" :versions="tab.versions || []"
                        @restore="(v) => restoreVersion(tab, v)" />
                    </div>
                  </div>
                </ResizablePanel>

                <ResizableHandle with-handle />

                <ResizablePanel :default-size="50" :min-size="20">
                  <ResponseViewer v-if="tab.response" :response="tab.response" :url="tab.url"
                    :variables="getTabVariables(tab)"
                    :environment-name="tab.serviceId ? activeEnvironments[tab.serviceId] : ''" />
                  <div v-else
                    class="h-full flex flex-col items-center justify-center text-muted-foreground gap-3 bg-muted/5">
                    <div class="p-4 rounded-full bg-muted/20 border border-dashed border-muted">
                      <Play class="h-8 w-8 opacity-20" />
                    </div>
                    <p class="text-[11px] font-medium tracking-tight">Send a request to see the response</p>
                  </div>
                </ResizablePanel>
              </ResizablePanelGroup>

              <!-- Service Settings View -->
              <ServiceSettingsView v-else :tab="tab" :git-status="gitStatuses[tab.serviceId]"
                @save="handleUpdateServiceSettings" @delete="handleDeleteService" @sync-git="handleSyncGit"
                @init-git="handleInitGit" @reload="handleReloadService" @add-variable="addVariableToAll"
                @remove-variable="removeVariable" @sync-variable-name="syncVariableName"
                @sync-variable-value="syncVariableValue" />
            </TabsContent>
          </div>
        </Tabs>
      </ResizablePanel>
    </ResizablePanelGroup>


    <!-- Dialogs -->
    <AddServiceDialog :open="isServiceDialogOpen" @update:open="isServiceDialogOpen = $event"
      @service-created="handleServiceCreated" />

    <AddEndpointDialog :open="isEndpointDialogOpen" :services="servicesStore.services"
      :all-active-variables="allActiveVariables" :active-environments="activeEnvironments"
      @update:open="isEndpointDialogOpen = $event" @endpoint-created="handleEndpointCreated" />

    <SwaggerImportDialog :open="isSwaggerDialogOpen" @update:open="isSwaggerDialogOpen = $event"
      @import-complete="handleSwaggerImportComplete" />

    <ShareRequestDialog :open="isShareDialogOpen" :tab="sharingTabData" @update:open="isShareDialogOpen = $event" />

    <UnsafeEnvironmentDialog :open="isUnsafeDialogOpen"
      :environment-name="unsafeTabToProceed ? getEnvName(unsafeTabToProceed) : ''" :countdown="unsafeCountdown"
      @update:open="isUnsafeDialogOpen = $event" @proceed="proceedWithUnsafeRequest" @cancel="cancelUnsafeRequest" />
  </div>
</template>
