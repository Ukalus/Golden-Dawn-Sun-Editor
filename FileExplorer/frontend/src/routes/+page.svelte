<script>
    import { invoke } from "@tauri-apps/api";
    $: entries= [];
    async function get_dict(){
        let fat = invoke('load_directory');
        fat.then((data) => { 

           
            entries = data ;
            console.log(entries)
        })
        ;
    }
    get_dict();
</script>


{#each entries as entry}
<p>{entry.name.map((e) => e = String.fromCharCode(e) )}</p>
{/each}




