<template>
  <div
    class="flex justify-center items-center h-8 w-8 font-extralight text-gray-100"
    :class="{
      'bg-red-500': selected,
      'bg-white': !selected,
    }"
  >
    {{ name }}
  </div>
</template>

<script>
import { defineComponent } from "vue";
import VueTypes from "vue-types";

export default defineComponent({
  name: "Hand",
  props: {
    handId: VueTypes.custom(
      (arg) =>
        typeof arg === "number" &&
        Number.isInteger(arg) &&
        arg >= 0 &&
        arg < 169
    ).isRequired,
    selected: VueTypes.bool.def(false),
  },
  computed: {
    HAND_NAME_TABLE: function () {
      const RANK_NAMES = `AKQJT98765432`.split("");
      const handNames = Array(13);
      for (const i of Array(13).keys()) {
        handNames[i] = Array(13);
        for (const j of Array(13).keys()) {
          let handName =
            i < j
              ? RANK_NAMES[i] + RANK_NAMES[j]
              : RANK_NAMES[j] + RANK_NAMES[i];
          if (i < j) {
            handName += "s";
          } else if (j < i) {
            handName += "o";
          }
          handNames[i][j] = handName;
        }
      }
      return handNames;
    },
    name: function () {
      const i = (this.handId / 13) | 0;
      const j = this.handId % 13;
      return this.HAND_NAME_TABLE[i][j];
    },
  },
});
</script>

<style scoped>
</style>
