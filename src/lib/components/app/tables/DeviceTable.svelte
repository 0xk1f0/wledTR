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

    const dispatch = createEventDispatcher();
    let loading = false;
    let addingNew: boolean = false;
    let validAddress: boolean = false;
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
        const RES = /^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$/.test(input);
        validAddress = RES;
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
            class="rounded-full px-7 py-2 bg-blue-400 bg-opacity-75 text-white text-xl active:bg-accent disabled:opacity-50"
            on:click={() => {
                addingNew = true;
            }}
            disabled={addingNew}>Add</button
        >
    </div>
{:else}
    <div class="flex flex-col justify-center items-center space-y-4">
        <table>
            <tbody>
                {#each data.devices as device}
                    <tr>
                        <td class="text-2xl">
                            <div
                                class="flex flex-row space-x-2 justify-center bg-blue-300 bg-opacity-20 pl-3 rounded-full my-1"
                            >
                                <button
                                    class="active:bg-accent disabled:opacity-50 rounded-full"
                                    on:click={() => dispatchSelect(device.host)}
                                >
                                    {device.host}
                                </button>
                                <button
                                    class="{checkDevices.has(device)
                                        ? 'bg-red-500'
                                        : 'bg-transparent'} text-2xl p-2 rounded-full"
                                    on:click={() => updateCheckedDevices(device)}
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
                            <!-- svelte-ignore a11y-autofocus -->
                            <input
                                id="newInput"
                                class="bg-transparent border-2 rounded-full px-4 py-1 text-center text-2xl"
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
                class="rounded-full px-7 py-2 bg-red-500 bg-opacity-75 text-white text-xl active:bg-accent disabled:opacity-50"
                on:click={() => deleteDevices(checkDevices)}>Delete ({checkDevices.size})</button
            >
        {:else}
            <button
                class="rounded-full px-7 py-2 bg-blue-400 bg-opacity-75 text-white text-xl active:bg-accent disabled:opacity-50"
                on:click={() => {
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
