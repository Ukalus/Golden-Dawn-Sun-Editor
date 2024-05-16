<script lang="ts">
    import { page } from '$app/stores';
    import { invoke } from '@tauri-apps/api';
    import { Button, P, Toggle } from 'flowbite-svelte';
	import { onMount } from 'svelte';
    
    let hexEditorOpen: boolean = false;
    $: fileArray = [];
    async function get_fnt_data(){
        let file = invoke('load_file',{start: parseInt($page.params.start_address), end: parseInt($page.params.end_address)});
        file.then((data) => { 
            fileArray = Object(data);
        })
        ;
    }
    get_fnt_data();
    // Intersection Observer API
    $: index = 0;
    $: chunkSize = 800;
    $: viewArr = fileArray.slice(0,index+chunkSize);
    onMount(() => {
        
        let options = {
        root: null,
        rootMargin: "0px",
        threshold: 1.0,
       
        };
        let target = document.querySelector("#showMore");
        console.log(target);
        if(target){
            let callback = (entries, observer) => {
            entries.forEach((entry) => {
                if(entry.isIntersecting) {
                    console.log("INTERSECTION")
                    index = index + chunkSize;
                }
            });
            };
            let observer = new IntersectionObserver(callback, options);
            observer.observe(target);
           
        }
    })
   
    let switchMode = true;
    function saveToPC(){
        console.log("saving")
        
    }

</script>

<div>
    <div class="grid grid-cols-4">
        <p>pathname: {$page.url.pathname}</p>
        <p>startadress: {$page.params.start_address}</p>
        <p>endadress: {$page.params.end_address}</p>
        <p>total size: {parseInt($page.params.end_address) - parseInt($page.params.start_address) }</p>
    </div>
    
    <Button on:click={saveToPC}>Save file to...</Button>
    <Button on:click={() => hexEditorOpen =!hexEditorOpen}>Open Hexeditor</Button>
    {#if hexEditorOpen}
        <Toggle bind:checked={switchMode}>String (WIP, dont use on large files)</Toggle>
        <div class="grid grid-cols-32">
            {#if switchMode}
                {#each viewArr as byteNum}
                    <P class="m-full text-gray-400">{byteNum.toString(16)}</P>
                {/each}
            {:else}
        
                {#each viewArr as byteNum}
                    <P class="m-full">{String.fromCharCode(byteNum)}</P>
                {/each}
            {/if}
            <div id="showMore"></div>
        </div>
    {/if}
</div>