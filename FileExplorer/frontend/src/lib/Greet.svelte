<script>
    import { invoke } from '@tauri-apps/api/tauri'
  
    let name = 'Ukalus'
    let greetMsg = ''
  
    async function greet() {
      greetMsg = await invoke('greet', { name })
    }
    greet()
  </script>
  
 {greetMsg}