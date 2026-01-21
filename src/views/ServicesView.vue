<script setup lang="ts">
import { onMounted } from 'vue'
import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from '@/components/ui/resizable'

// Stores & Composables
import { useServicesStore } from '@/stores/services'
import { useCollectionsStore } from '@/stores/collections'
import { useDialogState } from '@/composables/useDialogState'
import { useEnvironmentVariables } from '@/composables/useEnvironmentVariables'
import { useGitIntegration } from '@/composables/useGitIntegration'
import { useTabManager } from '@/composables/useTabManager'
import { useRequestExecution } from '@/composables/useRequestExecution'

// New Components
import ServiceExplorer from '@/components/services/ServiceExplorer.vue'
import RequestWorkspace from '@/components/workspace/RequestWorkspace.vue'
import { toast } from 'vue-sonner'

// Dialogs
import AddServiceDialog from '@/components/dialogs/AddServiceDialog.vue'
import AddEndpointDialog from '@/components/dialogs/AddEndpointDialog.vue'
import SwaggerImportDialog from '@/components/dialogs/SwaggerImportDialog.vue'
import ShareRequestDialog from '@/components/dialogs/ShareRequestDialog.vue'
import UnsafeEnvironmentDialog from '@/components/dialogs/UnsafeEnvironmentDialog.vue'
import { ref } from 'vue'

const servicesStore = useServicesStore()
const sharingTabData = ref<any>(null)

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
  getEnvName
} = useEnvironmentVariables()

const {
  gitStatuses,
  handleSyncGit,
  handleInitGit
} = useGitIntegration()

const {
  tabs,
  activeTab,
  addTab,
  closeTab,
  initializeTabsFromSavedState
} = useTabManager()

const {
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
})

// Event Handlers for child components
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
        active: true,
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
    // Git status is fetched inside Workspace/ServiceSettingsView when active
  }
}

const handleShareRequest = (tab: any) => {
  sharingTabData.value = tab
  isShareDialogOpen.value = true
}

const handleSaveRequest = async (payload: { serviceIndex: number, updatedItem: any, tab: any }) => {
  try {
    await servicesStore.updateService(payload.serviceIndex, payload.updatedItem)

    // Sync versions for tab
    const finalService = servicesStore.services[payload.serviceIndex]
    const finalEndpoint = finalService?.endpoints.find(e => e.id === payload.tab.endpointId)
    if (finalEndpoint) {
      payload.tab.versions = finalEndpoint.versions || []
    }

    toast.success('Endpoint saved', {
      description: `Changes to "${payload.tab.title}" have been persisted.`
    })
  } catch (error) {
    toast.error('Failed to save endpoint')
    console.error(error)
  }
}

const handleUpdateItem = async (payload: { index: number, data: any, tab: any }) => {
  try {
    await servicesStore.updateService(payload.index, payload.data)
    payload.tab.title = payload.data.name
    toast.success('Service updated')
  } catch (error) {
    toast.error('Failed to update service')
  }
}

const handleDeleteItem = async (payload: { index: number, id: string, tabId: string }) => {
  try {
    const serviceName = servicesStore.services[payload.index].name
    await servicesStore.deleteService(payload.index)
    // useTabManager handles closing usually, but here we explicitly ask for it
    // Wait, useTabManager is used in Workspace, but closing via prop sync or event?
    // Workspace emits delete-item, but it called closeTab BEFORE emitting? 
    // In current RequestWorkspace implementation (Step 233), it emits delete-item ONLY. It does NOT call closeTab.
    // So we must close the tab here.
    const tabsToClose = tabs.value.filter(t => t.serviceId === payload.id).map(t => t.id)
    tabsToClose.forEach(tid => {
      // We need closeTab exposed from useTabManager
      const tabIdx = tabs.value.findIndex(t => t.id === tid)
      if (tabIdx !== -1) tabs.value.splice(tabIdx, 1)
    })

    toast.success(`Service "${serviceName}" deleted`)
  } catch (error) {
    toast.error('Failed to delete service')
  }
}
</script>

<template>
  <div class="flex-1 overflow-hidden h-full">
    <ResizablePanelGroup direction="horizontal">
      <!-- Sidebar Component -->
      <ResizablePanel :default-size="20" :min-size="15">
        <ServiceExplorer @select-endpoint="handleSelectEndpoint" @select-service-settings="handleSelectServiceSettings"
          @env-change="servicesStore.setSelectedEnvironment" />
      </ResizablePanel>

      <ResizableHandle with-handle />

      <!-- Workspace Component -->
      <ResizablePanel :default-size="80">
        <RequestWorkspace v-model="activeTab" :items="servicesStore.services" :git-statuses="gitStatuses"
          label="Service" @sync-git="handleSyncGit" @init-git="handleInitGit" @share-request="handleShareRequest"
          @save-request="handleSaveRequest" @update-item="handleUpdateItem" @delete-item="handleDeleteItem"
          @reload-items="servicesStore.loadServices" />
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
