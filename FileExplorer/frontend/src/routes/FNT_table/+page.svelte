<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";

    
    type FNTData = {
        offset_to_subtable: number,
        id_first_file_subtable: number,
        id_parent_directory: number,
    }
    
    let fnt_data = {} as FNTData;
    async function get_fnt_data(){
        let fnt = invoke('load_fnt');
        fnt.then((data) => {
            console.log(fnt_data);
            fnt_data = Object(data) as FNTData;
        })
        ;
    }
    get_fnt_data();
</script>   

<Heading tag="h3">Directory Table</Heading>

<Table>
    <TableHead>
        <TableHeadCell>Offset Subtable</TableHeadCell>
        <TableHeadCell>ID of first file in Subtable</TableHeadCell>
        <TableHeadCell>Parent Directory</TableHeadCell>
    </TableHead>
    <TableBody>
        <TableBodyRow>
            {#if fnt_data}
                <TableBodyCell>{fnt_data.offset_to_subtable?.toString(16)}</TableBodyCell>
                <TableBodyCell>{fnt_data.id_first_file_subtable?.toString(16)}</TableBodyCell>
                <TableBodyCell>{fnt_data.id_parent_directory?.toString(16)}</TableBodyCell>
            {/if}
        </TableBodyRow>
    </TableBody>
</Table>

