<template>
  <div class="px-4 pb-4">
    <div
      class="md:container flex items-center flex-wrap justify-evenly mx-auto"
    >
      <!-- You -->
      <div class="block text-center mt-4">
        <span class="block text-2xl font-semibold">You:</span>
        <Card
          v-for="i in [0, 1]"
          :key="`card-${i}`"
          class="mx-1 my-2"
          :class="{ 'add-card-animation': animationIndices.includes(i) }"
          :card-id="usedCards[i]"
          :removable="usedCards[i] !== -1"
          :selected="inputPosition === i"
          :width="75"
          @click="setInputPosition(i)"
          @remove-card="removeCard(i)"
        />
      </div>

      <!-- Opponent -->
      <div class="block text-center mt-4">
        <span class="block text-2xl font-semibold pb-1">Opponent:</span>
        <Card
          v-for="i in [2, 3]"
          :key="`card-${i}`"
          class="mx-1 my-2"
          :class="{ 'add-card-animation': animationIndices.includes(i) }"
          :card-id="usedCards[i]"
          :removable="usedCards[i] !== -1"
          :selected="inputPosition === i"
          :width="75"
          @click="setInputPosition(i)"
          @remove-card="removeCard(i)"
        />
      </div>

      <!-- Community -->
      <div class="block text-center mt-4">
        <span class="block text-2xl font-semibold pb-1">Community Cards:</span>
        <Card
          v-for="i in [4, 5, 6, 7, 8]"
          :key="`card-${i}`"
          class="mx-1 my-2"
          :class="{
            'add-card-animation': animationIndices.includes(i),
            'mx-6': i === 7,
          }"
          :card-id="usedCards[i]"
          :removable="usedCards[i] !== -1"
          :selected="inputPosition === i"
          :width="75"
          @click="setInputPosition(i)"
          @remove-card="removeCard(i)"
        />
      </div>
    </div>

    <!-- Win rate bar -->
    <div
      class="md:container flex items-center justify-center mx-auto mt-6 mb-10"
    >
      <div class="w-1/6 font-semibold">
        <span class="block text-center text-xl">You:</span>
        <span class="block text-center text-2xl">
          {{ (100.0 * yourWinRate).toFixed(2) + "%" }}
        </span>
      </div>
      <div class="flex w-1/2 h-4 rounded-2xl justify-between bg-gray-300">
        <div
          class="bg-green-600 rounded-l-2xl win-rate-animation"
          :class="{ 'rounded-r-2xl': yourWinRate === 1.0 }"
          :style="{ width: 100 * yourWinRate + '%' }"
        />
        <div
          class="bg-red-600 rounded-r-2xl win-rate-animation"
          :class="{ 'rounded-l-2xl': oppWinRate === 1.0 }"
          :style="{ width: 100 * oppWinRate + '%' }"
        />
      </div>
      <div class="w-1/6 font-semibold">
        <span class="block text-center text-xl">Opponent:</span>
        <span class="block text-center text-2xl">
          {{ (100.0 * oppWinRate).toFixed(2) + "%" }}
        </span>
      </div>
    </div>

    <InputBox :is-used="isUsed" @add-card="addCard($event)" />
  </div>
</template>

<script>
import { defineComponent } from "vue";
import Card from "./Card.vue";
import InputBox from "./InputBox.vue";

let worker;

export default defineComponent({
  name: "Main",
  components: { Card, InputBox },

  data: function () {
    return {
      inputPosition: 0,
      animationIndices: [],
      isUsed: Array.from({ length: 52 }, () => false),
      usedCards: Array.from({ length: 9 }, () => -1),
      yourWinRate: 0.0,
      oppWinRate: 0.0,
      running: false,
    };
  },

  watch: {
    usedCards: {
      handler: function () {
        if (this.isValidInput()) {
          if (this.running) worker.terminate();
          this.running = true;
          worker = new Worker(new URL("../worker", import.meta.url));
          worker.onmessage = (ev) => {
            worker.terminate();
            this.running = false;
            this.yourWinRate = ev.data[0];
            this.oppWinRate = ev.data[1];
          };
          worker.postMessage(Int32Array.from(this.usedCards));
        } else {
          this.yourWinRate = 0.0;
          this.oppWinRate = 0.0;
        }
      },
      deep: true,
    },
  },

  methods: {
    addCard: function (card) {
      if (!this.isUsed[card]) {
        if (this.usedCards[this.inputPosition] != -1) {
          this.isUsed[this.usedCards[this.inputPosition]] = false;
        }
        this.isUsed[card] = true;
        this.usedCards[this.inputPosition] = card;
        if (!this.animationIndices.includes(this.inputPosition)) {
          this.animationIndices.push(this.inputPosition);
          setTimeout(() => this.animationIndices.shift(), 350);
        }
        this.inputPosition = Math.min(this.inputPosition + 1, 8);
      }
    },

    isValidInput: function () {
      for (let i = 0; i < 4; ++i) {
        if (this.usedCards[i] === -1) {
          return false;
        }
      }
      let com_first_empty = this.usedCards.slice(4, 9).indexOf(-1);
      if (com_first_empty === -1 || com_first_empty === 4) {
        return true;
      } else if (com_first_empty === 3) {
        return this.usedCards[8] === -1;
      } else if (com_first_empty === 0) {
        return this.usedCards.slice(5, 9).every((val) => val === -1);
      } else {
        return false;
      }
    },

    removeCard: function (pos) {
      this.isUsed[this.usedCards[pos]] = false;
      this.usedCards[pos] = -1;
    },

    setInputPosition: function (pos) {
      let com_first_empty = this.usedCards.slice(4, 9).indexOf(-1);
      if (pos === 1 && this.usedCards[0] === -1 && this.usedCards[1] === -1) {
        pos = 0;
      }
      if (pos === 3 && this.usedCards[2] === -1 && this.usedCards[3] === -1) {
        pos = 2;
      }
      if (4 <= pos && com_first_empty !== -1 && this.usedCards[pos] === -1) {
        pos = Math.min(pos, com_first_empty + 4);
      }
      this.inputPosition = pos;
    },
  },
});
</script>

<style scoped>
.add-card-animation {
  animation: bounce-in 0.3s;
}

@keyframes bounce-in {
  0% {
    transform: scale(1);
  }
  40% {
    transform: scale(1.15);
  }
  90% {
    transform: scale(0.95);
  }
  100% {
    transform: scale(1);
  }
}

.win-rate-animation {
  transition: all 0.2s ease-out;
}
</style>
