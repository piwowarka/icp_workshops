<template>
  <div>
    <Icp />
    <Calculator />
  </div>
</template>

<script>
import Icp from "./components/Icp.vue";
import Calculator from "./components/Calculator.vue";

export default {
  name: "App",
  components: {
    Icp,
    Calculator,
  },
};
</script>