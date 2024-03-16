<script lang="ts">
    import * as Table from '$lib/components/ui/table';
    import { Checkbox } from '$lib/components/ui/checkbox';
    import Button from '$lib/components/ui/button/button.svelte';
    import { onMount, createEventDispatcher } from 'svelte';
    import { Store } from '@tauri-apps/plugin-store';
    import Spinner from '../modules/Spinner.svelte';
    import { toast } from 'svelte-sonner';
    import { invoke } from '@tauri-apps/api/core';
    import type { Device, StoreData } from '$lib/types/store.ts';

    let dataStore: Store;
    let loading = false;
    let addingNew: boolean = false;
    let newInput: string = '';
    let data: Device[] = [];
    let checkDevices: Set<Device> = new Set();
    const dispatch = createEventDispatcher();

    onMount(async () => {
        loading = true;
        dataStore = new Store('devices.conf');
        let storeData = await dataStore.get<StoreData>('devices');
        if (storeData != null) data = storeData.devices;
        loading = false;
    });

    async function dispatchSelect(host: string) {
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
            toast.warning('Action Failed', {
                description: result
            });
            return false;
        }
        await sleep(200);
        loading = false;
        return host;
    }

    async function addDevice(device: Device) {
        loading = true;
        if (await checkInfo(device.host) != false) {
            data.push(device);
            await dataStore.set('devices', { devices: data });
            await dataStore.save();
        }
        addingNew = false;
        newInput = '';
        // svelte reinstate fix
        data = data;
        await sleep(200);
        loading = false;
    }

    async function deleteDevices(devices: Set<Device>) {
        loading = true;
        for (let entry of devices.values()) {
            data.splice(data.indexOf(entry), 1);
        }
        await dataStore.set('devices', { devices: data });
        await dataStore.save();
        checkDevices.clear();
        // svelte reinstate fix
        data = data;
        await sleep(200);
        loading = false;
    }
</script>

{#if loading}
    <div class="flex h-full justify-center items-center">
        <Spinner text="" />
    </div>
{:else if Object.keys(data).length < 1 && !addingNew}
    <div class="flex flex-col space-y-4 h-full justify-center items-center">
        <p>No Devices Found :&lbrace;</p>
        <Button
                size="lg"
                class="text-lg bg-blue-900 text-white"
                on:click={() => {
                    addingNew = true;
                }}
                disabled={addingNew}>Add</Button
            >
    </div>
{:else}
    <Table.Root>
        <Table.Body>
            {#each data as device}
                <Table.Row>
                    <Table.Cell class="text-left text-xl" on:click={() => dispatchSelect(device.host)}
                        >{device.host}</Table.Cell
                    >
                    <Table.Cell class="text-center"
                        ><Checkbox
                            class="scale-125"
                            on:click={() => updateCheckedDevices(device)}
                            checked={checkDevices.has(device)}
                        /></Table.Cell
                    >
                </Table.Row>
            {/each}
            {#if addingNew}
                <Table.Row class="text-left">
                    <Table.Cell class="text-xl">
                        <input class="bg-[rgba(0,0,0,0)]" bind:value={newInput} />
                    </Table.Cell>
                    <Table.Cell class="text-right"
                        ><Button
                            size="sm"
                            class="w-3 h-3 p-3 m-0 text-xl"
                            on:click={() => addDevice({ host: newInput, mdns: false })}
                            disabled={!validIP(newInput)}>+</Button
                        ></Table.Cell
                    >
                </Table.Row>
            {/if}
        </Table.Body>
    </Table.Root>
    <div class="flex flex-row justify-center items-center space-x-2 my-4">
        {#if checkDevices.size > 0}
            <Button size="lg" class="bg-red-800 text-white text-lg" on:click={() => deleteDevices(checkDevices)}
                >Delete ({checkDevices.size})</Button
            >
        {:else}
            <Button
                size="lg"
                class="text-lg bg-blue-900 text-white"
                on:click={() => {
                    addingNew = true;
                }}
                disabled={addingNew}>Add</Button
            >
        {/if}
    </div>
{/if}
