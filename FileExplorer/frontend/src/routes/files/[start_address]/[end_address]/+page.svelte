<script lang="ts">
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api';
    import { P, Toggle } from 'flowbite-svelte';
    
    $: fileArray = [];
    async function get_fnt_data(){
        let file = invoke('load_file',{start: parseInt($page.params.start_address), end: parseInt($page.params.start_address)+1200});
        file.then((data) => { 
            fileArray = Object(data);
        })
        ;
    }
    get_fnt_data();
    let switchMode = true;
</script>

<div>
    {$page.url.pathname}
    {JSON.stringify($page.params.start_address)}
    {JSON.stringify($page.params.end_address)}
    <Toggle bind:checked={switchMode}>String</Toggle>
    <div class="grid grid-cols-32">
        {#if switchMode}
            {#each fileArray as byteNum}
                <P class="m-full text-gray-400">{byteNum.toString(16)}</P>
            {/each}
        {:else}
    
            {#each fileArray as byteNum}
                <P class="m-full">{String.fromCharCode(byteNum)}</P>
            {/each}
        {/if}
    </div>
</div>