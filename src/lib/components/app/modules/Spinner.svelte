<!--
    The MIT License (MIT)

    Copyright (c) Utkarsh Verma

    Permission is hereby granted, free of charge, to any person obtaining a copy of
    this software and associated documentation files (the "Software"), to deal in
    the Software without restriction, including without limitation the rights to
    use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
    the Software, and to permit persons to whom the Software is furnished to do so,
    subject to the following conditions:

    The above copyright notice and this permission notice shall be included in all
    copies or substantial portions of the Software.

    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
    FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
    COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
    IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
    CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
-->

<script lang="ts">
    let { text = '' }: { text: string } = $props();
</script>

<div class="flex flex-col justify-center text-center">
    <svg fill="#fff" width="128" height="128" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"
        ><style>
            @reference "tailwindcss/theme";

            .spinner_l9ve {
                animation: spinner_rcyq 1.2s cubic-bezier(0.52, 0.6, 0.25, 0.99) infinite;
            }
            .spinner_cMYp {
                animation-delay: 0.4s;
            }
            .spinner_gHR3 {
                animation-delay: 0.8s;
            }
            @keyframes spinner_rcyq {
                0% {
                    transform: translate(12px, 12px) scale(0);
                    opacity: 1;
                }
                100% {
                    transform: translate(0, 0) scale(1);
                    opacity: 0;
                }
            }
        </style><path
            class="spinner_l9ve"
            d="M12,1A11,11,0,1,0,23,12,11,11,0,0,0,12,1Zm0,20a9,9,0,1,1,9-9A9,9,0,0,1,12,21Z"
            transform="translate(12, 12) scale(0)"
        /><path
            class="spinner_l9ve spinner_cMYp"
            d="M12,1A11,11,0,1,0,23,12,11,11,0,0,0,12,1Zm0,20a9,9,0,1,1,9-9A9,9,0,0,1,12,21Z"
            transform="translate(12, 12) scale(0)"
        /><path
            class="spinner_l9ve spinner_gHR3"
            d="M12,1A11,11,0,1,0,23,12,11,11,0,0,0,12,1Zm0,20a9,9,0,1,1,9-9A9,9,0,0,1,12,21Z"
            transform="translate(12, 12) scale(0)"
        /></svg
    >
    {#if text != ''}
        <p class="animate-pulse text-onBackground">{text}</p>
    {/if}
</div>
