<template>
  <Header />
  <Main />
</template>

<script>
import { defineComponent } from "vue";
import Header from "./components/Header.vue";
import Main from "./components/Main.vue";
export default defineComponent({
  name: "App",
  components: { Header, Main },
});
</script>
