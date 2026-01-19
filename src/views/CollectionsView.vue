<script setup lang="ts">
import { ref, onMounted, computed, onUnmounted } from 'vue'
import { toast } from 'vue-sonner'
import { useCollectionsStore } from '@/stores/collections'
import { Search, Plus, X, Layers, PlusCircle, Play, Settings2, ArrowRight } from 'lucide-vue-next'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'
import ServiceTree from '@/components/ServiceTree.vue'
import { syncVariableValue, syncVariableName, removeVariable, addVariableToAll } from '@/lib/environment-utils'
import { useDialogState } from '@/composables/useDialogState'
import { useEnvironmentVariables } from '@/composables/useEnvironmentVariables'
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
import AddCollectionDialog from '@/components/dialogs/AddCollectionDialog.vue'
import AddCollectionEndpointDialog from '@/components/dialogs/AddCollectionEndpointDialog.vue'
import ShareRequestDialog from '@/components/dialogs/ShareRequestDialog.vue'
import UnsafeEnvironmentDialog from '@/components/dialogs/UnsafeEnvironmentDialog.vue'
import AddToServiceDialog from '@/components/dialogs/AddToServiceDialog.vue'
import { useServicesStore } from '@/stores/services'

const sharingTabData = ref<any>(null)
const searchQuery = ref('')
const searchInput = ref<HTMLInputElement | null>(null)

const collectionsStore = useCollectionsStore()
const servicesStore = useServicesStore()
const {
  isServiceDialogOpen: isCollectionDialogOpen,
  isEndpointDialogOpen: isCollectionEndpointDialogOpen,
  isShareDialogOpen,
  isUnsafeDialogOpen
} = useDialogState()

const {
  activeEnvironments,
  getTabVariables,
  isUnsafeEnv,
  getEnvName
} = useEnvironmentVariables()

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
  await collectionsStore.loadCollections()
  await servicesStore.loadServices()
  await initializeTabsFromSavedState()
  window.addEventListener('keydown', handleGlobalKeydown)
  window.addEventListener('click', closeContextMenu)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleGlobalKeydown)
  window.removeEventListener('click', closeContextMenu)
})

const isAddToServiceDialogOpen = ref(false)
const endpointToMigrate = ref<any>(null)
const sourceCollectionMigrate = ref<any>(null)

const contextMenu = ref({
  show: false,
  x: 0,
  y: 0,
  endpoint: null as any
})

const handleEndpointContext = (e: MouseEvent, endpoint: any) => {
  contextMenu.value = {
    show: true,
    x: e.clientX,
    y: e.clientY,
    endpoint: endpoint
  }
}

const closeContextMenu = () => {
  contextMenu.value.show = false
}

const openMigrateDialog = () => {
  if (!contextMenu.value.endpoint) return

  endpointToMigrate.value = contextMenu.value.endpoint
  // Find source collection
  sourceCollectionMigrate.value = collectionsStore.collections.find(c =>
    c.endpoints.some(e => e.id === contextMenu.value.endpoint.id)
  )

  isAddToServiceDialogOpen.value = true
  closeContextMenu()
}

const handleEndpointMigrated = async (targetServiceId: string, updatedEndpoint: any, newVariables: any[]) => {
  try {
    // 1. Add endpoint to service
    const serviceIndex = servicesStore.services.findIndex(s => s.id === targetServiceId)
    if (serviceIndex === -1) return

    const service = servicesStore.services[serviceIndex]

    // Merge variables into ALL environments of the target service
    const updatedService = {
      ...service,
      environments: service.environments.map(env => ({
        ...env,
        variables: [
          ...env.variables,
          ...newVariables.filter(nv => !env.variables.some(ev => ev.name === nv.name))
        ]
      })),
      endpoints: [...service.endpoints, updatedEndpoint]
    }

    await servicesStore.updateService(serviceIndex, updatedService)

    // 2. Remove endpoint from collection
    const collectionIndex = collectionsStore.collections.findIndex(c =>
      c.endpoints.some(e => e.id === updatedEndpoint.id)
    )
    if (collectionIndex !== -1) {
      const collection = collectionsStore.collections[collectionIndex]
      const updatedCollection = {
        ...collection,
        endpoints: collection.endpoints.filter(e => e.id !== updatedEndpoint.id)
      }
      await collectionsStore.updateCollection(collectionIndex, updatedCollection)
    }

    // 3. Close tab if it was open
    closeTab(`endpoint-${updatedEndpoint.id}`)

    toast.success(`"${updatedEndpoint.name}" moved to service "${service.name}"`)
  } catch (err) {
    console.error('Migration failed:', err)
    toast.error('Failed to migrate endpoint')
  }
}

const handleGlobalKeydown = (e: KeyboardEvent) => {
  if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
    e.preventDefault()
    searchInput.value?.focus()
  }
}

const filteredCollections = computed(() => {
  if (!searchQuery.value) return collectionsStore.collections

  const query = searchQuery.value.toLowerCase()
  return collectionsStore.collections
    .map(collection => {
      const collectionMatches = collection.name.toLowerCase().includes(query)
      const matchingEndpoints = collection.endpoints.filter(endpoint =>
        endpoint.name.toLowerCase().includes(query) ||
        endpoint.url.toLowerCase().includes(query) ||
        endpoint.method.toLowerCase().includes(query)
      )

      if (collectionMatches) return collection
      if (matchingEndpoints.length > 0) {
        return {
          ...collection,
          endpoints: matchingEndpoints,
          isOpen: true
        }
      }
      return null
    })
    .filter((c): c is any => c !== null)
})

const handleCollectionCreated = (collection: any) => {
  collectionsStore.addCollection(collection)
}

const handleEndpointCreated = async (endpoint: any, collectionId: string) => {
  let finalCollectionId = collectionId
  let collectionIndex = collectionsStore.collections.findIndex(c => c.id === collectionId)

  if (collectionIndex === -1) {
    // Create a default collection if it doesn't exist or none was selected
    const newCollection: any = {
      id: `c-${Date.now()}`,
      name: 'My Collection',
      directory: '',
      isAuthenticated: false,
      authType: 'none',
      endpoints: [],
      environments: [
        {
          name: 'GLOBAL',
          isUnsafe: false,
          variables: [{ name: 'BASE_URL', value: 'https://api.example.com' }]
        }
      ],
      selectedEnvironment: 'GLOBAL'
    }
    await collectionsStore.addCollection(newCollection)
    finalCollectionId = newCollection.id
    collectionIndex = collectionsStore.collections.length - 1
  }

  const collection = collectionsStore.collections[collectionIndex]
  const updatedCollection = {
    ...collection,
    endpoints: [...collection.endpoints, { ...endpoint, serviceId: finalCollectionId }]
  }
  await collectionsStore.updateCollection(collectionIndex, updatedCollection)
  handleSelectEndpoint(updatedCollection.endpoints[updatedCollection.endpoints.length - 1])
}

const handleSaveRequest = async (tab: any) => {
  if (!tab.endpointId) {
    toast.error('Cannot save: This tab is not linked to an endpoint')
    return
  }

  let collectionIndex = -1
  let endpointIndex = -1

  for (let i = 0; i < collectionsStore.collections.length; i++) {
    const idx = collectionsStore.collections[i].endpoints.findIndex(e => e.id === tab.endpointId)
    if (idx !== -1) {
      collectionIndex = i
      endpointIndex = idx
      break
    }
  }

  if (collectionIndex === -1 || endpointIndex === -1) {
    toast.error('Endpoint not found in any collection')
    return
  }

  const collection = collectionsStore.collections[collectionIndex]
  const endpoint = collection.endpoints[endpointIndex]

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
    url: tab.url,
    params: tab.params.filter((p: any) => p.name).map((p: any) => ({ name: p.name, value: p.value, enabled: p.enabled ?? true })),
    headers: tab.headers.filter((h: any) => h.name).map((h: any) => ({ name: h.name, value: h.value, enabled: h.enabled ?? true })),
    body: tab.body.content,
    preflight: preflight,
    metadata: {
      ...endpoint.metadata,
      lastUpdated: Date.now()
    }
  }

  const updatedCollection = {
    ...collection,
    endpoints: [...collection.endpoints]
  }
  updatedCollection.endpoints[endpointIndex] = updatedEndpoint

  try {
    await collectionsStore.updateCollection(collectionIndex, updatedCollection)
    toast.success('Endpoint saved', {
      description: `Changes to "${endpoint.name}" have been persisted.`,
    })
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
  const collection = collectionsStore.collections.find(c => c.endpoints.some(e => e.id === endpoint.id))
  const existingTab = tabs.value.find(t => t.id === `endpoint-${endpoint.id}`)

  if (existingTab) {
    activeTab.value = existingTab.id
  } else {
    const initialParams = endpoint.params?.length > 0
      ? endpoint.params.map((p: any) => ({ ...p, enabled: true }))
      : [{ enabled: true, name: '', value: '' }]

    const initialHeaders = endpoint.headers?.length > 0
      ? endpoint.headers.map((h: any) => ({ ...h, enabled: true }))
      : [{ enabled: true, name: '', value: '' }]

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
      serviceId: collection?.id,
      title: endpoint.name,
      method: endpoint.method,
      url: endpoint.url,
      params: initialParams,
      headers: initialHeaders,
      body: {
        type: 'application/json',
        content: endpoint.body || ''
      },
      auth: {
        type: 'none',
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

const handleSelectCollectionSettings = (collection: any) => {
  const tabId = `settings-${collection.id}`
  const existingTab = tabs.value.find(t => t.id === tabId)
  if (existingTab) {
    activeTab.value = tabId
  } else {
    addTab({
      id: tabId,
      title: `${collection.name}`,
      type: 'settings',
      serviceId: collection.id,
      serviceData: JSON.parse(JSON.stringify(collection))
    })
  }
}

const handleUpdateCollectionSettings = async (tab: any) => {
  const index = collectionsStore.collections.findIndex(c => c.id === tab.serviceId)
  if (index !== -1) {
    try {
      await collectionsStore.updateCollection(index, tab.serviceData)
      tab.title = tab.serviceData.name
      updateTabSnapshot(tab)
      toast.success('Collection updated successfully')
    } catch (error) {
      toast.error('Failed to update collection')
    }
  }
}

const handleDeleteCollection = async (collectionId: string, tabId: string) => {
  const index = collectionsStore.collections.findIndex(c => c.id === collectionId)
  if (index !== -1) {
    const confirmation = confirm(`Are you sure you want to delete collection "${collectionsStore.collections[index].name}"?`)
    if (confirmation) {
      await collectionsStore.deleteCollection(index)
      closeTab(tabId)
      toast.success('Collection deleted')
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
            <input ref="searchInput" v-model="searchQuery" type="text" placeholder="Search scratchpad..."
              class="w-full bg-background border rounded-md py-1.5 pl-7 pr-8 text-xs focus:outline-none focus:ring-1 focus:ring-primary" />
            <button v-if="searchQuery" @click="searchQuery = ''"
              class="absolute right-2 top-2 p-0.5 hover:bg-muted rounded-sm transition-colors">
              <X class="h-3 w-3 text-muted-foreground" />
            </button>
          </div>

          <div class="flex items-center gap-1">
            <Popover>
              <PopoverTrigger as-child>
                <button class="p-1.5 hover:bg-muted rounded-md transition-colors text-muted-foreground">
                  <Plus class="h-4 w-4" />
                </button>
              </PopoverTrigger>
              <PopoverContent class="w-48 p-1" align="end">
                <div class="flex flex-col">
                  <button @click="isCollectionDialogOpen = true"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <Layers class="h-3.5 w-3.5 text-primary" />
                    <span>New Collection</span>
                  </button>
                  <button @click="isCollectionEndpointDialogOpen = true"
                    class="flex items-center gap-2 px-2 py-2 text-xs hover:bg-muted rounded-sm text-left transition-colors">
                    <PlusCircle class="h-3.5 w-3.5 text-green-500" />
                    <span>New Endpoint</span>
                  </button>
                </div>
              </PopoverContent>
            </Popover>
          </div>
        </div>
        <div class="flex-1 overflow-auto p-2">
          <div v-if="collectionsStore.collections.length === 0"
            class="h-full flex flex-col items-center justify-center p-4 text-center space-y-2 opacity-50">
            <Layers class="h-8 w-8 mb-2" />
            <p class="text-[11px] font-medium leading-tight">API Scratchpad</p>
            <p class="text-[10px]">Create a collection to start exploring APIs without git or directories.</p>
          </div>
          <ServiceTree v-else :services="filteredCollections" @select-endpoint="handleSelectEndpoint"
            @select-service-settings="handleSelectCollectionSettings"
            @env-change="collectionsStore.setSelectedEnvironment" @endpoint-context="handleEndpointContext" />
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
                    <RequestUrlBar v-model:method="tab.method" v-model:url="tab.url" :is-sending="isSending"
                      :is-unsafe="isUnsafeEnv(tab)" :variables="getTabVariables(tab)"
                      :environment-name="getEnvName(tab)" @send="handleSendRequest(tab)" @save="handleSaveRequest(tab)"
                      @share="handleShareRequest(tab)" />

                    <div class="flex gap-4 border-b pb-0 mb-4 text-xs font-medium text-muted-foreground">
                      <button v-for="sub in ['params', 'auth', 'headers', 'body']" :key="sub"
                        @click="tab.activeSubTab = sub"
                        :class="['pb-2 -mb-px px-1 transition-colors relative capitalize', tab.activeSubTab === sub ? 'text-primary' : 'hover:text-foreground']">
                        {{ sub }}
                        <div v-if="tab.activeSubTab === sub"
                          class="absolute bottom-0 left-0 right-0 h-0.5 bg-primary rounded-t-full"></div>
                      </button>
                    </div>

                    <div class="min-h-[200px]">
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

              <!-- Settings View -->
              <ServiceSettingsView v-else :tab="tab" @save="handleUpdateCollectionSettings"
                @delete="handleDeleteCollection" @add-variable="addVariableToAll" @remove-variable="removeVariable"
                @sync-variable-name="syncVariableName" @sync-variable-value="syncVariableValue" />
            </TabsContent>
          </div>
        </Tabs>
      </ResizablePanel>
    </ResizablePanelGroup>

    <!-- Dialogs -->
    <AddCollectionDialog :open="isCollectionDialogOpen" @update:open="isCollectionDialogOpen = $event"
      @collection-created="handleCollectionCreated" />

    <AddCollectionEndpointDialog :open="isCollectionEndpointDialogOpen" :collections="collectionsStore.collections"
      @update:open="isCollectionEndpointDialogOpen = $event" @endpoint-created="handleEndpointCreated" />

    <ShareRequestDialog :open="isShareDialogOpen" :tab="sharingTabData" @update:open="isShareDialogOpen = $event" />

    <UnsafeEnvironmentDialog :open="isUnsafeDialogOpen"
      :environment-name="unsafeTabToProceed ? getEnvName(unsafeTabToProceed) : ''" :countdown="unsafeCountdown"
      @update:open="isUnsafeDialogOpen = $event" @proceed="proceedWithUnsafeRequest" @cancel="cancelUnsafeRequest" />

    <AddToServiceDialog :open="isAddToServiceDialogOpen" @update:open="isAddToServiceDialogOpen = $event"
      :endpoint="endpointToMigrate" :source-collection="sourceCollectionMigrate" @added="handleEndpointMigrated" />

    <!-- Context Menu -->
    <Teleport to="body">
      <div v-if="contextMenu.show" :style="{ top: contextMenu.y + 'px', left: contextMenu.x + 'px' }"
        class="fixed z-[100] bg-popover text-popover-foreground border shadow-md rounded-md p-1 min-w-[150px] animate-in fade-in zoom-in-95 duration-100">
        <button @click="openMigrateDialog"
          class="flex w-full items-center gap-2 px-2.5 py-1.5 text-xs hover:bg-accent hover:text-accent-foreground rounded-sm transition-colors">
          <ArrowRight class="h-3.5 w-3.5 text-primary" />
          Add to Service
        </button>
      </div>
    </Teleport>
  </div>
</template>
