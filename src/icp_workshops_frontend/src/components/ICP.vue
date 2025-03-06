<script setup>
import { ref, onMounted } from "vue";
import { icp_workshops_backend } from "declarations/icp_workshops_backend/index";

let a = ref(null);
let b = ref(null);
let operator = ref("+");
let result = ref("");

async function handleCalculate(e) {
  e.preventDefault();
  if (a.value === null || b.value === null) {
    result.value = "Please enter both numbers";
    return;
  }

  try {
    const response = await icp_workshops_backend.calculate(a.value, b.value, operator.value);
    result.value = response;
    console.log(response);
    localStorage.setItem("result", response);
    console.log(localStorage.getItem("result"));
  } catch (error) {
    result.value = error;
  }
}

onMounted(() => {
  const storedResult = localStorage.getItem("result");
  if (storedResult) {
    result.value = storedResult;
  }
});
</script>

<template>
  <form action="#" @submit="handleCalculate">
    <label for="a">Enter A:</label>
    <input id="a" v-model.number="a" type="number" placeholder="Number A" />
    <label for="b">Enter B:</label>
    <input id="b" v-model.number="b" type="number" placeholder="Number B" />
    <label for="operator">Choose an operator:</label>
    <select id="operator" v-model="operator">
      <option value="+">+</option>
      <option value="-">-</option>
      <option value="*">*</option>
      <option value="/">/</option>
      <option value="%">%</option>
    </select>
    <button type="submit">Calculate</button>
    <p>{{ result }}</p>
  </form>
</template>