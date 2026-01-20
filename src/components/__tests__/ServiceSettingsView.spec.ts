import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import ServiceSettingsView from '../ServiceSettingsView.vue'

describe('ServiceSettingsView - isUnsafe Flag', () => {
    const createMockTab = (isUnsafe = false) => ({
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
                    variables: [
                        { name: 'BASE_URL', value: 'http://localhost:3000' }
                    ]
                },
                {
                    name: 'STAGING',
                    isUnsafe: isUnsafe,
                    variables: [
                        { name: 'BASE_URL', value: 'https://staging.example.com' }
                    ]
                },
                {
                    name: 'PRODUCTION',
                    isUnsafe: true,
                    variables: [
                        { name: 'BASE_URL', value: 'https://api.example.com' }
                    ]
                }
            ]
        }
    })

    beforeEach(() => {
        vi.clearAllMocks()
    })

    it('should display isUnsafe flag correctly for each environment', async () => {
        const tab = createMockTab(false)
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Find all environment headers
        const envHeaders = wrapper.findAll('th[class*="min-w-[150px]"]')

        // DEV environment should show green indicator (not unsafe)
        const devHeader = envHeaders[0]
        const devIndicator = devHeader.find('span[class*="w-1.5 h-1.5"]')
        expect(devIndicator.classes()).toContain('bg-green-500')
        expect(devIndicator.classes()).not.toContain('bg-destructive')

        // STAGING environment should show green indicator (not unsafe in this test)
        const stagingHeader = envHeaders[1]
        const stagingIndicator = stagingHeader.find('span[class*="w-1.5 h-1.5"]')
        expect(stagingIndicator.classes()).toContain('bg-green-500')

        // PRODUCTION environment should show red indicator (unsafe)
        const prodHeader = envHeaders[2]
        const prodIndicator = prodHeader.find('span[class*="w-1.5 h-1.5"]')
        expect(prodIndicator.classes()).toContain('bg-destructive')
        expect(prodIndicator.classes()).not.toContain('bg-green-500')
    })

    it('should toggle isUnsafe flag when switch is clicked', async () => {
        const tab = createMockTab(false)
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Get the STAGING environment (index 1)
        const stagingEnv = tab.serviceData.environments[1]
        expect(stagingEnv.isUnsafe).toBe(false)

        // Find the switch for STAGING environment
        const switches = wrapper.findAllComponents({ name: 'Switch' })
        const stagingSwitch = switches[1] // Second switch is for STAGING

        // Toggle the switch
        await stagingSwitch.vm.$emit('update:checked', true)
        await wrapper.vm.$nextTick()

        // Verify the flag was updated
        expect(stagingEnv.isUnsafe).toBe(true)

        // Toggle it back
        await stagingSwitch.vm.$emit('update:checked', false)
        await wrapper.vm.$nextTick()

        expect(stagingEnv.isUnsafe).toBe(false)
    })

    it('should preserve isUnsafe flag when saving', async () => {
        const tab = createMockTab(true)

        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Click save button
        const saveButton = wrapper.findAll('button').find(b => b.text().includes('Save Changes'))
        await saveButton?.trigger('click')

        // Verify save event was emitted with the tab data including isUnsafe flags
        expect(wrapper.emitted('save')).toBeDefined()
        expect(wrapper.emitted('save')?.[0][0]).toStrictEqual(tab)

        expect(tab.serviceData.environments[1].isUnsafe).toBe(true)
        expect(tab.serviceData.environments[2].isUnsafe).toBe(true)
    })


    it('should handle missing isUnsafe field gracefully', async () => {
        const tab = createMockTab()
        // Remove isUnsafe from one environment to simulate legacy data
        delete (tab.serviceData.environments[0] as any).isUnsafe

        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Should not crash and should treat missing field as false
        const envHeaders = wrapper.findAll('th[class*="min-w-[150px]"]')
        const devHeader = envHeaders[0]
        const devIndicator = devHeader.find('span[class*="w-1.5 h-1.5"]')

        // Should show green (safe) indicator when isUnsafe is undefined
        expect(devIndicator.exists()).toBe(true)
    })

    it('should display "Prod Warn" label for each environment', async () => {
        const tab = createMockTab()
        const wrapper = mount(ServiceSettingsView, {
            props: {
                tab,
                gitStatus: null
            }
        })

        await wrapper.vm.$nextTick()

        // Find all "Prod Warn" labels
        const prodWarnLabels = wrapper.findAll('span[class*="text-[9px]"]').filter(
            span => span.text().includes('Prod Warn')
        )

        // Should have one label per environment
        expect(prodWarnLabels.length).toBe(3)
    })
})
