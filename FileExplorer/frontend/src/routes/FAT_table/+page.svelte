<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, TableHead, TableHeadCell } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";
	import { goto } from "$app/navigation";
	import { onMount } from "svelte";

    
    type FATData = {
        file_addresses_list: FileAddresses[],
    }

    type FileAddresses = {
        start_address: number,
        end_address: number,
    }
    
    $: fat_data = {} as FATData;
    async function get_fat_data(){
        let fat = invoke('load_fat');
        fat.then((data) => { 
            fat_data = Object(data) as FATData;
            console.log(fat_data)
        })
        ;
    }
    get_fat_data();
    $: index = 0;
    let chunkSize = 30;
    $: showArray = [];
   
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

<Heading tag="h3">FAT Table</Heading>
{#if fat_data && fat_data.file_addresses_list}
<div>Number of entries:{fat_data.file_addresses_list.length}</div>
{/if}
<Table>
    <TableHead>
        <TableHeadCell>index</TableHeadCell>
        <TableHeadCell>start_address</TableHeadCell>
        <TableHeadCell>end_address</TableHeadCell>
    </TableHead>
   
    <TableBody >
        {#if index && fat_data && fat_data.file_addresses_list}
            {#each fat_data.file_addresses_list.slice(0,index+chunkSize) as fat,i}
                <TableBodyRow on:click={async () => goto(`/files/${fat.start_address}/${fat.end_address}`)}>
                    <TableBodyCell>{i}</TableBodyCell>
                    <TableBodyCell class="font-mono">{fat.start_address.toString(16)}</TableBodyCell>
                    <TableBodyCell class="font-mono">{fat.end_address.toString(16)}</TableBodyCell>
                </TableBodyRow>
            {/each}
        {/if}
        <TableBodyRow id="showMore" class="showMore">
            <TableBodyCell>loading</TableBodyCell>
            <TableBodyCell class="font-mono">loading</TableBodyCell>
            <TableBodyCell class="font-mono">loading</TableBodyCell>
        </TableBodyRow>
    </TableBody>
</Table>
