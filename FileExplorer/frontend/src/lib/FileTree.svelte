<script>
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, SidebarDropdownItem, SidebarDropdownWrapper } from 'flowbite-svelte';
    let spanClass = 'flex-1 ms-3 whitespace-nowrap';

    let folders = Array(10)
  </script>
  
  <Sidebar>
    <SidebarWrapper>
        <SidebarGroup>
            <SidebarItem label="File.txt"/>
            {#each folders as folder}
            <SidebarDropdownWrapper label="folder">
                <SidebarDropdownItem label="folder">
                </SidebarDropdownItem>
            </SidebarDropdownWrapper>
                
            {/each}
        </SidebarGroup>
        
    </SidebarWrapper>
  </Sidebar>