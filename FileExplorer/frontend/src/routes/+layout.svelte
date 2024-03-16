<script lang="ts">
	import '../app.pcss';
	import Nav from 'src/lib/Nav.svelte';
	import { page } from '$app/stores';
	import { sineIn } from 'svelte/easing';
	import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper, SidebarDropdownWrapper, SidebarDropdownItem } from 'flowbite-svelte';
	let spanClass = 'flex-1 ms-3 whitespace-nowrap';
	$: activeUrl = $page.url.pathname;
	
	
</script>


<Nav/>


<div class="flex flex-row w-full min-h-[100vh] bg-white dark:bg-slate-800">
	<Sidebar>
		<SidebarWrapper>
		  <SidebarGroup>
			<SidebarItem label="File" />
			<SidebarDropdownWrapper label="Folder" isOpen={true}>
			  <SidebarDropdownItem label="File" href="/products" />
			  <SidebarDropdownItem label="File2" href="/cart" />
			</SidebarDropdownWrapper>
			<SidebarDropdownWrapper label="Folder2">
			  <SidebarDropdownItem label="File" href="/components/item1" />
			  <SidebarDropdownItem label="File 2" href="/components/item2" />
			</SidebarDropdownWrapper>
		  </SidebarGroup>
		</SidebarWrapper>
	  </Sidebar>
	<div class="grow">
		<slot/>
	</div>
</div>



