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
    <TableHead>
        <TableHeadCell>length</TableHeadCell>
        <TableHeadCell>name</TableHeadCell>
        <TableHeadCell>sub id</TableHeadCell>
    </TableHead>
    <TableBody>
        {#each entries as entry}
        <TableBodyRow>
            <TableBodyCell>{entry.name_length}</TableBodyCell>
            <TableBodyCell>{entry.name.map((number) => String.fromCharCode(number)).join("")}</TableBodyCell>
            <TableBodyCell>{entry.sub_directory_id}</TableBodyCell>

            
        </TableBodyRow>
        {/each}
    </TableBody>
</Table>




