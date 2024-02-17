<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";

    
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

<Table>
    <TableHead>
        <TableHeadCell>index</TableHeadCell>
        <TableHeadCell>start_address</TableHeadCell>
        <TableHeadCell>end_address</TableHeadCell>
    </TableHead>
    <TableBody>
        {#if fat_data && fat_data.file_addresses_list}
            {#each fat_data.file_addresses_list as file,_}
                <TableBodyRow>
                    <TableBodyCell>{_}</TableBodyCell>
                    <TableBodyCell class="font-mono">{file.start_address.toString(16)}</TableBodyCell>
                    <TableBodyCell class="font-mono">{file.end_address.toString(16)}</TableBodyCell>
                </TableBodyRow>
            {/each}
        {/if}
    </TableBody>
</Table>
