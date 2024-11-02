<!--
    iro.js - Mozilla Public License Version 2.0
    https://github.com/jaames/iro.js
-->

<script lang="ts">
    import iro from '@jaames/iro';
    import { onMount, createEventDispatcher } from 'svelte';

    let { width = 250, initial = $bindable('#ffffff') }: { width: number; initial: string } = $props();

    const dispatch = createEventDispatcher();

    async function dispatchColor(hex: string, rgb: { r: number; g: number; b: number }) {
        dispatch('color', {
            hex: hex,
            rgb: rgb
        });
    }

    onMount(() => {
        let colorPicker: any = new (iro as any).ColorPicker('#picker', {
            width: width,
            color: initial,
            borderWidth: 0,
            layout: [
                {
                    component: iro.ui.Wheel
                }
            ]
        });

        colorPicker.on('color:change', function (color: any) {
            dispatchColor(color.hexString, color.rgb);
        });
    });
</script>

<div>
    <div id="picker"></div>
</div>
