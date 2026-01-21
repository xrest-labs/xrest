import { invoke } from '@tauri-apps/api/core'
import type { Service } from '@/types'
import type { IServiceGateway } from '@/domains/service/ports'

export class TauriServiceGateway implements IServiceGateway {
    async loadServices(): Promise<Service[]> {
        return invoke<Service[]>('get_services')
    }

    async saveServices(services: Service[]): Promise<Service[]> {
        return invoke<Service[]>('save_services', { services })
    }

    async getGitStatus(directory: string): Promise<any> {
        return invoke<any>('get_git_status', { directory })
    }

    async initGit(directory: string, remoteUrl?: string): Promise<void> {
        return invoke('git_init', { directory, remoteUrl })
    }

    async syncGit(directory: string): Promise<void> {
        return invoke('git_sync', { directory })
    }

    async importService(directory: string): Promise<Service> {
        return invoke('import_service', { directory })
    }
}
