import type { Service } from '@/types'
import type { IServiceGateway } from './ports'

export class ServiceManager {
    constructor(private gateway: IServiceGateway) { }

    async getAllServices(): Promise<Service[]> {
        return this.gateway.loadServices()
    }

    async saveServices(services: Service[]): Promise<Service[]> {
        return this.gateway.saveServices(services)
    }

    async addService(services: Service[], service: Service): Promise<Service[]> {
        const newServices = [...services, service]
        return this.saveServices(newServices)
    }

    async updateService(services: Service[], index: number, service: Service): Promise<Service[]> {
        const newServices = [...services]
        newServices[index] = service
        return this.saveServices(newServices)
    }

    async deleteService(services: Service[], index: number): Promise<Service[]> {
        const newServices = [...services]
        newServices.splice(index, 1)
        return this.saveServices(newServices)
    }

    async getGitStatus(directory: string): Promise<any> {
        return this.gateway.getGitStatus(directory)
    }

    async initGit(directory: string, remoteUrl?: string): Promise<void> {
        return this.gateway.initGit(directory, remoteUrl)
    }

    async syncGit(directory: string): Promise<void> {
        return this.gateway.syncGit(directory)
    }

    async importService(directory: string): Promise<Service> {
        return this.gateway.importService(directory)
    }

    async importCurl(serviceId: string, curlCommand: string): Promise<Service> {
        return this.gateway.importCurl(serviceId, curlCommand)
    }
}
