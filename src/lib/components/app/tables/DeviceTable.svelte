<script lang="ts">
    // shadcdn
    import * as Table from '$lib/components/ui/table';
    import { Checkbox } from '$lib/components/ui/checkbox';
    import Button from '$lib/components/ui/button/button.svelte';
    import { toast } from 'svelte-sonner';
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
            {#each data.devices as device}
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
                        <!-- svelte-ignore a11y-autofocus -->
                        <input
                            id="newInput"
                            class="bg-[rgba(0,0,0,0)] border-l-4 border-r-4"
                            bind:value={newInput}
                            autofocus
                        />
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
