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
	<div class="grow">
		<slot/>
	</div>
</div>



