<script lang="ts">
    import { P, Button, Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { getHexValue } from "src/lib/utils";
    import { invoke } from "@tauri-apps/api";

    
    type FNTData = {
        offset_to_subtable: any[],
        id_first_file_subtable: any[],
        id_parent_directory: any[],
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
            <TableBodyCell>{getHexValue(fnt_data?.offset_to_subtable ?? [])}</TableBodyCell>
            <TableBodyCell>{getHexValue(fnt_data?.id_first_file_subtable ?? [])}</TableBodyCell>
            <TableBodyCell>{getHexValue(fnt_data?.id_parent_directory ?? [])}</TableBodyCell>
            {/if}
        </TableBodyRow>
    </TableBody>
</Table>

<!-- <Heading tag="h3">First Sub-Table</Heading>
<TableBody>
    <TableHead>
        <TableHeadCell>Length/Type of Subtable()</TableHeadCell>
        <TableHeadCell>File Name</TableHeadCell>
        <TableHeadCell>Raw bytes Filename</TableHeadCell>
    </TableHead>
    <TableBody>
        <TableBodyRow>
            <TableBodyCell>{getHexValue([fnt_data.sub_table?.type_or_length ?? []])}</TableBodyCell>
            <TableBodyCell>
                {#each Object.values(fnt_data.sub_table?.file_name ?? []) as chara}
                    {String.fromCharCode(chara)}
                {/each}
            </TableBodyCell>
            <TableBodyCell>{getHexValue(fnt_data?.sub_table?.file_name ?? []) ?? ""}</TableBodyCell>
        </TableBodyRow>
    </TableBody>
</TableBody>
<p></p> -->
