<script lang="ts">
    // components
    import Spinner from '$lib/components/app/modules/Spinner.svelte';
    // types
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
    <div class="bg-blue-300 bg-opacity-25 p-4 rounded-xl">
        <table>
            <tbody>
                {#each Object.entries(data) as [type, value]}
                    {#if isWifiData(value)}
                        <tr class="text-left">
                            <td>{type}</td>
                            <td>{value.bssid}/{value.channel}/{value.rssi}/{value.signal}</td>
                        </tr>
                    {:else if !exceptions.includes(type)}
                        <tr class="text-left">
                            <td>{type}</td>
                            <td>{value}</td>
                        </tr>
                    {/if}
                {/each}
            </tbody>
        </table>
    </div>
{/if}
