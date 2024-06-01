<script lang="ts">
    import { Toggle, P } from "flowbite-svelte";
    import { onMount } from "svelte";
    import { page } from "$app/stores";
    
    let switchMode = true;
    export let data: number[];
    export let chunkSize = 400;
    let index = 0;
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
</script>

<div>
    <div class="grid grid-cols-4">
        <p>pathname: {$page.url.pathname}</p>
        <p>startadress: {$page.params.start_address}</p>
        <p>endadress: {$page.params.end_address}</p>
        <p>total size: {parseInt($page.params.end_address) - parseInt($page.params.start_address) }</p>
    </div>
    
    
    
        <Toggle bind:checked={switchMode}>String (WIP, dont use on large files)</Toggle>
        <div class="grid grid-cols-32">
            {#if switchMode}
                {#each data as byteNum}
                    <P class="m-full text-gray-400">{byteNum.toString(16)}</P>
                {/each}
            {:else}
        
                {#each data as byteNum}
                    <P class="m-full">{String.fromCharCode(byteNum)}</P>
                {/each}
            {/if}
            <div id="showMore" class="showMore"></div>
        </div>
</div>