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
    import { M3 } from 'tauri-plugin-m3';
    // types
    import type { StateResponse, InfoResponse, PresetResponse } from '$lib/types/responses.ts';
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
    let loading = true;
    let powered = false;
    let brightness: number = 0;
    let loaderText = '';
    let deviceName: string = 'Unknown';
    let currentColor: string = '#ffffff';
    let currentRgb: { r: number; g: number; b: number } = { r: 255, g: 255, b: 255 };
    let currenPreset: number = -1;
    let screenWidth = window.innerWidth;
    let screenHeight = window.innerHeight;
    let infoData: InfoResponse;
    let presetData: { [x: string]: PresetResponse } | false = false;
    let availablePresets: number[];
    let storage: StorageHandler = new StorageHandler('devices.conf');
    let data: StoreData = { devices: [] };
    let tab = {
        info: false,
        light: true,
        devices: false
    };

    onMount(async () => {
        // apply material colors
        await M3.fetch("system").apply();
        await M3.barColor("system");
        // do rest of init
        await getCurrentWindow().onResized(async () => {
            if (screenWidth != window.innerWidth || screenHeight != window.innerHeight) {
                loading = true;
                await sleep(200);
                screenWidth = window.innerWidth;
                screenHeight = window.innerHeight;
                loading = false;
            }
        });
        data = await storage.open().load();
        if (data.devices.length > 0) host = data.devices[0].host;
        if (host != '') await refresh();
        loading = false;
    });

    async function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

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
        loading = true;
        powered = false;
        brightness = 0;
        deviceName = 'Unknown';
        currentColor = '#ffffff';
        currentRgb = { r: 255, g: 255, b: 255 };
        currenPreset = -1;
        let success = await getState();
        if (success) {
            await getInfo();
            await getPresets();
        }
        await sleep(200);
        loading = false;
    }

    async function getState(): Promise<boolean> {
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
            currenPreset = data.ps;
            powered = data.on;
            brightness = data.bri;
        } catch {
            console.warn(result);
            return false;
        }
        return true;
    }

    async function getInfo() {
        let result: string = await invoke('get_info', {
            host: host
        });
        try {
            infoData = JSON.parse(result);
            deviceName = infoData.name;
        } catch {
            console.warn(result);
        }
    }

    async function getPresets() {
        let result: string = await invoke('get_presets', {
            host: host
        });
        try {
            presetData = JSON.parse(result);
            if (presetData) {
                availablePresets = Object.entries(presetData)
                    .filter((entry) => entry[1].n !== undefined)
                    .map((entry) => {
                        return Number(entry[0]);
                    });
            }
        } catch {
            presetData = false;
            console.warn(result);
        }
    }

    async function setPower() {
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
    }

    async function setBrightness() {
        let result: string = await invoke('set_brightness', {
            host: host,
            brightness: brightness
        });
        if (result != 'ok') {
            console.warn(result);
        }
    }

    async function setColor() {
        let result: string = await invoke('set_color', {
            host: host,
            r: currentRgb.r,
            g: currentRgb.g,
            b: currentRgb.b
        });
        if (result != 'ok') {
            console.warn(result);
        }
    }

    async function setPreset(id: number) {
        let result: string = await invoke('set_preset', {
            host: host,
            preset: id
        });
        if (result != 'ok') {
            console.warn(result);
        }
    }

    async function presetSwitch(direction: 'up' | 'down') {
        let chosenPresetId = 0;
        let currentIndex = availablePresets.indexOf(currenPreset);
        if (direction == 'up') {
            if (currenPreset == -1) {
                chosenPresetId = availablePresets[0];
            } else {
                chosenPresetId =
                    currentIndex == availablePresets.length - 1
                        ? availablePresets[0]
                        : availablePresets[currentIndex + 1];
            }
        } else {
            if (currenPreset == -1) {
                chosenPresetId = availablePresets[0];
            } else {
                chosenPresetId =
                    currentIndex == 0
                        ? availablePresets[availablePresets.length - 1]
                        : availablePresets[currentIndex - 1];
            }
        }

        await setPreset(chosenPresetId);
    }

    async function colorChange(data: any) {
        currentColor = data.hex;
        currentRgb = data.rgb;
    }

    async function deviceChange(data: any) {
        tabSwitch('light');
        loading = true;
        host = data;
        await refresh();
        await sleep(200);
        loading = false;
    }

    async function tableChange() {
        tabSwitch('light');
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
    <div
        transition:fade={{ delay: 0, duration: 150 }}
        class="flex flex-1 flex-col justify-between items-center bg-background"
    >
        {#if tab.light}
            {#if host == ''}
                <div class="flex h-full w-full justify-center items-center">
                    <p class="font-bold text-base text-onBackground">Select/Add a Device</p>
                </div>
            {:else}
                <div class="flex flex-1 flex-col justify-center items-center space-y-10">
                    <div class="flex flex-row w-full justify-center items-center">
                        <p class="font-bold italic text-3xl align-middle text-primary">
                            {host == '' ? 'wledTR' : deviceName}
                        </p>
                    </div>
                    <Picker
                        bind:initial={currentColor}
                        width={Math.max(Math.min(Math.round(screenWidth * 0.66), 450), 100)}
                        change={colorChange}
                        end={() => {
                            setColor().then(() => {
                                refresh();
                            });
                        }}
                    />
                    <div class="flex flex-col w-2/3 justify-center items-center space-y-5 rounded-full py-4">
                        <div class="w-[80%]">
                            <input
                                type="range"
                                id="brightness-slider"
                                class="w-full h-6 bg-primary rounded-full appearance-none"
                                bind:value={brightness}
                                onchange={() => {
                                    setBrightness().then(() => {
                                        refresh();
                                    });
                                }}
                                min="1"
                                max="255"
                                step="1"
                            />
                        </div>
                    </div>
                    {#if presetData != false}
                        <div class="flex flex-row justify-center items-center space-x-6">
                            <button
                                onclick={() => {
                                    presetSwitch('down').then(() => {
                                        refresh();
                                    });
                                }}
                                class="py-2 px-4 bg-primary rounded-full active:bg-accent disabled:opacity-50 text-onPrimary font-bold text-xl"
                            >
                                &lt
                            </button>
                            <p class="text-onBackground uppercase text-xl">
                                {currenPreset == -1 ? 'Inactive' : presetData[currenPreset].n}
                            </p>
                            <button
                                onclick={() => {
                                    presetSwitch('up').then(() => {
                                        refresh();
                                    });
                                }}
                                class="py-2 px-4 bg-primary rounded-full active:bg-accent disabled:opacity-50 text-onPrimary font-bold text-xl"
                            >
                                &gt
                            </button>
                        </div>
                    {/if}
                    <div class="flex flex-row justify-center items-center space-x-6">
                        <button
                            class="p-4 bg-primary rounded-full active:bg-accent disabled:opacity-50"
                            onclick={() => {
                                setPower().then(() => {
                                    refresh();
                                });
                            }}
                        >
                            <img
                                width="48"
                                height="48"
                                class="dark:invert"
                                src={powered ? FireSolid : FireOutline}
                                alt=""
                            />
                        </button>
                    </div>
                </div>
            {/if}
        {:else if tab.devices}
            <DeviceTable select={deviceChange} change={tableChange} />
        {:else if tab.info}
            {#if host == ''}
                <div class="flex w-full h-full justify-center items-center">
                    <p class="font-bold text-base text-onBackground">Select/Add a Device</p>
                </div>
            {:else}
                <InfoTable bind:data={infoData} />
            {/if}
        {/if}
        <div class="flex w-full justify-center min-h-[6.5rem] max-h-[6.5rem] bg-surfaceVariant">
            <div class="flex flex-1 flex-row justify-between my-auto">
                <div>
                    <button
                        style="opacity: {tab.devices ? '100%' : '50%'};"
                        onclick={() => tabSwitch('devices')}
                        class="w-14 mx-10"
                        ><div class="rounded-full p-1 {tab.devices ? 'bg-primary' : 'bg-transparent'}">
                            <img
                                width="24"
                                height="24"
                                class="mx-auto dark:invert"
                                src={tab.devices ? BookmarkSolid : BookmarkOutline}
                                alt=""
                            />
                        </div>
                        <p class="text-onSurfaceVariant">Devices</p></button
                    >
                </div>
                <div>
                    <button
                        style="opacity: {tab.light ? '100%' : '50%'};"
                        onclick={() => tabSwitch('light')}
                        class="w-14 mx-2"
                        ><div class="rounded-full p-1 {tab.light ? 'bg-primary' : 'bg-transparent'}">
                            <img
                                width="24"
                                height="24"
                                class="mx-auto dark:invert"
                                src={tab.light ? LightSolid : LightOutline}
                                alt=""
                            />
                        </div>
                        <p class="text-onSurfaceVariant">Light</p></button
                    >
                </div>
                <div>
                    <button
                        style="opacity: {tab.info ? '100%' : '50%'};"
                        onclick={() => tabSwitch('info')}
                        class="w-14 mx-10"
                        ><div class="rounded-full p-1 {tab.info ? 'bg-primary' : 'bg-transparent'}">
                            <img
                                width="24"
                                height="24"
                                class="mx-auto dark:invert"
                                src={tab.info ? SettingsSolid : SettingsOutline}
                                alt=""
                            />
                        </div>
                        <p class="text-onSurfaceVariant">Info</p>
                    </button>
                </div>
            </div>
        </div>
    </div>
{/if}
