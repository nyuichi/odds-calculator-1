<template>
  <div
    class="md:container block lg:flex flex-wrap ring ring-blue-500 bg-blue-100 rounded-xl mx-auto py-1"
  >
    <div class="lg:flex w-full">
      <!-- Spades -->
      <div class="flex flex-wrap justify-center p-1">
        <Card
          v-for="i in Array.from({ length: 13 }, (_, i) => 4 * i + 3)"
          :key="`card-input-${i}`"
          class="m-1"
          :card-id="i"
          :disabled="isUsed[i]"
          :width="50"
          @click="$emit('add-card', i)"
        />
        <svg v-for="i in 5" :key="'dummy-' + i" class="mx-1 h-0" width="50px" />
      </div>

      <!-- Hearts -->
      <div class="flex flex-wrap justify-center p-1">
        <Card
          v-for="i in Array.from({ length: 13 }, (_, i) => 4 * i + 2)"
          :key="`card-input-${i}`"
          class="m-1"
          :card-id="i"
          :disabled="isUsed[i]"
          :width="50"
          @click="$emit('add-card', i)"
        />
        <svg
          v-for="i in 5"
          :key="'dummy-' + (i + 5)"
          class="mx-1 h-0"
          width="50px"
        />
      </div>

      <!-- Diamonds -->
      <div class="flex flex-wrap justify-center p-1">
        <Card
          v-for="i in Array.from({ length: 13 }, (_, i) => 4 * i + 1)"
          :key="`card-input-${i}`"
          class="m-1"
          :card-id="i"
          :disabled="isUsed[i]"
          :width="50"
          @click="$emit('add-card', i)"
        />
        <svg
          v-for="i in 5"
          :key="'dummy-' + (i + 10)"
          class="mx-1 h-0"
          width="50px"
        />
      </div>

      <!-- Clubs -->
      <div class="flex flex-wrap justify-center p-1">
        <Card
          v-for="i in Array.from({ length: 13 }, (_, i) => 4 * i)"
          :key="`card-input-${i}`"
          class="m-1"
          :card-id="i"
          :disabled="isUsed[i]"
          :width="50"
          @click="$emit('add-card', i)"
        />
        <svg
          v-for="i in 5"
          :key="'dummy-' + (i + 15)"
          class="mx-1 h-0"
          width="50px"
        />
      </div>
    </div>

    <div class="flex w-full justify-center my-1">
      <button
        class="bg-blue-500 hover:bg-blue-600 text-lg text-white shadow-md px-4 py-1 rounded-md focus:outline-none focus:ring focus:ring-blue-400 focus:ring-offset-2"
        @click="$emit('clear')"
      >
        Clear all
      </button>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import VueTypes from "vue-types";
import Card from "./Card.vue";
export default defineComponent({
  name: "InputBox",
  components: { Card },
  props: {
    isUsed: VueTypes.arrayOf(Boolean).isRequired,
  },
  emits: ["add-card", "clear"],
});
</script>
