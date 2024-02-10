<script>
	import { Card } from 'flowbite-svelte';
	import '../app.pcss';
	import Nav from 'src/lib/Nav.svelte';
</script>


<Nav/>
<Card class="max-w-full h-vh">
	<slot />
</Card>

