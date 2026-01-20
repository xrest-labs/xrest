import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ServiceSettingsView from '../ServiceSettingsView.vue'

describe('ServiceSettingsView - Reload Button', () => {
    const createMockServiceTab = () => ({
        id: 'settings-test',
        type: 'settings',
        serviceId: 'service-1',
        title: 'Test Service',
        serviceData: {
            id: 'service-1',
            name: 'Test Service',
            directory: '/test/path',
            isAuthenticated: false,
            authType: 'none',
            endpoints: [],
            environments: [
                {
                    name: 'DEV',
                    isUnsafe: false,
                    variables: [{ name: 'BASE_URL', value: 'http://localhost:3000' }]
                }
            ]
        }
    })

    const createMockCollectionTab = () => ({
        id: 'settings-test',
        type: 'settings',
        serviceId: 'collection-1',
        title: 'Test Collection',
        serviceData: {
            id: 'collection-1',
            name: 'Test Collection',
            directory: '', // Collections don't have directories
            isAuthenticated: false,
            authType: 'none',
            endpoints: [],
            environments: [
                {
                    name: 'GLOBAL',
                    isUnsafe: false,
                    variables: [{ name: 'BASE_URL', value: 'http://localhost:3000' }]
                }
            ]
        }
    })

    beforeEach(() => {
        vi.clearAllMocks()
    })

    it('should show reload button for services with directories', async () => {
        const tab = createMockServiceTab()
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Find reload button
        const reloadButton = wrapper.findAll('button').find(b => b.text().includes('Reload'))
        expect(reloadButton).toBeDefined()
        expect(reloadButton?.exists()).toBe(true)
    })

    it('should NOT show reload button for collections (no directory)', async () => {
        const tab = createMockCollectionTab()
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Reload button should not exist for collections
        const reloadButton = wrapper.findAll('button').find(b => b.text().includes('Reload'))
        expect(reloadButton).toBeUndefined()
    })

    it('should emit reload event when reload button is clicked', async () => {
        const tab = createMockServiceTab()
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Click reload button
        const reloadButton = wrapper.findAll('button').find(b => b.text().includes('Reload'))
        await reloadButton?.trigger('click')

        // Verify reload event was emitted with serviceId
        expect(wrapper.emitted('reload')).toBeDefined()
        expect(wrapper.emitted('reload')?.[0][0]).toBe('service-1')
    })

    it('should have correct button order: Reload, Delete, Save', async () => {
        const tab = createMockServiceTab()
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Get all buttons in the header
        const buttons = wrapper.findAll('button').filter(b =>
            b.text().includes('Reload') ||
            b.text().includes('Delete') ||
            b.text().includes('Save')
        )

        expect(buttons.length).toBeGreaterThanOrEqual(3)

        // Check button order
        const buttonTexts = buttons.map(b => b.text())
        const reloadIndex = buttonTexts.findIndex(t => t.includes('Reload'))
        const deleteIndex = buttonTexts.findIndex(t => t.includes('Delete'))
        const saveIndex = buttonTexts.findIndex(t => t.includes('Save'))

        expect(reloadIndex).toBeLessThan(deleteIndex)
        expect(deleteIndex).toBeLessThan(saveIndex)
    })

    it('should have outline variant for reload button', async () => {
        const tab = createMockServiceTab()
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Find the reload button component
        const buttons = wrapper.findAllComponents({ name: 'Button' })
        const reloadButton = buttons.find(b => b.text().includes('Reload'))

        expect(reloadButton?.props('variant')).toBe('outline')
    })
})
