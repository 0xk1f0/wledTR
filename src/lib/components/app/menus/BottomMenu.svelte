<script lang="ts">
    import * as Drawer from '$lib/components/ui/drawer';
    import { fade } from 'svelte/transition';
    export let title: string = '';
    export let open: boolean = false;
</script>

<Drawer.Root bind:open>
    <Drawer.Trigger>
        <div class="flex flex-col justify-center text-center">
            <p class="text-xl font-mono font-bold uppercase">Edit</p>
        </div>
    </Drawer.Trigger>
    <Drawer.Content
        transition={fade}
        transitionConfig={{ delay: 0, duration: 150 }}
        class="flex flex-col justify-between"
    >
        <Drawer.Header>
            <Drawer.Title class="text-center">
                <div>{title}</div>
            </Drawer.Title>
            <Drawer.Description class="flex flex-1 overflow-scroll">
                <div class="flex-1 text-center"><slot /></div>
            </Drawer.Description>
        </Drawer.Header>
    </Drawer.Content>
</Drawer.Root>
