<script lang="ts">
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api';
    import { Button } from 'flowbite-svelte';
	import HexView from 'src/lib/HexView.svelte';
    
    let hexEditorOpen: boolean = false;
    $: fileArray = [];
    async function get_fnt_data(){
        let file = invoke('load_file_content',{start: parseInt($page.params.start_address), end: parseInt($page.params.end_address)});
        file.then((data) => { 
            fileArray = Object(data);
        })
        ;
    }
    get_fnt_data();
    // Intersection Observer API
    $: index = 0;
    $: chunkSize = 1600;
    $: viewArr = fileArray.slice(0,index+chunkSize);
    
   
    
    function saveToPC(){
        console.log("saving")
    }

</script>
<Button on:click={() => hexEditorOpen =!hexEditorOpen}>Open Hexeditor</Button>
{#if hexEditorOpen}
    <HexView data={viewArr}/>
{/if}