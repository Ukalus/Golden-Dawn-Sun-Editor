<script lang="ts">
    import { P, Button, Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { getHexValue } from "src/lib/utils";
    import { invoke } from "@tauri-apps/api";

    
    type FATData = {
        file_adresses: FileAddresses[],
    }

    type FileAddresses = {
        start_adress: any[],
        end_adress: any[],
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
        <TableHeadCell>start_address</TableHeadCell>
        <TableHeadCell>end_address</TableHeadCell>
    </TableHead>
    <TableBody>
        {#if fat_data.file_adresses}
            {#each fat_data.file_adresses as file}
                <TableBodyRow>
                    <TableBodyCell>{getHexValue(file.start_adress.reverse())}</TableBodyCell>
                    <TableBodyCell>{getHexValue(file.end_adress.reverse())}</TableBodyCell>
                </TableBodyRow>
            {/each}
        {/if}
    </TableBody>
</Table>