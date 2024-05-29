<script>
    import { invoke } from "@tauri-apps/api";
	import {Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";

    let main_table_entry;
    async function get_main_entry(){
        let load_main_table_entry = invoke('load_directory');
        load_main_table_entry.then((data) => { 

           
            main_table_entry = data ;
            console.log(main_table_entry)
        })
        ;
    }
    get_main_entry();
</script>



<Table>
    <TableBody>
        {#if main_table_entry && main_table_entry.sub_table_entries}
        <TableBodyRow class="dark:bg-green-900">
            <TableBodyCell>{main_table_entry.id_first_file}</TableBodyCell>
            <TableBodyCell>{main_table_entry.id_parent_directory}</TableBodyCell>
            <TableBodyCell>{main_table_entry.offset_to_subtable}</TableBodyCell>
        </TableBodyRow>
            
            {#each main_table_entry.sub_table_entries as entry}
                {#if entry.File}
                    <TableBodyRow class="dark:bg-violet-900">
                        <TableBodyCell>{entry.File.length}</TableBodyCell>
                        <TableBodyCell>{entry.File.name}</TableBodyCell>
                        <TableBodyCell>{entry.File.id} (0x{entry.File.id.toString(16)})</TableBodyCell>
                    </TableBodyRow>
                {:else}
                    <TableBodyRow class="dark:bg-rose-900">
                        <TableBodyCell>{entry.Directory.length}</TableBodyCell>
                        <TableBodyCell>{entry.Directory.name}</TableBodyCell>
                        <TableBodyCell>{entry.Directory.sub_directory_id}</TableBodyCell>
                    </TableBodyRow>
                {/if}
            {/each}
           
        {/if}
        
    
    
    </TableBody>
</Table>




