<script lang="ts">
    // shadcdn
    import { Button } from '$lib/components/ui/button/index.js';
    import Switch from '$lib/components/ui/switch/switch.svelte';
    import { Slider } from '$lib/components/ui/slider';
    import { toast } from 'svelte-sonner';
    // components
    import Label from '../ui/label/label.svelte';
    import Loader from './Loader.svelte';
    import Picker from './Picker.svelte';
    // svelte
    import { fade } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { getCurrent } from '@tauri-apps/api/window';
    // tauri
    import { invoke } from '@tauri-apps/api/core';
    // types
    import type { StateResponse, InfoResponse } from '../../types/responses.ts';

    // @TODO: Dont make this hardcoded
    const HOST = '1.2.3.4';

    let loading = false;
    let powered = false;
    let brightness: number[] = [0];
    let loaderText = 'Loading';
    let deviceName: string = 'Unknown';
    let currentColor: string = '#ffffff';
    let currentRgb: { r: number; g: number; b: number } = { r: 255, g: 255, b: 255 };
    let screenWidth = window.innerWidth;
    let screenHeight = window.innerHeight;

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
        await getState();
        await getInfo();
    });

    function toHex(c: number) {
        const hex = c.toString(16);
        return hex.length === 1 ? '0' + hex : hex;
    }

    async function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    async function getState() {
        loaderText = '';
        loading = true;
        let result: string = await invoke('get_state', {
            host: HOST
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
        }
        await sleep(200);
        loading = false;
    }

    async function getInfo() {
        loaderText = '';
        loading = true;
        let result: string = await invoke('get_info', {
            host: HOST
        });
        try {
            let data: InfoResponse = JSON.parse(result);
            deviceName = data.name;
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
            host: HOST
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
            host: HOST,
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
            host: HOST,
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
</script>

{#if loading}
    <Loader text={loaderText} />
{:else}
    <div transition:fade={{ delay: 0, duration: 150 }} class="flex flex-1 flex-col justify-between items-center">
        <div>
            <p class="font-bold text-2xl mt-6">{deviceName}</p>
        </div>
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
            <p class="font-bold text-sm mb-6 italic text-gray-600 opacity-75">wledTR - Made by 0xk1f0</p>
        </div>
    </div>
{/if}
