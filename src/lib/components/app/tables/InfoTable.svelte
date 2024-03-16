<script lang="ts">
    import * as Table from '$lib/components/ui/table';
    import Spinner from '../modules/Spinner.svelte';
    import type { InfoResponse, WifiData } from '$lib/types/responses';

    export let data: InfoResponse;

    let exceptions = ['leds', 'fs', 'maps', 'lm', 'lip'];

    function isWifiData(obj: any): obj is WifiData {
        return (
            typeof obj.bssid === 'string' &&
            typeof obj.rssi === 'number' &&
            typeof obj.signal === 'number' &&
            typeof obj.channel === 'number'
        );
    }
</script>

{#if Object.keys(data).length < 1}
    <div class="flex h-full justify-center items-center">
        <Spinner text="" />
    </div>
{:else}
    <Table.Root>
        <Table.Header>
            <Table.Row class="text-left">
                <Table.Head class="font-bold text-neutral-100">Type</Table.Head>
                <Table.Head class="font-bold text-neutral-100">Value</Table.Head>
            </Table.Row>
        </Table.Header>
        <Table.Body>
            {#each Object.entries(data) as [type, value]}
                {#if isWifiData(value)}
                    <Table.Row class="text-left">
                        <Table.Cell>{type}</Table.Cell>
                        <Table.Cell>{value.bssid}/{value.channel}/{value.rssi}/{value.signal}</Table.Cell>
                    </Table.Row>
                {:else if !exceptions.includes(type)}
                    <Table.Row class="text-left">
                        <Table.Cell>{type}</Table.Cell>
                        <Table.Cell>{value}</Table.Cell>
                    </Table.Row>
                {/if}
            {/each}
        </Table.Body>
    </Table.Root>
{/if}
