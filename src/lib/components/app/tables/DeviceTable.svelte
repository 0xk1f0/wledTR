<script lang="ts">
    // svelte
    import { onMount, createEventDispatcher } from 'svelte';
    // components
    import Spinner from '$lib/components/app/modules/Spinner.svelte';
    // tauri
    import { invoke } from '@tauri-apps/api/core';
    // types
    import type { Device, StoreData } from '$lib/types/store.ts';
    // utils
    import StorageHandler from '$lib/util/storage';
    // icons
    import TrashOutline from '$lib/assets/trash-outline.svg';
    import TrashSolid from '$lib/assets/trash-solid.svg';

    let { select, change } = $props();
    let loading = $state(false);
    let addingNew: boolean = $state(false);
    let newInput: string = $state('');
    let checkDevices: Set<Device> = $state(new Set());
    let data: StoreData = $state({ devices: [] });
    let storage: StorageHandler = new StorageHandler('devices.conf');

    onMount(async () => {
        loading = true;
        data = await storage.open().load();
        loading = false;
    });

    function updateCheckedDevices(device: Device) {
        if (checkDevices.has(device)) {
            checkDevices.delete(device);
        } else {
            checkDevices.add(device);
        }
        // svelte reinstate fix
        checkDevices = checkDevices;
    }

    function validIP(input: string): boolean {
        const RES = /^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$/.test(input);
        return RES;
    }

    async function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    async function checkInfo(host: string): Promise<string | false> {
        loading = true;
        let result: string = await invoke('check_info', {
            host: host
        });
        if (!validIP(result) || result != host) {
            console.warn(result);
            return false;
        }
        await sleep(200);
        loading = false;
        return host;
    }

    async function addDevice(device: Device) {
        loading = true;
        let checkIsValid = await checkInfo(device.host);
        if (checkIsValid) (await storage.open().add().single(device)).save();
        addingNew = false;
        newInput = '';
        data = await storage.open().load();
        await sleep(200);
        loading = false;
        if (checkIsValid) change();
    }

    async function deleteDevices(devices: Set<Device>) {
        loading = true;
        (await storage.open().remove().multiple(Array.from(devices.values()))).save();
        checkDevices.clear();
        data = await storage.open().load();
        await sleep(200);
        loading = false;
        change();
    }
</script>

{#if loading}
    <div class="flex h-full justify-center items-center">
        <Spinner text="" />
    </div>
{:else if data.devices.length < 1 && !addingNew}
    <div class="m-auto flex flex-col space-y-4 justify-center items-center max-h-[calc(100vh-12rem)] overflow-scroll">
        <p class="m-4 font-bold text-base text-onBackground">No Devices Found</p>
        <button
            class="rounded-full px-7 py-2 bg-primary text-onPrimary text-xl disabled:opacity-50"
            onclick={() => {
                addingNew = true;
            }}
            disabled={addingNew}>Add</button
        >
    </div>
{:else}
    <div class="m-auto flex flex-col justify-center items-center space-y-4 max-h-[calc(100vh-12rem)] overflow-scroll">
        <table>
            <tbody>
                {#each data.devices as device}
                    <tr>
                        <td class="text-2xl">
                            <div
                                class="flex flex-row space-x-2 justify-center bg-surfaceVariant text-onSurfaceVariant pl-3 rounded-full my-1"
                            >
                                <button
                                    class="disabled:opacity-50 rounded-full"
                                    onclick={() => select(device.host)}
                                >
                                    {device.host}
                                </button>
                                <button
                                    class="{checkDevices.has(device)
                                        ? 'bg-secondaryContainer'
                                        : 'bg-transparent'} text-2xl p-2 rounded-full"
                                    onclick={() => updateCheckedDevices(device)}
                                    ><img
                                        width="28"
                                        height="28"
                                        src={checkDevices.has(device) ? TrashSolid : TrashOutline}
                                        alt=""
                                    /></button
                                >
                            </div>
                        </td>
                    </tr>
                {/each}
                {#if addingNew}
                    <tr class="text-left">
                        <td>
                            <!-- svelte-ignore a11y_autofocus -->
                            <input
                                id="newInput"
                                class="bg-transparent border-2 border-onBackground rounded-full px-4 py-1 text-center text-onBackground text-2xl font-medium outline-none"
                                bind:value={newInput}
                                autofocus
                            />
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
        {#if checkDevices.size > 0}
            <button
                class="rounded-full px-7 py-2 bg-secondaryContainer text-onSecondaryContainer text-xl disabled:opacity-50"
                onclick={() => deleteDevices(checkDevices)}>Delete ({checkDevices.size})</button
            >
        {:else}
            <button
                class="rounded-full px-7 py-2 bg-primary text-onPrimary text-xl disabled:opacity-50"
                onclick={() => {
                    if (addingNew && validIP(newInput)) {
                        addDevice({ host: newInput, mdns: false });
                        addingNew = false;
                    } else {
                        addingNew = true;
                    }
                }}
                disabled={!validIP(newInput) && addingNew}>Add</button
            >
        {/if}
    </div>
{/if}
