<script lang="ts">
    import { P, Button, Table, TableBody, TableBodyCell, TableBodyRow, Heading, Card, TableHead, TableHeadCell } from "flowbite-svelte";
    import { getHexValue } from "src/lib/utils";
    import { invoke } from "@tauri-apps/api";

    type MetaData = {
        game_title: any[],
        game_code: any[],
        maker_code: any[],
        unit_code: number,
        encryption_seed_select: number,
        device_capability: number,
    }
    let metaData = {} as MetaData;
    async function get_meta_data(){
        let romMetadata = invoke('load_meta',{path: "/home/ukalus/projects/golden_sun_reverse/ROM/Golden (En)SunDarkDawn.nds"});
        romMetadata.then((meta) => {
            metaData = Object(meta);
            console.log(metaData)
        })
    }
    get_meta_data();
</script>
<Heading tag="h3">ROM Header</Heading>
        <Table>
            <TableHead>
                <TableHeadCell>Key</TableHeadCell>
                <TableHeadCell>String</TableHeadCell>
                <TableHeadCell>Raw bytes</TableHeadCell>
            </TableHead>
            <TableBody>
                {#each Object.entries(metaData) as [key,value]} 
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
                            {:else}
                                {value.toString(16).toUpperCase()}
                            {/if}
                            </TableBodyCell>
                        </TableBodyRow>

                    {/each}
            </TableBody>
        </Table>