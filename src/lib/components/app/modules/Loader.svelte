<script lang="ts">
    import { fade } from 'svelte/transition';
    import Spinner from './Spinner.svelte';

    export let text: string = '';
</script>

<div transition:fade={{ delay: 0, duration: 600 }}>
    <div class="absolute z-30 left-[calc(50%-64px)] top-[calc(50%-64px)]">
        <Spinner {text} />
    </div>
</div>
