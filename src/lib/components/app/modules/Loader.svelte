<script lang="ts">
    import { fade } from 'svelte/transition';
    import Spinner from '$lib/components/app/modules/Spinner.svelte';

    let { text = '' }: { text: string } = $props();
</script>

<div transition:fade={{ delay: 0, duration: 600 }}>
    <div class="absolute z-30 left-[calc(50%-64px)] top-[calc(50%-64px)]">
        <Spinner {text} />
    </div>
</div>
