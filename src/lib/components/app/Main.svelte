<script lang="ts">
    // components
    import Loader from '$lib/components/app/modules/Loader.svelte';
    import Picker from '$lib/components/app/modules/Picker.svelte';
    import InfoTable from '$lib/components/app/tables/InfoTable.svelte';
    import DeviceTable from '$lib/components/app/tables/DeviceTable.svelte';
    // svelte
    import { fade } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { getCurrentWindow } from '@tauri-apps/api/window';
    // tauri
    import { invoke } from '@tauri-apps/api/core';
    // types
    import type { StateResponse, InfoResponse } from '$lib/types/responses.ts';
    import type { StoreData } from '$lib/types/store.ts';
    // utils
    import StorageHandler from '$lib/util/storage';
    // icons
    import CheckmarkOutline from '$lib/assets/checkmark-outline.svg';
    import BookmarkOutline from '$lib/assets/bookmark-outline.svg';
    import BookmarkSolid from '$lib/assets/bookmark-solid.svg';
    import LightOutline from '$lib/assets/light-outline.svg';
    import LightSolid from '$lib/assets/light-solid.svg';
    import SettingsOutline from '$lib/assets/settings-outline.svg';
    import SettingsSolid from '$lib/assets/settings-solid.svg';
    import FireOutline from '$lib/assets/fire-outline.svg';
    import FireSolid from '$lib/assets/fire-solid.svg';

    let host = '';
    let loading = false;
    let powered = false;
    let brightness: number = 0;
    let loaderText = 'Loading';
    let deviceName: string = 'Unknown';
    let currentColor: string = '#ffffff';
    let currentRgb: { r: number; g: number; b: number } = { r: 255, g: 255, b: 255 };
    let screenWidth = window.innerWidth;
    let screenHeight = window.innerHeight;
    let infoData: InfoResponse;
    let storage: StorageHandler = new StorageHandler('devices.conf');
    let data: StoreData = { devices: [] };
    let tab = {
        info: false,
        light: true,
        devices: false
    };

    onMount(async () => {
        await getCurrentWindow().onResized(async () => {
            if (screenWidth != window.innerWidth || screenHeight != window.innerHeight) {
                loaderText = 'Loading';
                loading = true;
                await sleep(500);
                screenWidth = window.innerWidth;
                screenHeight = window.innerHeight;
                loading = false;
            }
        });
        data = await storage.open().load();
        if (data.devices.length > 0) host = data.devices[0].host;
        if (host != '') await refresh();
    });

    function toHex(c: number) {
        const hex = c.toString(16);
        return hex.length === 1 ? '0' + hex : hex;
    }

    function tabSwitch(type: string) {
        // reset all
        tab = {
            info: false,
            light: false,
            devices: false
        };
        // then eval
        switch (type) {
            case 'info':
                tab.info = true;
                break;
            case 'light':
                tab.light = true;
                break;
            case 'devices':
                tab.devices = true;
                break;
        }
    }

    async function refresh() {
        powered = false;
        brightness = 0;
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
            brightness = data.bri;
        } catch {
            console.warn(result);
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
            console.warn(result);
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
            console.warn(result);
        }
        await sleep(200);
        loading = false;
    }

    async function setBrightness() {
        loaderText = '';
        loading = true;
        let result: string = await invoke('set_brightness', {
            host: host,
            brightness: brightness
        });
        if (result != 'ok') {
            console.warn(result);
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
            console.warn(result);
        }
        await sleep(200);
        loading = false;
    }

    async function colorChange(event: any) {
        currentColor = event.detail.hex;
        currentRgb = event.detail.rgb;
    }

    async function deviceChange(event: any) {
        tabSwitch('light');
        loaderText = '';
        loading = true;
        host = event.detail.host;
        await refresh();
        await sleep(200);
        loading = false;
    }

    async function tableChange() {
        tabSwitch('light');
        loaderText = '';
        loading = true;
        host = '';
        data = await storage.open().load();
        if (data.devices.length > 0) host = data.devices[0].host;
        if (host != '') await refresh();
        await sleep(200);
        loading = false;
    }
</script>

{#if loading}
    <Loader text={loaderText} />
{:else}
    <div transition:fade={{ delay: 0, duration: 150 }} class="flex flex-1 flex-col justify-between items-center">
        {#if tab.light}
            {#if host == ''}
                <div class="flex h-full w-full justify-center items-center">
                    <p class="font-bold text-base">Select/Add a Device</p>
                </div>
            {:else}
                <div class="flex flex-row w-full justify-center items-center my-8 pt-[2vh]">
                    <p class="font-bold text-3xl align-middle">{host == '' ? 'wledTR' : deviceName}</p>
                </div>
                <div class="flex flex-1 flex-col justify-center items-center space-y-10">
                    <Picker
                        bind:initial={currentColor}
                        width={Math.max(Math.min(Math.round(screenWidth * 0.66), 450), 100)}
                        on:color={colorChange}
                    />
                    <div
                        class="flex flex-col w-3/4 justify-center items-center space-y-5 rounded-full py-4"
                    >
                        <p class="rounded-full text-2xl font-bold font-mono uppercase" style="color: {currentColor}">
                            {currentColor}
                        </p>
                        <div class="w-3/4">
                            <input
                                type="range"
                                id="brightness-slider"
                                class="w-full h-2 bg-blue-800 bg-opacity-50 rounded-full appearance-none"
                                bind:value={brightness}
                                min="1"
                                max="255"
                                step="1"
                            />
                        </div>
                        <label class="font-bold font-mono text-2xl" for="brightness-slider"
                            >{Math.round((brightness * 100) / 255)}%</label
                        >
                    </div>
                    <div class="flex flex-row justify-center items-center space-x-4">
                        <button
                            class="p-4 bg-blue-600 bg-opacity-25 rounded-full active:bg-accent disabled:opacity-50"
                            onclick={setPower}
                        >
                            <img width="48" height="48" src={powered ? FireSolid : FireOutline} alt="" />
                        </button>
                        <button
                            class="p-4 bg-blue-600 bg-opacity-25 rounded-full active:bg-accent disabled:opacity-50"
                            disabled={!powered}
                            onclick={() => {
                                setColor();
                                setBrightness();
                            }}
                            ><img width="48" height="48" src={CheckmarkOutline} alt="" />
                        </button>
                    </div>
                </div>
            {/if}
        {:else if tab.devices}
            <DeviceTable on:select={deviceChange} on:change={tableChange} />
        {:else if tab.info}
            {#if host == ''}
                <div class="flex w-full justify-center items-center h-[calc(100vh-8rem)]">
                    <p class="font-bold text-base">Select/Add a Device</p>
                </div>
            {:else}
                <InfoTable bind:data={infoData} />
            {/if}
        {/if}
        <div class="flex w-full justify-center min-h-[5.25rem] max-h-[5.25rem] bg-blue-500 bg-opacity-5 pb-[2vh]">
            <div class="flex flex-1 flex-row justify-between my-auto">
                <div>
                    <button
                        style="opacity: {tab.devices ? '100%' : '50%'};"
                        onclick={() => tabSwitch('devices')}
                        class="w-14 mx-10"
                        ><div class="bg-opacity-50 rounded-full p-1 {tab.devices ? 'bg-blue-600' : 'bg-transparent'}">
                            <img
                                width="24"
                                height="24"
                                class="mx-auto"
                                src={tab.devices ? BookmarkSolid : BookmarkOutline}
                                alt=""
                            />
                        </div>
                        <p>Devices</p></button
                    >
                </div>
                <div>
                    <button
                        style="opacity: {tab.light ? '100%' : '50%'};"
                        onclick={() => tabSwitch('light')}
                        class="w-14 mx-2"
                        ><div class="bg-opacity-50 rounded-full p-1 {tab.light ? 'bg-blue-600' : 'bg-transparent'}">
                            <img
                                width="24"
                                height="24"
                                class="mx-auto"
                                src={tab.light ? LightSolid : LightOutline}
                                alt=""
                            />
                        </div>
                        <p>Light</p></button
                    >
                </div>
                <div>
                    <button
                        style="opacity: {tab.info ? '100%' : '50%'};"
                        onclick={() => tabSwitch('info')}
                        class="w-14 mx-10"
                        ><div class="bg-opacity-50 rounded-full p-1 {tab.info ? 'bg-blue-600' : 'bg-transparent'}">
                            <img
                                width="24"
                                height="24"
                                class="mx-auto"
                                src={tab.info ? SettingsSolid : SettingsOutline}
                                alt=""
                            />
                        </div>
                        <p>Info</p>
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
