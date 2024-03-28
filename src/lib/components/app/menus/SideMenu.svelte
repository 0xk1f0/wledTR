<script lang="ts">
    import * as Sheet from '$lib/components/ui/sheet';
    import { fade } from 'svelte/transition';

    export let name: string = '';
    export let side: 'right' | 'left' = 'left';
    export let title: string = '';
    export let open: boolean = false;
</script>

<Sheet.Root closeOnOutsideClick={true} bind:open>
    <Sheet.Trigger>
        <div class="flex flex-col justify-center text-center">
            <p class="text-xl font-mono font-bold uppercase">{name}</p>
        </div>
    </Sheet.Trigger>
    <Sheet.Content
        transition={fade}
        transitionConfig={{ delay: 0, duration: 150 }}
        bind:side
        class="flex flex-col justify-between"
    >
        <Sheet.Header>
            <Sheet.Title class="text-center">
                <div>{title}</div>
            </Sheet.Title>
        </Sheet.Header>
        <Sheet.Description class="flex flex-1 overflow-scroll">
            <div class="flex-1 text-center"><slot /></div>
        </Sheet.Description>
    </Sheet.Content>
</Sheet.Root>
