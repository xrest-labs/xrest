import type { IServiceGateway } from '@/domains/service/ports'
import { TauriServiceGateway } from './service/tauri-gateway'
import { MockServiceGateway } from './service/mock-gateway'

export class AdapterFactory {
    static getServiceGateway(): IServiceGateway {
        // Check if running in Tauri environment
        // @ts-ignore
        if (window.__TAURI_INTERNALS__) {
            return new TauriServiceGateway()
        }
        return new MockServiceGateway()
    }
}
