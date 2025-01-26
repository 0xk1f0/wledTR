<!--
    iro.js - Mozilla Public License Version 2.0
    https://github.com/jaames/iro.js
-->

<script lang="ts">
    import iro from '@jaames/iro';
    import { onMount } from 'svelte';

    let { width = 250, initial = $bindable('#ffffff'), change, end } = $props();

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
            change({
                hex: color.hexString,
                rgb: color.rgb
            });
        });

        colorPicker.on('input:end', function (color: any) {
            end({
                hex: color.hexString,
                rgb: color.rgb
            });
        });
    });
</script>

<div>
    <div id="picker"></div>
</div>
