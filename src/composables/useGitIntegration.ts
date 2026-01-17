/**
 * Git Integration Composable
 *
 * Manages git operations for service directories including:
 * - Fetching git status
 * - Initializing git repositories
 * - Syncing with remote repositories
 */

import { ref } from 'vue'
import { useServicesStore } from '@/stores/services'

/**
 * Hook to manage git integration for services
 * @returns Object with git status and operation functions
 */
export const useGitIntegration = () => {
  const servicesStore = useServicesStore()
  const gitStatuses = ref<Record<string, any>>({})

  /**
   * Fetch git status for a service directory
   * @param serviceId - Service ID
   * @param directory - Directory path to check
   */
  const fetchGitStatus = async (serviceId: string, directory: string) => {
    const status = await servicesStore.getGitStatus(directory)
    if (status) {
      gitStatuses.value[serviceId] = status
    }
  }

  /**
   * Sync git repository with remote
   * @param serviceId - Service ID
   * @param directory - Directory path to sync
   */
  const handleSyncGit = async (serviceId: string, directory: string) => {
    await servicesStore.syncGit(directory)
    await fetchGitStatus(serviceId, directory)
  }

  /**
   * Initialize git repository in directory
   * @param serviceId - Service ID
   * @param directory - Directory path to initialize
   * @param gitUrl - Optional remote URL to set
   */
  const handleInitGit = async (serviceId: string, directory: string, gitUrl?: string) => {
    await servicesStore.initGit(directory, gitUrl)
    await fetchGitStatus(serviceId, directory)
  }

  return {
    gitStatuses,
    fetchGitStatus,
    handleSyncGit,
    handleInitGit
  }
}
