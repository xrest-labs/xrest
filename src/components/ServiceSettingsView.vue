<script setup lang="ts">
import { Save, Trash2, GitBranch, CheckCircle2, RefreshCw, Plus, Settings2, Globe, AlertCircle } from 'lucide-vue-next'
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from '@/components/ui/table'
import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Switch } from '@/components/ui/switch'
import {
  getUniqueVariableNames,
  syncVariableValue,
  syncVariableName,
  removeVariable,
  addVariableToAll
} from '@/lib/environment-utils'
import { computed } from 'vue'

const props = defineProps<{
  tab: any
  gitStatus?: any
}>();

const tab = computed(() => props.tab);
const gitStatus = computed(() => props.gitStatus);

const emit = defineEmits<{
  (e: 'save', tab: any): void
  (e: 'delete', serviceId: string, tabId: string): void
  (e: 'syncGit', serviceId: string, directory: string): void
  (e: 'initGit', serviceId: string, directory: string, gitUrl?: string): void
}>()
</script>

<template>
  <div class="h-full overflow-auto p-6 max-w-5xl mx-auto space-y-8 pb-32">
    <!-- Header Area -->
    <div class="flex items-center justify-between border-b pb-4">
      <div class="space-y-1">
        <h2 class="text-xl font-bold tracking-tight">
          {{ tab.serviceData.directory ? 'Service Settings' : 'Collection Settings' }}
        </h2>
        <p class="text-sm text-muted-foreground">Manage environments, variables, and {{ tab.serviceData.directory ?
          'project integration' : 'settings' }} for {{ tab.serviceData.name }}.</p>
      </div>
      <div class="flex items-center gap-2">
        <Button variant="destructive" size="sm" class="h-8 gap-2" @click="emit('delete', tab.serviceId, tab.id)">
          <Trash2 class="h-3.5 w-3.5" /> {{ tab.serviceData.directory ? 'Delete Service' : 'Delete Collection' }}
        </Button>
        <Button variant="default" size="sm" class="h-8 gap-2 px-4" @click="emit('save', tab)">
          <Save class="h-3.5 w-3.5" /> Save Changes
        </Button>
      </div>
    </div>

    <!-- General Settings Section -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
      <div class="space-y-4 md:col-span-2">
        <div class="bg-card rounded-lg border shadow-sm p-5 space-y-4">
          <div class="flex items-center gap-2 mb-2">
            <Settings2 class="h-4 w-4 text-primary" />
            <h3 class="text-sm font-semibold uppercase tracking-wider">General Information</h3>
          </div>
          <div class="grid gap-4">
            <div class="space-y-2">
              <Label class="text-xs">Service Name</Label>
              <Input v-model="tab.serviceData.name" class="h-9" placeholder="Enter service name" />
            </div>
            <div v-if="tab.serviceData.directory" class="space-y-2">
              <Label class="text-xs">Base Directory</Label>
              <div class="flex gap-2">
                <Input :model-value="tab.serviceData.directory" readonly
                  class="h-9 bg-muted/30 font-mono text-[11px] flex-1" />
              </div>
              <p class="text-[10px] text-muted-foreground">All service configuration is stored in this directory as YAML
                files.</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Quick Status Card -->
      <div v-if="tab.serviceData.directory" class="space-y-4">
        <div class="bg-card rounded-lg border shadow-sm p-5 h-full">
          <div class="flex items-center gap-2 mb-4">
            <GitBranch class="h-4 w-4 text-primary" />
            <h3 class="text-sm font-semibold uppercase tracking-wider">Git Status</h3>
          </div>
          <div v-if="gitStatus" class="space-y-4 mt-2">
            <div class="flex items-center justify-between">
              <span class="text-xs text-muted-foreground">Status</span>
              <div v-if="gitStatus.isGit"
                class="flex items-center gap-1.5 text-xs font-bold text-green-600 bg-green-50 px-2 py-0.5 rounded-full border border-green-100">
                <CheckCircle2 class="h-3 w-3" /> TRACKED
              </div>
              <div v-else
                class="flex items-center gap-1.5 text-xs font-bold text-muted-foreground bg-muted px-2 py-0.5 rounded-full border">
                UNTRACKED
              </div>
            </div>
            <div v-if="gitStatus.isGit" class="space-y-3">
              <div class="flex items-center justify-between text-xs border-b border-dashed pb-2">
                <span class="text-muted-foreground">Branch</span>
                <span class="font-mono font-bold">{{ gitStatus.branch || 'main' }}</span>
              </div>
              <div class="flex items-center justify-between text-xs border-b border-dashed pb-2">
                <span class="text-muted-foreground">Dirty Files</span>
                <span :class="['font-bold', gitStatus.hasUncommittedChanges ? 'text-orange-500' : 'text-green-600']">
                  {{ gitStatus.hasUncommittedChanges ? 'Changes Detected' : 'Clean' }}
                </span>
              </div>
              <Button variant="outline" size="sm" class="w-full h-8 text-[11px] gap-2 mt-2"
                @click="emit('syncGit', tab.serviceId, tab.serviceData.directory)">
                <RefreshCw class="h-3 w-3" /> Sync with Remote
              </Button>
            </div>
            <div v-else class="p-3 bg-muted/20 rounded border border-dashed text-center space-y-2">
              <p class="text-[10px] text-muted-foreground">This service directory is not a git repository. Initialize
                git for automatic change tracking.</p>
              <Button variant="outline" size="sm" class="h-7 text-[10px] gap-1.5"
                @click="emit('initGit', tab.serviceId, tab.serviceData.directory)">
                <GitBranch class="h-3 w-3" /> Initialize Git
              </Button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Environments & Variables Section -->
    <div class="bg-card rounded-lg border shadow-sm p-6 overflow-hidden">
      <div class="flex items-center justify-between mb-6">
        <div class="flex items-center gap-2">
          <Globe class="h-4 w-4 text-primary" />
          <h3 class="text-sm font-semibold uppercase tracking-wider">Shared Environments & Variables</h3>
        </div>
        <Button variant="ghost" size="sm" class="h-8 gap-2 text-primary hover:bg-primary/5"
          @click="addVariableToAll(tab.serviceData.environments)">
          <Plus class="h-3.5 w-3.5" /> Add New Variable
        </Button>
      </div>

      <div class="relative border rounded-md overflow-hidden bg-background">
        <Table>
          <TableHeader>
            <TableRow class="hover:bg-transparent bg-muted/30">
              <TableHead class="w-[180px] font-bold text-xs">Variable Name</TableHead>
              <TableHead v-for="env in tab.serviceData.environments" :key="env.name" class="min-w-[150px] border-l">
                <div class="flex flex-col gap-1 items-start py-2">
                  <div class="flex items-center gap-1.5">
                    <span
                      :class="['w-1.5 h-1.5 rounded-full', env.isUnsafe ? 'bg-destructive shadow-[0_0_8px_rgba(239,68,68,0.5)]' : 'bg-green-500 shadow-[0_0_8px_rgba(34,197,94,0.5)]']"></span>
                    <span class="font-bold text-xs">{{ env.name }}</span>
                  </div>
                  <div class="flex items-center gap-1.5 mt-1">
                    <Switch :checked="env.isUnsafe" @update:checked="env.isUnsafe = $event" class="scale-50" />
                    <span class="text-[9px] uppercase tracking-tighter font-bold text-muted-foreground opacity-70">Prod
                      Warn</span>
                  </div>
                </div>
              </TableHead>
              <TableHead class="w-12 border-l"></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            <TableRow v-for="varName in getUniqueVariableNames(tab.serviceData.environments)" :key="varName"
              class="group h-11 hover:bg-muted/10 transition-colors">
              <TableCell class="p-0 font-medium">
                <input :value="varName"
                  @blur="(e) => syncVariableName(tab.serviceData.environments, varName, (e.target as HTMLInputElement).value)"
                  class="w-full h-11 bg-transparent border-none text-[10px] px-3 font-mono focus:outline-none focus:ring-0" />
              </TableCell>
              <TableCell v-for="env in tab.serviceData.environments" :key="env.name" class="p-0 border-l group/cell">
                <input :value="env.variables.find((v: any) => v.name === varName)?.value || ''"
                  @input="(e) => syncVariableValue(env, varName, (e.target as HTMLInputElement).value)"
                  class="w-full h-11 bg-transparent border-none text-[10px] px-4 font-mono focus:outline-none focus:ring-0 group-hover/cell:bg-muted/20 transition-all"
                  :placeholder="`Value for ${env.name}`" />
              </TableCell>
              <TableCell class="p-0 border-l text-center">
                <button @click="removeVariable(tab.serviceData.environments, varName)"
                  class="p-1 text-muted-foreground hover:text-destructive opacity-20 group-hover:opacity-100 transition-opacity">
                  <Trash2 class="h-3.5 w-3.5" />
                </button>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>
      <div v-if="tab.serviceData.environments && getUniqueVariableNames(tab.serviceData.environments).length === 0"
        class="flex flex-col items-center justify-center py-12 text-muted-foreground border border-t-0 rounded-b-md bg-muted/5">
        <AlertCircle class="h-8 w-8 opacity-20 mb-2" />
        <p class="text-xs">No variables defined for this {{ tab.serviceData.directory ? 'service' : 'collection' }}.</p>
        <button class="text-[10px] text-primary hover:underline mt-1"
          @click="addVariableToAll(tab.serviceData.environments)">Add your first variable</button>
      </div>
    </div>
  </div>
</template>
