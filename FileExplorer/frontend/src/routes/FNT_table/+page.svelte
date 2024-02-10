<script lang="ts">
    import { P, Button, Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { getHexValue } from "src/lib/utils";
    import { invoke } from "@tauri-apps/api";

    
    type FNTData = {
        id_first_subtable: any[],
        offset_to_subtable: any[],
        sub_table: {
            type_or_length: number,
            file_name: any[],
        }

    }
    
    $: fnt_data = {} as FNTData;
    async function get_fnt_data(){
        let fnt = invoke('load_fnt');
        fnt.then((data) => {
            fnt_data = Object(data) as FNTData;
            console.log(fnt_data)
        })
        ;
    }
    get_fnt_data();
</script>   

<Heading tag="h3">FNT Table</Heading>

<Table>
    <TableHead>
        <TableHeadCell>Key</TableHeadCell>
        <TableHeadCell>String</TableHeadCell>
        <TableHeadCell>Raw bytes</TableHeadCell>
    </TableHead>
    <TableBody>
        {#each Object.entries(fnt_data) as [key,value]} 
                <TableBodyRow>
                    <TableBodyCell>{key}</TableBodyCell>
                    <TableBodyCell>
                        {#if value.constructor === Array}
                            {#each value as num}
                                {String.fromCharCode(num)}
                            {/each}
                        {:else}
                            {value}
                        {/if}
                    
                    </TableBodyCell>
                    <TableBodyCell>
                        {#if value.constructor === Array}
                        <P class="font-mono">{getHexValue(value.reverse())}</P>
                        {/if}
                    </TableBodyCell>
                </TableBodyRow>

            {/each}
    </TableBody>
</Table>

<Heading tag="h3">Sub-Table</Heading>
<TableBody>
    <TableHead>
        <TableHeadCell>Length/Type of Subtable()</TableHeadCell>
        <TableHeadCell>File Name</TableHeadCell>
    </TableHead>
    <TableBody>
        <TableBodyRow>
            
            <TableBodyCell>
                {#each Object.values(fnt_data.sub_table?.file_name ?? []) as chara}
                    {String.fromCharCode(chara)}
                {/each}
            </TableBodyCell>
        </TableBodyRow>
    </TableBody>
</TableBody>
<p>{getHexValue(fnt_data?.sub_table?.file_name ?? []) ?? ""}</p>
