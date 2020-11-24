<template>
  <nav
    class="flex shadow-lg items-center justify-between flex-wrap bg-gray-800 p-3"
  >
    <div class="w-auto flex items-center flex-grow flex-shrink text-gray-200">
      <Card
        :card-id="51"
        :width="35"
        :is-interactive="false"
        class="mx-2 transform -rotate-6"
      />
      <a class="font-semibold text-2xl tracking-tight mx-2" href="#">
        Heads-up Odds Calculator
      </a>
    </div>
    <div class="w-auto block">
      <div class="float-right py-0">
        <a href="#" class="inline-block text-gray-100 hover:text-blue-300 mr-8">
          About
        </a>
        <a
          href="https://github.com/b-inary/odds-calculator"
          class="inline-block text-gray-100 hover:text-blue-300 mr-4"
        >
          GitHub
        </a>
      </div>
    </div>
  </nav>
</template>

<script>
import { defineComponent } from "vue";
import Card from "./Card.vue";
export default defineComponent({
  name: "Header",
  components: { Card },
});
</script>
