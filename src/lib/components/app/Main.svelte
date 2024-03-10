<script lang="ts">
    import * as AlertDialog from '$lib/components/ui/alert-dialog';
    import { Button } from '$lib/components/ui/button/index.js';
    import Loader from './Loader.svelte';

    let loading = false;

    async function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    async function doSomething() {
        loading = true;
        await sleep(3000);
        loading = false;
    }
</script>

{#if loading}
    <div class="flex flex-1 justify-center items-center">
        <Loader />
    </div>
{:else}
    <div class="flex flex-1 justify-center items-center border-2 m-4 rounded-lg">
        <AlertDialog.Root>
            <AlertDialog.Trigger>
                <Button variant="outline">Apply</Button>
            </AlertDialog.Trigger>
            <AlertDialog.Content>
                <AlertDialog.Header>
                    <AlertDialog.Title>Confirmation</AlertDialog.Title>
                    <AlertDialog.Description>Apply Settings?</AlertDialog.Description>
                </AlertDialog.Header>
                <AlertDialog.Footer>
                    <AlertDialog.Cancel>Cancel</AlertDialog.Cancel>
                    <AlertDialog.Action on:click={doSomething}>Continue</AlertDialog.Action>
                </AlertDialog.Footer>
            </AlertDialog.Content>
        </AlertDialog.Root>
    </div>
{/if}
