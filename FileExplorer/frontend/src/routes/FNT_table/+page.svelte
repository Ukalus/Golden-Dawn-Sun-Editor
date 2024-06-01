<script lang="ts">
    import { invoke } from "@tauri-apps/api";
	import {Button, NumberInput, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";

    let offset = 0;
    let main_table_entry;
    async function get_main_entry(offset: number){
        let load_main_table_entry = invoke('load_directory',{offset: offset});
        load_main_table_entry.then((data) => { 

           
            main_table_entry = data ;
            console.log(main_table_entry)
        })
        ;
    }
    $: get_main_entry(offset);

    async function save_file(file: {}){
        let load_main_table_entry = invoke('write_file_to_system',{ndsFile: file});
        load_main_table_entry.then(() => { 
            console.log("saved file");
        })
        ;
    }
    
</script>


<NumberInput bind:value={offset}></NumberInput>
<Table>
    <TableBody>
        {#if main_table_entry && main_table_entry.sub_table_entries}
        <TableBodyRow class="dark:bg-green-900">
            <TableBodyCell>{main_table_entry.id_first_file}</TableBodyCell>
            <TableBodyCell>{main_table_entry.id_parent_directory}</TableBodyCell>
            <TableBodyCell>{main_table_entry.offset_to_subtable}</TableBodyCell>
            <TableBodyCell>Action</TableBodyCell>

        </TableBodyRow>
            {#each main_table_entry.sub_table_entries as entry}
                {#if entry.NDSFile}
                    <TableBodyRow on:click={async () => goto(`/files/${fat.start_address}/${fat.end_address}`)} class="dark:bg-violet-900">
                        <TableBodyCell>{entry.NDSFile.length}</TableBodyCell>
                        <TableBodyCell>{entry.NDSFile.name}</TableBodyCell>
                        <TableBodyCell>{entry.NDSFile.id} (0x{entry.NDSFile.id.toString(16)})</TableBodyCell>
                        <TableBodyCell><Button on:click={() => save_file(entry.NDSFile)}>Save file</Button></TableBodyCell>
                    </TableBodyRow>
                {:else}
                    <TableBodyRow class="dark:bg-rose-900">
                        <TableBodyCell>{entry.NDSDirectory.length}</TableBodyCell>
                        <TableBodyCell>{entry.NDSDirectory.name}</TableBodyCell>
                        <TableBodyCell>{entry.NDSDirectory.sub_directory_id}</TableBodyCell>
                        <TableBodyCell><Button>Open Folder</Button></TableBodyCell>
                    </TableBodyRow>
                {/if}
            {/each}
           
        {/if}
        
    
    
    </TableBody>
</Table>