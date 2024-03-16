<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";
	import { goto } from "$app/navigation";

    
    type FATData = {
        file_addresses_list: FileAddresses[],
    }

    type FileAddresses = {
        start_address: number,
        end_address: number,
    }
    
    $: fat_data = {} as FATData;
    async function get_fnt_data(){
        let fat = invoke('load_fat');
        fat.then((data) => { 
            fat_data = Object(data) as FATData;
            console.log(fat_data)
        })
        ;
    }
    get_fnt_data();
</script>   

<Heading tag="h3">FAT Table</Heading>
{#if fat_data && fat_data.file_addresses_list}
<div>Number of entries:{fat_data.file_addresses_list.length}</div>
<Table>
    <TableHead>
        <TableHeadCell>index</TableHeadCell>
        <TableHeadCell>start_address</TableHeadCell>
        <TableHeadCell>end_address</TableHeadCell>
    </TableHead>
    <TableBody>
       
            {#each fat_data.file_addresses_list as file,_}
                <TableBodyRow on:click={async () => goto(`/files/${file.start_address}/${file.end_address}`)}>
                    <TableBodyCell>{_}</TableBodyCell>
                    <TableBodyCell class="font-mono">{file.start_address.toString(16)}</TableBodyCell>
                    <TableBodyCell class="font-mono">{file.end_address.toString(16)}</TableBodyCell>
                </TableBodyRow>
            {/each}
        
    </TableBody>
</Table>
{/if}