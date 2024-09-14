<script>
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, SidebarDropdownItem, SidebarDropdownWrapper } from 'flowbite-svelte';
    let spanClass = 'flex-1 ms-3 whitespace-nowrap';
  </script>
    
    <div class="grid grid-cols-16">
        <Sidebar class="span-col-4">
            <SidebarWrapper>
              <SidebarGroup>
                {#each Array(20) as _,i}
                <SidebarDropdownWrapper label={"Folder" + i}>
                    {#each Array(3) as _,j}
                    <SidebarDropdownWrapper class="ps-4" label={"Folder" + j}>
                        {#each Array(4) as _,i}
                            <SidebarItem class="ps-8" label={"File_" + i + "_" + j + ".cbdml"}></SidebarItem>
                        {/each}
                    </SidebarDropdownWrapper>
                    {/each}
                 
                 
                </SidebarDropdownWrapper>
                {/each}
              </SidebarGroup>
            </SidebarWrapper>
          </Sidebar>
          <div class="bg-white span-col-12"></div>
    </div>


