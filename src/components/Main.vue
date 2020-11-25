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
    <div class="md:container flex items-center justify-center mx-auto mt-6">
      <div class="w-1/6 font-semibold">
        <span class="block text-center text-xl">You:</span>
        <span class="block text-center text-2xl">
          {{ (100.0 * yourWinRate).toFixed(2) + "%" }}
        </span>
      </div>
      <div class="flex w-1/2 h-4 rounded-2xl justify-between bg-gray-300">
        <div
          class="bg-green-600 rounded-l-2xl win-rate-animation"
          :class="{ 'rounded-r-2xl': yourWinRate > 0.99 }"
          :style="{ width: 100 * yourWinRate + '%' }"
        />
        <div
          class="bg-red-600 rounded-r-2xl win-rate-animation"
          :class="{ 'rounded-l-2xl': oppWinRate > 0.99 }"
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

    <!-- Infomation -->
    <div class="md:container flex items-center justify-center mx-auto mb-6">
      <div class="flex items-center">
        <span class="spinner inline-block mr-2" :class="{ hidden: !running }" />
        <span>{{ message }}</span>
      </div>
    </div>

    <InputBox :is-used="isUsed" @add-card="addCard($event)" @clear="clear()" />

    <div class="md:container flex justify-center mx-auto my-6">
      <div class="inline-block mr-10">
        <table class="table-auto mb-8 mx-auto">
          <tr v-for="i in Array(13).keys()" :key="`hands-tr-${i}`">
            <td v-for="j in Array(13).keys()" :key="`hands-td-${i * 13 + j}`">
              <Hand
                :key="`hand-${i * 13 + j}`"
                class="px-1 py-1"
                :hand-id="i * 13 + j"
                :selected="selectedHands[i * 13 + j]"
                @click="selectedHands[i * 13 + j] = !selectedHands[i * 13 + j]"
              />
            </td>
          </tr>
        </table>
        <div class="text-2xl font-semibold mx-auto text-center">
          {{ (ratio * 100).toFixed(2) }}%
        </div>
      </div>
      <div class="inline-block">
        <span class="block text-2xl font-semibold pb-1">Made hands</span>
        <table class="table-auto mt-6 mb-8 mx-auto">
          <tr v-for="kind in MADE_HAND_KINDS" :key="`made-hand-tr-${kind}`">
            <td>{{ kind }}</td>
            <td class="pl-6 text-right">
              {{
                Number.isNaN(madeHandProb[kind])
                  ? "-"
                  : (madeHandProb[kind] * 100).toFixed(2) + "%"
              }}
            </td>
          </tr>
        </table>
      </div>
      <div class="inline-block mx-6">
        <span class="block text-2xl font-semibold pb-1">Draws</span>
        <table class="table-auto mt-6 mb-8 mx-auto">
          <tr v-for="kind in DRAW_KINDS" :key="`draw-tr-${kind}`">
            <td>{{ kind }}</td>
            <td class="pl-6 text-right">
              {{
                Number.isNaN(drawProb[kind])
                  ? "-"
                  : (drawProb[kind] * 100).toFixed(2) + "%"
              }}
            </td>
          </tr>
        </table>
      </div>
      <div class="inline-block">
        <span class="block text-2xl font-semibold pb-1">Combinations</span>
        <table class="table-auto mt-6 mb-8 mx-auto">
          <tr
            v-for="kind in MADE_HAND_DRAW_COMBO_KINDS"
            :key="`made-hand-draw-combo-tr-${kind}`"
          >
            <td>{{ kind }}</td>
            <td class="pl-6 text-right">
              {{
                Number.isNaN(madeHandDrawComboProb[kind])
                  ? "-"
                  : (madeHandDrawComboProb[kind] * 100).toFixed(2) + "%"
              }}
            </td>
          </tr>
        </table>
      </div>
    </div>
  </div>
</template>

<script>
import { defineComponent } from "vue";
import Card from "./Card.vue";
import InputBox from "./InputBox.vue";
import Hand from "./Hand.vue";

let worker;
let rangeAnalyzer;

function runRangeAnalyzer() {
  if (this.analyzerRunning) {
    rangeAnalyzer.terminate();
    this.analyzerRunning = false;
  }

  const supported =
    [2, 3, 7, 8].every((v) => this.usedCards[v] === -1) &&
    [0, 1, 4, 5, 6].every((v) => this.usedCards[v] !== -1);
  if (!supported) {
    for (const key in this.madeHandProb) {
      this.madeHandProb[key] = NaN;
    }
    return;
  }

  this.analyzerRunning = true;
  rangeAnalyzer = new Worker(new URL("../range-analyzer", import.meta.url));
  rangeAnalyzer.onmessage = (ev) => {
    rangeAnalyzer.terminate();
    this.analyzerRunning = false;
    this.madeHandProb["straight flush"] = ev.data[0];
    this.madeHandProb["quads"] = ev.data[1];
    this.madeHandProb["full house"] = ev.data[2];
    this.madeHandProb["flush"] = ev.data[3];
    this.madeHandProb["straight"] = ev.data[4];
    this.madeHandProb["trips"] = ev.data[5];
    this.madeHandProb["two pair"] = ev.data[6];
    this.madeHandProb["pair"] = ev.data[7];
    this.madeHandProb["no made hand"] = ev.data[8];
  };
  const input = [];
  input.push(this.usedCards[0]);
  input.push(this.usedCards[1]);
  input.push(this.usedCards[4]);
  input.push(this.usedCards[5]);
  input.push(this.usedCards[6]);
  let selected = [];
  for (let i = 0; i < 13; ++i) {
    for (let j = 0; j < 13; ++j) {
      if (this.selectedHands[i * 13 + j]) {
        selected.push(i * 13 + j);
      }
    }
  }
  input.push(selected.length);
  Array.prototype.push.apply(input, selected);
  rangeAnalyzer.postMessage(Int32Array.from(input));
}

export default defineComponent({
  name: "Main",
  components: { Card, InputBox, Hand },

  data: function () {
    const MADE_HAND_KINDS = [
      "straight flush",
      "quads",
      "full house",
      "flush",
      "straight",
      "trips",
      "two pair",
      // "over pair",
      // "top pair",
      // "pocket pair (1/2)",
      // "middle pair",
      // "pocket pair (2/3)",
      // "bottom pair",
      // "under pair",
      "pair",
      // "ace high",
      "no made hand",
    ];
    const DRAW_KINDS = ["flush draw", "OESD", "gutshot", "backdoor flush"];
    const MADE_HAND_DRAW_COMBO_KINDS = [
      "flush draw + pair",
      "flush draw + OESD",
      "flush draw + gutshot",
      "OESD + pair",
      "gutshot + pair",
    ];
    return {
      inputPosition: 0,
      animationIndices: [],
      isUsed: Array.from({ length: 52 }, () => false),
      usedCards: Array.from({ length: 9 }, () => -1),
      usedCardsOld: Array.from({ length: 9 }, () => -1),
      yourWinRate: 0.0,
      oppWinRate: 0.0,
      running: false,
      selectedHands: Array.from({ length: 169 }, () => false),
      MADE_HAND_KINDS,
      DRAW_KINDS,
      MADE_HAND_DRAW_COMBO_KINDS,
      madeHandProb: Object.fromEntries(MADE_HAND_KINDS.map((v) => [v, NaN])),
      drawProb: Object.fromEntries(DRAW_KINDS.map((v) => [v, NaN])),
      madeHandDrawComboProb: Object.fromEntries(
        MADE_HAND_DRAW_COMBO_KINDS.map((v) => [v, NaN])
      ),
      analyzerRunning: false,
    };
  },

  computed: {
    currentStage: function () {
      let com_first_empty = this.usedCards.slice(4, 9).indexOf(-1);
      if (com_first_empty === -1) return "River";
      if (com_first_empty < 3) return "Pre-flop";
      if (com_first_empty === 3) return "Flop";
      return "Turn";
    },

    message: function () {
      if (this.running) {
        return "Computing...";
      }
      if (this.usedCards[0] === -1 || this.usedCards[1] === -1) {
        return "Please input your cards.";
      }
      return `[${this.currentStage}] Tie: ${Math.max(
        100.0 * (1.0 - this.yourWinRate - this.oppWinRate),
        0.0
      ).toFixed(2)}%`;
    },

    ratio: function () {
      let count = 0;
      for (let i = 0; i < 13; ++i) {
        for (let j = 0; j < 13; ++j) {
          const id = i * 13 + j;
          if (this.selectedHands[id]) {
            if (i === j) {
              count += 6;
            } else if (i < j) {
              count += 4;
            } else {
              count += 12;
            }
          }
        }
      }
      return count / 1326;
    },
  },

  watch: {
    usedCards: {
      handler: function () {
        runRangeAnalyzer.bind(this)();

        let new_input = this.sanitizeInput(this.usedCards);
        let old_input = this.sanitizeInput(this.usedCardsOld);
        this.usedCardsOld = this.usedCards.slice();

        if (new_input.length === 0) {
          if (this.running) {
            worker.terminate();
            this.running = false;
          }
          this.yourWinRate = 0.0;
          this.oppWinRate = 0.0;
          return;
        }

        let unchanged =
          new_input.length === old_input.length &&
          new_input.every((val, idx) => val === old_input[idx]);
        console.log(unchanged);

        if (unchanged) return;

        if (this.running) worker.terminate();
        this.running = true;
        worker = new Worker(new URL("../worker", import.meta.url));
        worker.onmessage = (ev) => {
          worker.terminate();
          this.running = false;
          this.yourWinRate = ev.data[0];
          this.oppWinRate = ev.data[1];
        };
        worker.postMessage(new_input);
      },
      deep: true,
    },
    selectedHands: {
      handler: function () {
        runRangeAnalyzer.bind(this)();
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

    clear: function () {
      this.isUsed = Array.from({ length: 52 }, () => false);
      this.usedCards = Array.from({ length: 9 }, () => -1);
      this.inputPosition = 0;
    },

    sanitizeInput: function (input) {
      if (input[0] === -1 || input[1] === -1) {
        return new Int32Array();
      }
      let result = Int32Array.from(input);
      let com_first_empty = input.slice(4, 9).indexOf(-1);
      if (com_first_empty === -1) return result; // river
      if (com_first_empty < 3) result.fill(-1, 4); // pre-flop
      if (com_first_empty === 3) result[8] = -1; // flop
      return result;
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

.spinner {
  text-indent: -9999em;
  width: 1em;
  height: 1em;
  border-radius: 50%;
  background: #404040;
  background: -moz-linear-gradient(left, #404040 10%, rgba(64, 64, 64, 0) 42%);
  background: -webkit-linear-gradient(
    left,
    #404040 10%,
    rgba(64, 64, 64, 0) 42%
  );
  background: -o-linear-gradient(left, #404040 10%, rgba(64, 64, 64, 0) 42%);
  background: -ms-linear-gradient(left, #404040 10%, rgba(64, 64, 64, 0) 42%);
  background: linear-gradient(to right, #404040 10%, rgba(64, 64, 64, 0) 42%);
  position: relative;
  -webkit-animation: spinner-animation 0.8s infinite linear;
  animation: spinner-animation 0.8s infinite linear;
  -webkit-transform: translateZ(0);
  -ms-transform: translateZ(0);
  transform: translateZ(0);
}

.spinner:before {
  width: 50%;
  height: 50%;
  background: #404040;
  border-radius: 100% 0 0 0;
  position: absolute;
  top: 0;
  left: 0;
  content: "";
}

.spinner:after {
  @apply bg-gray-100;
  width: 75%;
  height: 75%;
  border-radius: 50%;
  content: "";
  margin: auto;
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  right: 0;
}

@-webkit-keyframes spinner-animation {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}

@keyframes spinner-animation {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}
</style>
