import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { type Service } from '@/types'
import { toast } from 'vue-sonner'

export const useCollectionsStore = defineStore('collections', () => {
    const collections = ref<Service[]>([])
    const isLoading = ref(false)

    const loadCollections = async () => {
        isLoading.value = true
        try {
            const data = await invoke<Service[]>('get_collections')
            collections.value = data || []
        } catch (error) {
            console.error('Failed to load collections:', error)
            toast.error('Failed to load collections', {
                description: String(error)
            })
        } finally {
            isLoading.value = false
        }
    }

    const saveCollections = async () => {
        try {
            const updated = await invoke<Service[]>('save_collections', { collections: collections.value })
            if (updated) {
                collections.value = updated
            }
        } catch (error) {
            console.error('Failed to save collections:', error)
        }
    }

    const addCollection = async (collection: Service) => {
        collections.value.push(collection)
        await saveCollections()
    }

    const updateCollection = async (index: number, collection: Service) => {
        collections.value[index] = collection
        await saveCollections()
    }

    const deleteCollection = async (index: number) => {
        collections.value.splice(index, 1)
        await saveCollections()
    }

    const setSelectedEnvironment = async (collectionId: string, env: string) => {
        const index = collections.value.findIndex(c => c.id === collectionId)
        if (index !== -1) {
            collections.value[index].selectedEnvironment = env
            await saveCollections()
        }
    }

    return {
        collections,
        isLoading,
        loadCollections,
        saveCollections,
        addCollection,
        updateCollection,
        deleteCollection,
        setSelectedEnvironment
    }
})
