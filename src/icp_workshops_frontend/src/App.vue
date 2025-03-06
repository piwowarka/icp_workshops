<template>
  <div>
    <Icp />
    <Calculator />
  </div>
</template>

<script>
import Icp from "./components/ICP.vue";
import Calculator from "./components/calculator.vue";

export default {
  name: "App",
  components: {
    Icp,
    Calculator,
  },
};
</script>