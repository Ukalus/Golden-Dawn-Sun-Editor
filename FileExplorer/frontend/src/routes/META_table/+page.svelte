<script lang="ts">
    import { Table, TableBody, TableBodyCell, TableBodyRow, Heading, TableHead, TableHeadCell } from "flowbite-svelte";
    import { invoke } from "@tauri-apps/api";

    type MetaData = {
        game_title: string,
        game_code: number,
        maker_code: number,
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
                            <TableBodyCell>{value}</TableBodyCell>
                        </TableBodyRow>

                    {/each}
            </TableBody>
        </Table>