<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell, Input, NumberInput, Modal } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";

    enum NodeType {
        File ="File",
        Directory ="Directory",
        EndOfSubTable ="EndOfSubTable",
    }

    function decideFiletype(subEntry: subEntry){
       if(subEntry.name_length <= 127){
        return NodeType.File;
       }
       else if(subEntry.name_length != 0 && subEntry.name_length != 128){
        return subEntry.sub_directory_id;
       }
       else {
        return NodeType.EndOfSubTable;

       }
        subEntry.name_length > 127 ? subEntry.sub_directory_id: "File"
    }
    type FNTData = {
        entry_type: any,
        offset_to_subtable: number,
        id_first_file: number,
        id_parent_directory: number,
        next_entry: subEntry,
    }

    type subEntry = {
        name_length: number,
        name: String,
        sub_directory_id: number,
    }
    
    let fnt_data = [] as FNTData[];
    let fnt_sub_table = [] as subEntry[];

    async function get_fnt_data(){
        let fnt = invoke('load_main_table');
        fnt.then((data) => {
            console.log(fnt_data);
            fnt_data = Object(data) as any;
        })
        ;
    }

    async function get_sub_table(activeMainDict: FNTData){
        let fnt = invoke('load_sub_table_entry',{ mainDirectoryEntry: activeMainDict});
        fnt.then((data) => {
            console.log(data)
            fnt_sub_table = Array(data) as any;
        })
        ;
    }
    get_fnt_data();
    $: console.log(fnt_data)

    let filterDict: string;
    $: modalOpen = fnt_sub_table.length > 0;
</script>   

<Heading tag="h3">Directory Table</Heading>
<Input bind:value={filterDict}/>
<Table>
    <TableHead>
        <TableHeadCell>Offset Subtable</TableHeadCell>
        <TableHeadCell>ID of first file in Subtable</TableHeadCell>
        <TableHeadCell>Parent Directory</TableHeadCell>
        <TableBodyCell>Entry Type</TableBodyCell>
    </TableHead>
    <TableBody>
        {#if fnt_data}
            {#each fnt_data as entry}
                {#if filterDict ?  entry.id_parent_directory?.toString(16) == filterDict: true}
                <TableBodyRow on:click={() => get_sub_table(entry)}>
                    <TableBodyCell>{entry.offset_to_subtable?.toString(16)}</TableBodyCell>
                    <TableBodyCell>{entry.id_first_file}</TableBodyCell>
                    <TableBodyCell>{entry.id_parent_directory?.toString(16)}</TableBodyCell>
                    <TableBodyCell>{entry.entry_type}</TableBodyCell>
                </TableBodyRow>
                {/if}
            {/each}
        {/if}
    </TableBody>
</Table>

<Modal open={modalOpen} >
    <Table>
        <TableHead>
            <TableHeadCell>Length</TableHeadCell>
            <TableHeadCell>Name</TableHeadCell>
            <TableHeadCell>sub_directory_id</TableHeadCell>
        </TableHead>
        <TableBody>
            {#each fnt_sub_table as subEntry}
            <TableBodyCell>{subEntry.name_length}</TableBodyCell>
            <TableBodyCell>{subEntry.name}</TableBodyCell>
            <TableBodyCell>{decideFiletype(subEntry)}</TableBodyCell>

             {/each}

        </TableBody>l
    </Table>
</Modal>

