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

    const dispatch = createEventDispatcher();
    let loading = false;
    let addingNew: boolean = false;
    let newInput: string = '';
    let checkDevices: Set<Device> = new Set();
    let data: StoreData = { devices: [] };
    let storage: StorageHandler = new StorageHandler('devices.conf');

    onMount(async () => {
        data = await storage.open().load();
        loading = true;
        loading = false;
    });

    function dispatchSelect(host: string) {
        dispatch('select', {
            host: host
        });
    }

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
        return /^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$/.test(input);
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
        if (checkIsValid) dispatch('change');
    }

    async function deleteDevices(devices: Set<Device>) {
        loading = true;
        (await storage.open().remove().multiple(Array.from(devices.values()))).save();
        checkDevices.clear();
        data = await storage.open().load();
        await sleep(200);
        loading = false;
        dispatch('change');
    }
</script>

{#if loading}
    <div class="flex h-full justify-center items-center">
        <Spinner text="" />
    </div>
{:else if data.devices.length < 1 && !addingNew}
    <div class="flex flex-col space-y-4 h-full justify-center items-center">
        <p class="m-4">No Devices Found :&lbrace;</p>
        <button
            class="border-2 rounded-lg p-2 border-blue-900 text-white text-2xl active:bg-accent disabled:opacity-50"
            on:click={() => {
                addingNew = true;
            }}
            disabled={addingNew}>Add</button
        >
    </div>
{:else}
    <div class="flex flex-col justify-center items-center space-y-4 my-4">
        <table>
            <tbody>
                {#each data.devices as device}
                    <tr>
                        <td class="text-left text-2xl p-2">
                            <button
                                class="active:bg-accent disabled:opacity-50"
                                on:click={() => dispatchSelect(device.host)}
                            >
                                {device.host}
                            </button>
                            <input
                                type="checkbox"
                                class="ml-4 scale-[3.0] -translate-y-[0.2rem] checked:scale-[2.0]"
                                on:click={() => updateCheckedDevices(device)}
                                checked={checkDevices.has(device)}
                            />
                        </td>
                    </tr>
                {/each}
                {#if addingNew}
                    <tr class="text-left">
                        <td class="text-xl">
                            <!-- svelte-ignore a11y-autofocus -->
                            <input
                                id="newInput"
                                class="bg-[rgba(0,0,0,0)] border-l-4 border-r-4"
                                bind:value={newInput}
                                autofocus
                            />
                            <button
                                class="w-3 h-3 p-3 m-0 text-3xl active:bg-accent disabled:opacity-50"
                                on:click={() => addDevice({ host: newInput, mdns: false })}
                                disabled={!validIP(newInput)}>+</button
                            >
                        </td>
                    </tr>
                {/if}
            </tbody>
        </table>
        {#if checkDevices.size > 0}
            <button
                class="border-2 rounded-lg p-2 border-red-800 text-white text-2xl active:bg-accent disabled:opacity-50"
                on:click={() => deleteDevices(checkDevices)}>Delete ({checkDevices.size})</button
            >
        {:else}
            <button
                class="border-2 rounded-lg p-2 border-blue-900 text-white text-2xl active:bg-accent disabled:opacity-50"
                on:click={() => {
                    addingNew = true;
                }}
                disabled={addingNew}>Add</button
            >
        {/if}
    </div>
{/if}
