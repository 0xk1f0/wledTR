<script lang="ts">
    import { Button } from '$lib/components/ui/button/index.js';
    import Switch from '$lib/components/ui/switch/switch.svelte';
    import Label from '../ui/label/label.svelte';
    import Loader from './Loader.svelte';
    import Picker from './Picker.svelte';
    import { fade } from 'svelte/transition';
    import { onMount } from 'svelte';
    import { getCurrent } from '@tauri-apps/api/window';
    import { invoke } from '@tauri-apps/api/core';

    // @TODO: Dont make this hardcoded
    const HOST = "1.2.3.4"

    let loading = false;
    let powered = false;
    let loaderText = 'Loading';
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
    });

    async function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    async function setPower() {
        loaderText = '';
        loading = true;
        let result: string = await invoke('power_toggle', {
            host: HOST
        });
        if (result == 'on') {
            powered = true;
        } else {
            powered = false;
        }
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
    <div
        transition:fade={{ delay: 0, duration: 150 }}
        class="flex flex-1 flex-col justify-between items-center border-2 m-2.5 rounded-xl"
    >
        <div>
            <p class="font-bold text-2xl mt-6">wledTR</p>
        </div>
        <div class="flex flex-1 flex-col justify-center items-center space-y-10">
            <Picker width={Math.max(Math.min(Math.round(screenWidth * 0.66), 450), 100)} on:color={colorChange} />
            <p class="text-lg font-bold uppercase" style="color: {currentColor}">{currentColor}</p>
            <div class="flex flex-col justify-center items-center space-y-5">
                <Switch class="scale-[175%]" id="power-switch" bind:checked={powered} on:click={setPower} />
                <Label class="font-bold text-xl" for="power-switch">{powered ? 'ON' : 'OFF'}</Label>
            </div>
            <Button
                style="width: {Math.max(Math.min(Math.round(screenWidth * 0.33), 450), 100)}px"
                class="font-semibold h-16 text-xl border-[3px]"
                size="icon"
                variant="outline"
                on:click={setColor}>Apply</Button
            >
        </div>
        <div>
            <p class="font-bold text-sm mb-6 italic text-gray-600 opacity-75">Made by 0xk1f0</p>
        </div>
    </div>
{/if}
