<script>
    import { invoke } from "@tauri-apps/api";
	import {Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
    $: entries= [];
    async function get_dict(){
        let load_directory = invoke('load_directory');
        load_directory.then((data) => { 

           
            entries = data ;
            console.log(entries)
        })
        ;
    }
    let main_table_entry;
    async function get_main_entry(){
        let load_main_table_entry = invoke('load_main_table_entry');
        load_main_table_entry.then((data) => { 

           
            main_table_entry = data ;
            console.log(main_table_entry)
        })
        ;
    }
    get_main_entry();
    get_dict();
</script>



<Table>
    
    <TableBody>
        {#each entries as entry}
       
            {#if entry.Main}
            <TableBodyRow class="dark:bg-green-900">
                <TableBodyCell>{entry.Main.id_first_file}</TableBodyCell>
                <TableBodyCell>{entry.Main.id_parent_directory}</TableBodyCell>
                <TableBodyCell>{entry.Main.offset_to_subtable}</TableBodyCell>
            </TableBodyRow>
            {:else if entry.Sub && entry.Sub.sub_directory_id}
            <TableBodyRow class="dark:bg-violet-900">
                <TableBodyCell>{entry.Sub.name_length}</TableBodyCell>
                <TableBodyCell>{entry.Sub.name.map((number) => String.fromCharCode(number)).join("")}</TableBodyCell>
                <TableBodyCell>{entry.Sub.sub_directory_id}</TableBodyCell>
            </TableBodyRow>
            {:else if entry.Sub }
            <TableBodyRow class="dark:bg-rose-900">
                <TableBodyCell>{entry.Sub.name_length}</TableBodyCell>
                <TableBodyCell>{entry.Sub.name.map((number) => String.fromCharCode(number)).join("")}</TableBodyCell>
                <TableBodyCell>{entry.Sub.sub_directory_id}</TableBodyCell>
            </TableBodyRow>
            {/if}
            
        
        {/each}
    </TableBody>
</Table>




