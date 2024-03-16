<script lang="ts">
    // shadcdn
    import { Button } from '$lib/components/ui/button/index.js';
    import Switch from '$lib/components/ui/switch/switch.svelte';
    import { Slider } from '$lib/components/ui/slider';
    import { toast } from 'svelte-sonner';
    // components
    import Label from '../ui/label/label.svelte';
    import Loader from './modules/Loader.svelte';
    import Picker from './modules/Picker.svelte';
    import SideMenu from './menus/SideMenu.svelte';
    import InfoTable from './tables/InfoTable.svelte';
    import DeviceTable from './tables/DeviceTable.svelte';
    // svelte
    import { fade } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { getCurrent } from '@tauri-apps/api/window';
    // tauri
    import { invoke } from '@tauri-apps/api/core';
    import { Store } from '@tauri-apps/plugin-store';
    // types
    import type { StateResponse, InfoResponse } from '../../types/responses.ts';
    import type { Device, StoreData } from '$lib/types/store.ts';

    let host = '';
    let loading = false;
    let powered = false;
    let dataStore: Store;
    let brightness: number[] = [0];
    let loaderText = 'Loading';
    let deviceName: string = 'Unknown';
    let currentColor: string = '#ffffff';
    let currentRgb: { r: number; g: number; b: number } = { r: 255, g: 255, b: 255 };
    let screenWidth = window.innerWidth;
    let screenHeight = window.innerHeight;
    let infoData: InfoResponse;
    let menus = {
        left: false,
        right: false
    };

    onMount(async () => {
        await getCurrent().onResized(async () => {
            if (screenWidth != window.innerWidth || screenHeight != window.innerHeight) {
                loaderText = 'Loading';
                loading = true;
                await sleep(500);
                screenWidth = window.innerWidth;
                screenHeight = window.innerHeight;
                loading = false;
            }
        });
        dataStore = new Store('devices.conf');
        let storeData = await dataStore.get<StoreData>('devices');
        if (storeData != null && storeData.devices.length > 0) host = storeData.devices[0].host;
        if (host != '') await refresh();
    });

    function toHex(c: number) {
        const hex = c.toString(16);
        return hex.length === 1 ? '0' + hex : hex;
    }

    async function refresh() {
        powered = false;
        brightness = [0];
        deviceName = 'Unknown';
        currentColor = '#ffffff';
        currentRgb = { r: 255, g: 255, b: 255 };
        let success = await getState();
        if (success) await getInfo();
    }

    async function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    async function getState(): Promise<boolean> {
        loaderText = '';
        loading = true;
        let result: string = await invoke('get_state', {
            host: host
        });
        try {
            let data: StateResponse = JSON.parse(result);
            currentRgb = {
                r: data.seg[0].col[0][0],
                g: data.seg[0].col[0][1],
                b: data.seg[0].col[0][2]
            };
            currentColor = '#' + toHex(currentRgb.r) + toHex(currentRgb.g) + toHex(currentRgb.b);
            powered = data.on;
            brightness = [data.bri];
        } catch {
            toast.warning('Action Failed', {
                description: result
            });
            return false;
        }
        await sleep(200);
        loading = false;
        return true;
    }

    async function getInfo() {
        loaderText = '';
        loading = true;
        let result: string = await invoke('get_info', {
            host: host
        });
        try {
            infoData = JSON.parse(result);
            deviceName = infoData.name;
        } catch {
            toast.warning('Action Failed', {
                description: result
            });
        }
        await sleep(200);
        loading = false;
    }

    async function setPower() {
        loaderText = '';
        loading = true;
        let result: string = await invoke('power_toggle', {
            host: host
        });
        if (result == 'on') {
            powered = true;
        } else if (result == 'off') {
            powered = false;
        } else {
            toast.warning('Action Failed', {
                description: result
            });
        }
        await sleep(200);
        loading = false;
    }

    async function setBrightness() {
        loaderText = '';
        loading = true;
        console.log('Trigger');
        let result: string = await invoke('set_brightness', {
            host: host,
            brightness: brightness[0]
        });
        if (result != 'ok') {
            toast.warning('Action Failed', {
                description: result
            });
        }
        await sleep(200);
        loading = false;
    }

    async function setColor() {
        loaderText = 'Applying';
        loading = true;
        let result: string = await invoke('set_color', {
            host: host,
            r: currentRgb.r,
            g: currentRgb.g,
            b: currentRgb.b
        });
        if (result != 'ok') {
            toast.warning('Action Failed', {
                description: result
            });
        }
        await sleep(200);
        loading = false;
    }

    async function colorChange(event: any) {
        currentColor = event.detail.hex;
        currentRgb = event.detail.rgb;
    }

    async function deviceChange(event: any) {
        menus.left = false;
        loaderText = '';
        loading = true;
        host = event.detail.host;
        await refresh();
        await sleep(200);
        loading = false;
    }
</script>

{#if loading}
    <Loader text={loaderText} />
{:else}
    <div transition:fade={{ delay: 0, duration: 150 }} class="flex flex-1 flex-col justify-between items-center">
        <div class="flex flex-row w-full justify-between items-center mt-3">
            <div class="ml-3">
                <SideMenu bind:open={menus.left} side="left" title="Lights">
                    <DeviceTable on:select={deviceChange} />
                </SideMenu>
            </div>
            {#if host != ''}
                <p class="font-bold text-3xl align-middle">{deviceName}</p>
                <div class="mr-3">
                    <SideMenu bind:open={menus.right} side="right" title="Info">
                        <InfoTable bind:data={infoData} />
                    </SideMenu>
                </div>
            {/if}
        </div>
        {#if host == ''}
            <div class="flex h-full w-full justify-center items-center">
                <p>Select a Device</p>
            </div>
        {:else}
            <div class="flex flex-1 flex-col justify-center items-center space-y-10">
                <Picker
                    bind:initial={currentColor}
                    width={Math.max(Math.min(Math.round(screenWidth * 0.66), 450), 100)}
                    on:color={colorChange}
                />
                <p class="text-lg font-bold uppercase" style="color: {currentColor}">{currentColor}</p>
                <div class="flex flex-col w-3/4 justify-center items-center space-y-5">
                    <Slider id="brightness-slider" bind:value={brightness} max={255} step={1} />
                    <Label class="font-bold text-xl" for="brightness-slider"
                        >{brightness[0]} ({Math.round((brightness[0] * 100) / 255)}%)</Label
                    >
                </div>
                <div class="flex flex-col justify-center items-center space-y-5">
                    <Switch class="scale-[175%]" id="power-switch" bind:checked={powered} on:click={setPower} />
                    <Label class="font-bold text-xl" for="power-switch">{powered ? 'ON' : 'OFF'}</Label>
                </div>
                <Button
                    style="width: {Math.max(Math.min(Math.round(screenWidth * 0.33), 450), 100)}px"
                    class="font-semibold h-16 text-xl border-[3px]"
                    size="icon"
                    variant="outline"
                    disabled={!powered}
                    on:click={() => {
                        setColor();
                        setBrightness();
                    }}>Apply</Button
                >
            </div>
            <div>
                <p class="sticky bottom-0 left-1/2 font-bold text-sm mb-6 italic text-gray-600 opacity-75">wledTR</p>
            </div>
        {/if}
    </div>
{/if}
