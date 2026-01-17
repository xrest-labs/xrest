import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { type Service } from '@/types'
import { toast } from 'vue-sonner'

export const useServicesStore = defineStore('services', () => {
    const services = ref<Service[]>([])
    const isLoading = ref(false)

    const loadServices = async () => {
        isLoading.value = true
        try {
            const data = await invoke<Service[]>('get_services')
            services.value = data || []
        } catch (error) {
            console.error('Failed to load services:', error)
            toast.error('Failed to load services', {
                description: String(error)
            })
        } finally {
            isLoading.value = false
        }
    }

    const saveServices = async () => {
        try {
            const updated = await invoke<Service[]>('save_services', { services: services.value })
            if (updated) {
                services.value = updated
            }
        } catch (error) {
            console.error('Failed to save services:', error)
        }
    }

    const addService = async (service: Service) => {
        services.value.push(service)
        await saveServices()
    }

    const updateService = async (index: number, service: Service) => {
        services.value[index] = service
        await saveServices()
    }

    const deleteService = async (index: number) => {
        services.value.splice(index, 1)
        await saveServices()
    }

    const setSelectedEnvironment = async (serviceId: string, env: string) => {
        const index = services.value.findIndex(s => s.id === serviceId)
        if (index !== -1) {
            services.value[index].selectedEnvironment = env
            await saveServices()
        }
    }

    const getGitStatus = async (directory: string) => {
        try {
            return await invoke<any>('get_git_status', { directory })
        } catch (error) {
            console.error('Failed to get git status:', error)
            return null
        }
    }

    const initGit = async (directory: string, remoteUrl?: string) => {
        try {
            await invoke('git_init', { directory, remoteUrl })
            toast.success('Git initialized successfully')
        } catch (error) {
            toast.error('Failed to initialize git', {
                description: String(error)
            })
        }
    }

    const syncGit = async (directory: string) => {
        try {
            await invoke('git_sync', { directory })
            toast.success('Git sync completed')
        } catch (error) {
            toast.error('Failed to sync git', {
                description: String(error)
            })
        }
    }

    return {
        services,
        isLoading,
        loadServices,
        saveServices,
        addService,
        updateService,
        deleteService,
        setSelectedEnvironment,
        getGitStatus,
        initGit,
        syncGit
    }
})
