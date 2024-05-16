<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";

    
    type FNTData = {
        entry_type: any,
        offset_to_subtable: number,
        id_first_file: number,
        id_parent_directory: number,
        next_entry: subEntry,
    }

    type subEntry = {
        length: number,
        name: Array<number>,
        sub_directory_id: number,
    }
    
    let fnt_data = [] as FNTData[];
    async function get_fnt_data(){
        let fnt = invoke('load_fnt');
        fnt.then((data) => {
            console.log(fnt_data);
            fnt_data = Object(data) as any;
        })
        ;
    }
    get_fnt_data();
    $: console.log(fnt_data)
</script>   

<Heading tag="h3">Directory Table</Heading>

<Table>
    <TableHead>
        <TableHeadCell>Offset Subtable</TableHeadCell>
        <TableHeadCell>ID of first file in Subtable</TableHeadCell>
        <TableHeadCell>Parent Directory</TableHeadCell>
        <TableBodyCell>Entry Type</TableBodyCell>
        <TableBodyCell>Subtable Length</TableBodyCell>
        <TableBodyCell>SubTable data</TableBodyCell>
        <TableBodyCell>Subtable id</TableBodyCell>
    </TableHead>
    <TableBody>
        {#if fnt_data}
            {#each fnt_data as entry}
            <TableBodyRow>
                <TableBodyCell>{entry.offset_to_subtable?.toString(16)}</TableBodyCell>
                <TableBodyCell>{entry.id_first_file}</TableBodyCell>
                <TableBodyCell>{entry.id_parent_directory?.toString(16)}</TableBodyCell>
                <TableBodyCell>{entry.entry_type}</TableBodyCell>
                {#if entry.next_entry}
                <TableBodyCell>  {entry.next_entry.length}</TableBodyCell>
                <TableBodyCell>  {entry.next_entry.name.map((number) => String.fromCharCode(number)).join("")}</TableBodyCell>
                <TableBodyCell>  {entry.next_entry.sub_directory_id}</TableBodyCell>

                   
                
                {/if}
            </TableBodyRow>
            {/each}
        {/if}
    </TableBody>
</Table>

