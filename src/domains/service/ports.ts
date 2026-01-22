import type { Service } from '@/types'

export interface IServiceGateway {
    loadServices(): Promise<Service[]>;
    saveServices(services: Service[]): Promise<Service[]>;
    getGitStatus(directory: string): Promise<any>;
    initGit(directory: string, remoteUrl?: string): Promise<void>;
    syncGit(directory: string): Promise<void>;
    importService(directory: string): Promise<Service>;
    importCurl(serviceId: string, curlCommand: string): Promise<Service>;
}
